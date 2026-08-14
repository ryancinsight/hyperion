use aequitas::systems::si::{
    quantities::{AreaPerMass, Energy},
    units::{MegaElectronVolt, SquareCentimeterPerGram},
};
use eunomia::{FloatElement, NumericElement, RealField, UnitScalar};

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
///
/// Intervals use the log-log natural cubic-spline form described by the NIST
/// XCOM method. The published four-significant-digit output is an
/// interpolation aid, not an accuracy guarantee; the sparse embedded table
/// therefore makes no global error claim between knots. Natural endpoint
/// conditions are the explicit local boundary choice because the embedded
/// table does not publish endpoint slopes. Independent XCOM checks belong in
/// the contract suite rather than being converted into a fabricated runtime
/// tolerance.
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
    /// through Aequitas. Between adjacent knots this evaluates a natural
    /// cubic spline in log-energy/log-coefficient space using native `T`
    /// arithmetic.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::PhotonEnergyOutOfRange`] outside the inclusive
    /// 0.01–20 `MeV` interval and [`TransportError::DerivedNonFinite`] if
    /// interpolation produces a non-finite coefficient.
    pub fn at<T: RealField + UnitScalar>(
        self,
        energy: PhotonEnergy<T>,
    ) -> Result<MassAttenuation<T>, TransportError<T>> {
        let energy_mev = energy.in_unit::<MegaElectronVolt>();
        let energy_base = energy.into_quantity().into_base();
        let minimum = <T as FloatElement>::from_f64(MINIMUM_ENERGY_MEV);
        let maximum = <T as FloatElement>::from_f64(MAXIMUM_ENERGY_MEV);
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
            <T as FloatElement>::from_f64(coefficients[upper])
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

fn upper_knot<T: RealField + UnitScalar>(energy_base: T) -> usize {
    let mut upper = 0;
    while upper + 1 < KNOT_COUNT && energy_base > knot_energy_base(PHOTON_ENERGY_MEV[upper]) {
        upper += 1;
    }
    upper
}

fn interpolate<T: RealField + UnitScalar>(
    energy_base: T,
    upper: usize,
    coefficients: &[f64; KNOT_COUNT],
) -> T {
    let lower = upper - 1;
    let lower_energy = knot_energy_base::<T>(PHOTON_ENERGY_MEV[lower]).ln();
    let upper_energy = knot_energy_base::<T>(PHOTON_ENERGY_MEV[upper]).ln();
    let energy = energy_base.ln();
    let span = upper_energy - lower_energy;
    let lower_weight = (upper_energy - energy) / span;
    let upper_weight = (energy - lower_energy) / span;
    let lower_coefficient = <T as FloatElement>::from_f64(coefficients[lower]);
    let upper_coefficient = <T as FloatElement>::from_f64(coefficients[upper]);
    let second_derivatives = spline_second_derivatives::<T>(coefficients);
    let six = <T as FloatElement>::from_f64(6.0);
    let curvature = ((lower_weight * lower_weight * lower_weight - lower_weight)
        * second_derivatives[lower]
        + (upper_weight * upper_weight * upper_weight - upper_weight) * second_derivatives[upper])
        * span
        * span
        / six;
    (lower_weight * lower_coefficient.ln() + upper_weight * upper_coefficient.ln() + curvature)
        .exp()
}

fn spline_second_derivatives<T: RealField + UnitScalar>(
    coefficients: &[f64; KNOT_COUNT],
) -> [T; KNOT_COUNT] {
    let mut second = [<T as NumericElement>::ZERO; KNOT_COUNT];
    let mut work = [<T as NumericElement>::ZERO; KNOT_COUNT];
    let two = <T as FloatElement>::from_f64(2.0);
    let six = <T as FloatElement>::from_f64(6.0);

    for index in 1..KNOT_COUNT - 1 {
        let previous_energy = knot_energy_base::<T>(PHOTON_ENERGY_MEV[index - 1]).ln();
        let energy = knot_energy_base::<T>(PHOTON_ENERGY_MEV[index]).ln();
        let next_energy = knot_energy_base::<T>(PHOTON_ENERGY_MEV[index + 1]).ln();
        let previous_coefficient = <T as FloatElement>::from_f64(coefficients[index - 1]).ln();
        let coefficient = <T as FloatElement>::from_f64(coefficients[index]).ln();
        let next_coefficient = <T as FloatElement>::from_f64(coefficients[index + 1]).ln();
        let left_span = energy - previous_energy;
        let right_span = next_energy - energy;
        let total_span = next_energy - previous_energy;
        let sigma = left_span / total_span;
        let pivot = sigma * second[index - 1] + two;
        second[index] = (sigma - <T as NumericElement>::ONE) / pivot;
        let left_slope = (coefficient - previous_coefficient) / left_span;
        let right_slope = (next_coefficient - coefficient) / right_span;
        work[index] =
            (six * (right_slope - left_slope) / total_span - sigma * work[index - 1]) / pivot;
    }

    for index in (0..KNOT_COUNT - 1).rev() {
        second[index] = second[index] * second[index + 1] + work[index];
    }
    second
}

fn knot_energy_base<T: RealField + UnitScalar>(energy_mev: f64) -> T {
    Energy::from_unit::<MegaElectronVolt>(<T as FloatElement>::from_f64(energy_mev)).into_base()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_every_knot_bypasses_interpolation<T: RealField + UnitScalar>(
        table: NistMassAttenuationTable,
    ) {
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
