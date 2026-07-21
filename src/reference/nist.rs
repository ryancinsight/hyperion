use aequitas::systems::si::{
    quantities::{AreaPerMass, Energy},
    units::{MegaElectronVolt, SquareCentimeterPerGram},
};
use eunomia::RealField;

use super::nist_data::{
    CORTICAL_BONE_MASS_ATTENUATION, DRY_AIR_MASS_ATTENUATION, KNOT_COUNT,
    LIQUID_WATER_MASS_ATTENUATION, MAXIMUM_ENERGY_MEV, MINIMUM_ENERGY_MEV, PHOTON_ENERGY_MEV,
};
use crate::{
    TransportError, TransportLaw, coefficient::MassAttenuation, quantity::PhotonEnergy, validation,
};

/// Bounded NIST photon mass-attenuation reference table.
///
/// The embedded values are the `mu/rho` column from NIST's
/// [X-Ray Mass Attenuation Coefficients](https://physics.nist.gov/PhysRefData/XrayMassCoef/)
/// tables over the shared 0.01–20 `MeV` range. The selected knots do not cross a
/// represented absorption edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NistMassAttenuationTable {
    /// Dry air near sea level.
    DryAir,
    /// Liquid water.
    LiquidWater,
    /// Cortical bone from ICRU-44.
    CorticalBone,
}

impl NistMassAttenuationTable {
    /// Return the mass attenuation coefficient at `energy`.
    ///
    /// Exact knots bypass interpolation and convert the stored NIST value
    /// through Aequitas. Between adjacent knots this evaluates log-linear
    /// interpolation in native `T` arithmetic.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::PhotonEnergyOutOfRange`] outside the inclusive
    /// 0.01–20 `MeV` interval and [`TransportError::DerivedNonFinite`] if
    /// interpolation produces a non-finite coefficient.
    pub fn at<T: RealField>(
        self,
        energy: PhotonEnergy<T>,
    ) -> Result<MassAttenuation<T>, TransportError<T>> {
        let energy_mev = energy.in_unit::<MegaElectronVolt>();
        let energy_base = energy.into_quantity().into_base();
        let minimum = T::from_f64(MINIMUM_ENERGY_MEV);
        let maximum = T::from_f64(MAXIMUM_ENERGY_MEV);
        let minimum_base: T = knot_energy_base(MINIMUM_ENERGY_MEV);
        let maximum_base: T = knot_energy_base(MAXIMUM_ENERGY_MEV);
        if energy_base < minimum_base || energy_base > maximum_base {
            return Err(TransportError::PhotonEnergyOutOfRange {
                value: energy_mev,
                minimum,
                maximum,
            });
        }

        let coefficients = self.coefficients();
        let upper = upper_knot(energy_base);
        let value = if energy_base == knot_energy_base(PHOTON_ENERGY_MEV[upper]) {
            T::from_f64(coefficients[upper])
        } else {
            interpolate(energy_base, upper, coefficients)
        };
        let finite = validation::derived_finite(TransportLaw::NistInterpolation, value)?;
        MassAttenuation::new(AreaPerMass::from_unit::<SquareCentimeterPerGram>(finite))
    }

    const fn coefficients(self) -> &'static [f64; KNOT_COUNT] {
        match self {
            Self::DryAir => &DRY_AIR_MASS_ATTENUATION,
            Self::LiquidWater => &LIQUID_WATER_MASS_ATTENUATION,
            Self::CorticalBone => &CORTICAL_BONE_MASS_ATTENUATION,
        }
    }
}

fn upper_knot<T: RealField>(energy_base: T) -> usize {
    let mut upper = 0;
    while upper + 1 < KNOT_COUNT && energy_base > knot_energy_base(PHOTON_ENERGY_MEV[upper]) {
        upper += 1;
    }
    upper
}

fn interpolate<T: RealField>(energy_base: T, upper: usize, coefficients: &[f64; KNOT_COUNT]) -> T {
    let lower = upper - 1;
    let lower_energy: T = knot_energy_base(PHOTON_ENERGY_MEV[lower]);
    let upper_energy: T = knot_energy_base(PHOTON_ENERGY_MEV[upper]);
    let lower_coefficient = T::from_f64(coefficients[lower]);
    let upper_coefficient = T::from_f64(coefficients[upper]);
    let log_fraction =
        (energy_base.ln() - lower_energy.ln()) * (upper_energy.ln() - lower_energy.ln()).recip();
    let log_coefficient =
        lower_coefficient.ln() + (upper_coefficient.ln() - lower_coefficient.ln()) * log_fraction;
    log_coefficient.exp()
}

fn knot_energy_base<T: RealField>(energy_mev: f64) -> T {
    Energy::from_unit::<MegaElectronVolt>(T::from_f64(energy_mev)).into_base()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_every_knot_bypasses_interpolation<T: RealField>(table: NistMassAttenuationTable) {
        for (&energy_mev, &coefficient) in PHOTON_ENERGY_MEV.iter().zip(table.coefficients()) {
            let energy = PhotonEnergy::new(Energy::from_unit::<MegaElectronVolt>(T::from_f64(
                energy_mev,
            )))
            .expect("NIST knot energies are finite and positive");
            let actual = table
                .at(energy)
                .expect("NIST knot lies inside its own table")
                .into_quantity()
                .into_base();
            let expected =
                AreaPerMass::from_unit::<SquareCentimeterPerGram>(T::from_f64(coefficient))
                    .into_base();
            assert_eq!(actual, expected, "energy={energy_mev} MeV");
        }
    }

    #[test]
    fn every_embedded_knot_is_exact_for_every_supported_real_scalar() {
        for table in [
            NistMassAttenuationTable::DryAir,
            NistMassAttenuationTable::LiquidWater,
            NistMassAttenuationTable::CorticalBone,
        ] {
            assert_every_knot_bypasses_interpolation::<f32>(table);
            assert_every_knot_bypasses_interpolation::<f64>(table);
        }
    }
}
