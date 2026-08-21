//! Chromophore molar-extinction spectra and the absorption they imply.
//!
//! Hyperion owns optical coefficients, so the reference spectra that produce
//! them belong here rather than in an integrator's leaf crate (ADR 0001,
//! chromophore revision).
//!
//! Spectra are tabulated, not analytic: each is a `&'static` table of
//! (wavelength, molar extinction) samples, read without allocating, which is
//! what lets this compile under `no_std`.

use aequitas::systems::si::quantities::ReciprocalLength;
use eunomia::{RealField, UnitScalar};

use super::{Absorption, InteractionCoefficient};
use crate::{TransportError, ValueKind, validation};

/// Oxyhemoglobin (`HbO₂`) molar extinction, M⁻¹·cm⁻¹ per hemoglobin molecule.
///
/// Scott Prahl's 1999 OMLC compilation presents these values in the
/// preformatted source table's `lambda`/`Hb02` columns using the 64,500 g/mol
/// molecular mass of hemoglobin ([source table][prahl]). A hemoglobin
/// molecule is the tetramer, so tetramer-molar concentrations pair with these
/// values directly; no additional per-heme-to-tetramer factor is applied.
/// The page was retrieved on 2026-08-20.
///
/// [prahl]: https://omlc.org/spectra/hemoglobin/summary.html
const OXYHEMOGLOBIN_SAMPLES: &[(u16, f64)] = &[
    (450, 62_816.0),
    (474, 30_113.6),
    (500, 20_932.8),
    (524, 29_269.2),
    (532, 43_876.0),
    (550, 43_016.0),
    (574, 53_308.0),
    (600, 3_200.0),
    (624, 774.0),
    (650, 368.0),
    (674, 285.6),
    (700, 290.0),
    (724, 364.0),
    (750, 518.0),
    (774, 677.2),
    (800, 816.0),
    (824, 944.8),
    (850, 1_058.0),
    (874, 1_137.6),
    (900, 1_198.0),
    (924, 1_227.2),
    (950, 1_204.0),
    (974, 1_150.8),
    (1000, 1_024.0),
];

/// Deoxyhemoglobin (Hb) molar extinction, M⁻¹·cm⁻¹ per hemoglobin molecule.
/// These values are transcribed from the source table's `lambda`/`Hb` columns;
/// the source and molecular/tetramer normalization are those of
/// [`OXYHEMOGLOBIN_SAMPLES`].
const DEOXYHEMOGLOBIN_SAMPLES: &[(u16, f64)] = &[
    (450, 103_292.0),
    (474, 15_048.4),
    (500, 20_862.0),
    (524, 34_397.6),
    (532, 40_584.0),
    (550, 53_412.0),
    (574, 41_716.0),
    (600, 14_677.2),
    (624, 5_906.8),
    (650, 3_750.12),
    (674, 2_627.64),
    (700, 1_794.28),
    (724, 1_244.44),
    (750, 1_405.24),
    (774, 1_213.0),
    (800, 761.72),
    (824, 693.48),
    (850, 691.32),
    (874, 714.08),
    (900, 761.84),
    (924, 776.64),
    (950, 602.24),
    (974, 402.28),
    (1000, 206.784),
];

/// A tabulated molar-extinction spectrum, sampled in ascending wavelength.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtinctionSpectrum {
    samples: &'static [(u16, f64)],
}

/// Oxyhemoglobin spectrum.
pub const OXYHEMOGLOBIN: ExtinctionSpectrum = ExtinctionSpectrum {
    samples: OXYHEMOGLOBIN_SAMPLES,
};

/// Deoxyhemoglobin spectrum.
pub const DEOXYHEMOGLOBIN: ExtinctionSpectrum = ExtinctionSpectrum {
    samples: DEOXYHEMOGLOBIN_SAMPLES,
};

impl ExtinctionSpectrum {
    /// Inclusive wavelength bounds of the table, in nanometres.
    #[must_use]
    pub const fn wavelength_bounds_nm(&self) -> (u16, u16) {
        // Both tables are non-empty consts; indexing is in bounds by
        // construction, and the samples are ascending.
        (self.samples[0].0, self.samples[self.samples.len() - 1].0)
    }

