//! Chromophore molar-extinction spectra and the absorption they imply.
//!
//! Hyperion owns optical coefficients, so the reference spectra that produce
//! them belong here rather than in an integrator's leaf crate (ADR 0032 §4).
//!
//! Spectra are tabulated, not analytic: each is a `&'static` table of
//! (wavelength, molar extinction) samples, read without allocating, which is
//! what lets this compile under `no_std`.

use aequitas::systems::si::quantities::ReciprocalLength;
use eunomia::{RealField, UnitScalar};

use super::{Absorption, InteractionCoefficient};
use crate::TransportError;

/// Oxyhemoglobin (`HbO₂`) molar extinction, M⁻¹·cm⁻¹ per tetramer.
///
/// Prahl SA (1999), *Optical Absorption of Hemoglobin*, OMLC compiled
/// tabulation (Gratzer / Kollias); per-heme values scaled ×4 because a
/// tetramer carries four heme groups. Concentrations paired with these values
/// are therefore tetramer-molar.
const OXYHEMOGLOBIN_SAMPLES: &[(u16, f64)] = &[
    (450, 251_264.0),
    (475, 120_454.4),
    (500, 83_731.2),
    (525, 117_076.8),
    (532, 175_504.0),
    (550, 172_064.0),
    (575, 196_688.0),
    (600, 12_800.0),
    (625, 3_096.0),
    (650, 1_472.0),
    (675, 1_142.4),
    (700, 1_160.0),
    (725, 1_560.0),
    (750, 2_072.0),
    (775, 2_708.8),
    (800, 3_264.0),
    (825, 3_825.6),
    (850, 4_232.0),
    (875, 4_550.4),
    (900, 4_792.0),
    (925, 4_907.2),
    (950, 4_816.0),
    (975, 4_576.0),
    (1000, 4_096.0),
];

/// Deoxyhemoglobin (Hb) molar extinction, M⁻¹·cm⁻¹ per tetramer. Same source
/// and normalisation as [`OXYHEMOGLOBIN_SAMPLES`].
const DEOXYHEMOGLOBIN_SAMPLES: &[(u16, f64)] = &[
    (450, 413_168.0),
    (475, 60_193.6),
    (500, 83_448.0),
    (525, 137_590.4),
    (532, 162_336.0),
    (550, 213_648.0),
    (575, 173_360.0),
    (600, 58_708.8),
    (625, 23_627.2),
    (650, 15_000.48),
    (675, 10_510.56),
    (700, 7_177.12),
    (725, 4_408.8),
    (750, 5_620.96),
    (775, 4_852.0),
    (800, 3_046.88),
    (825, 2_773.28),
    (850, 2_765.28),
    (875, 2_856.32),
    (900, 3_047.36),
    (925, 3_089.44),
    (950, 2_408.96),
    (975, 1_557.152),
    (1000, 827.136),
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