    /// Molar extinction at `wavelength_nm`, linearly interpolated between the
    /// two bracketing samples, in M⁻¹·cm⁻¹.
    ///
    /// Interpolation is done in continuous wavelength. Quantising the query to
    /// the nearest tabulated nanometre first — as the integrator-side
    /// implementation did — discards sub-nanometre resolution, which matters
    /// near the steep Soret band below 600 nm where neighbouring samples
    /// differ by more than an order of magnitude.
    ///
    /// # Errors
    ///
    /// [`TransportError::WavelengthOutOfRange`] when `wavelength_nm` falls
    /// outside the table. The bounds are the measured range; extrapolating
    /// past them would return a number the source data does not support, so
    /// this rejects instead of clamping.
    pub fn molar_extinction<T: RealField>(&self, wavelength_nm: T) -> Result<T, TransportError<T>> {
        let (low, high) = self.wavelength_bounds_nm();
        let minimum = T::from_f64(f64::from(low));
        let maximum = T::from_f64(f64::from(high));
        if !(wavelength_nm >= minimum && wavelength_nm <= maximum) {
            return Err(TransportError::WavelengthOutOfRange {
                value: wavelength_nm,
                minimum,
                maximum,
            });
        }

        let mut previous = self.samples[0];
        for &(sample_nm, epsilon) in self.samples {
            let sample = T::from_f64(f64::from(sample_nm));
            if sample == wavelength_nm {
                return Ok(T::from_f64(epsilon));
            }
            if sample > wavelength_nm {
                let (lower_nm, lower_eps) = previous;
                let lower = T::from_f64(f64::from(lower_nm));
                let span = sample - lower;
                let t = (wavelength_nm - lower) / span;
                let lower_eps = T::from_f64(lower_eps);
                return Ok(lower_eps + t * (T::from_f64(epsilon) - lower_eps));
            }
            previous = (sample_nm, epsilon);
        }
        // The bounds check above guarantees a bracketing sample was found.
        Ok(T::from_f64(self.samples[self.samples.len() - 1].1))
    }
}

/// Absorption coefficient of hemoglobin at `wavelength_nm`, from tetramer-molar
/// oxy and deoxy concentrations in mol/L.
///
/// Beer-Lambert, with the centimetre-based tabulation converted to SI:
///
/// ```text
/// mu_a [m^-1] = ln(10) * (eps_HbO2 * c_HbO2 + eps_Hb * c_Hb) [M^-1 cm^-1 * mol/L] * 100 [cm/m]
/// ```
///
/// # Errors
///
/// [`TransportError::WavelengthOutOfRange`] when the wavelength is outside the
/// tabulated range, and [`TransportError::InvalidValue`] when the resulting
/// coefficient is negative or non-finite — which a negative concentration
/// produces.
pub fn hemoglobin_absorption<T: RealField + UnitScalar>(
    wavelength_nm: T,
    oxy_molar: T,
    deoxy_molar: T,
) -> Result<InteractionCoefficient<T, Absorption>, TransportError<T>> {
    let oxy_molar =
        validation::finite_non_negative(ValueKind::ChromophoreConcentration, oxy_molar)?;
    let deoxy_molar =
        validation::finite_non_negative(ValueKind::ChromophoreConcentration, deoxy_molar)?;
    let oxy_epsilon = OXYHEMOGLOBIN.molar_extinction(wavelength_nm)?;
    let deoxy_epsilon = DEOXYHEMOGLOBIN.molar_extinction(wavelength_nm)?;
    let per_centimetre = oxy_epsilon * oxy_molar + deoxy_epsilon * deoxy_molar;
    let ln10 = T::from_f64(core::f64::consts::LN_10);
    let centimetres_per_metre = T::from_f64(100.0);
    let per_metre = ln10 * per_centimetre * centimetres_per_metre;
    InteractionCoefficient::new(ReciprocalLength::from_base(per_metre))
}

#[cfg(test)]
mod tests;
