use aequitas::systems::si::{
    quantities::{AreaPerMass, Energy},
    units::{MegaElectronVolt, SquareCentimeterPerGram},
};
use eunomia::{FloatElement, RealField, UnitScalar};

use super::nist_data::{
    CORTICAL_BONE_TABLE, DRY_AIR_TABLE, KNOT_COUNT, LIQUID_WATER_TABLE, MAXIMUM_ENERGY_MEV,
    MINIMUM_ENERGY_MEV, NistTable, PHOTON_ENERGY_MEV,
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
    /// arithmetic. The spline curvature, the knot log-energies, and the knot
    /// energies are stored in the table, so a query performs no tridiagonal
    /// solve and no knot-energy conversion beyond the bracketing search.
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
        let table = self.table();
        let minimum = <T as FloatElement>::from_f64(MINIMUM_ENERGY_MEV);
        let maximum = <T as FloatElement>::from_f64(MAXIMUM_ENERGY_MEV);
        let minimum_base: T = <T as FloatElement>::from_f64(table.knots[0]);
        let maximum_base: T = <T as FloatElement>::from_f64(table.knots[KNOT_COUNT - 1]);
        if energy_base < minimum_base || energy_base > maximum_base {
            return Err(TransportError::PhotonEnergyOutOfRange {
                value: energy_mev,
                minimum,
                maximum,
            });
        }

        let upper = upper_knot(energy_base);
        let value = if energy_base == knot_energy_base(PHOTON_ENERGY_MEV[upper]) {
            <T as FloatElement>::from_f64(table.coefficients[upper])
        } else {
            interpolate(energy_base, upper, table)
        };
        let finite = validation::derived_finite(TransportLaw::NistInterpolation, value)?;
        MassAttenuation::new(AreaPerMass::from_unit::<SquareCentimeterPerGram>(finite))
    }

    const fn table(self) -> &'static NistTable<KNOT_COUNT> {
        match self {
            Self::DryAir => &DRY_AIR_TABLE,
            Self::LiquidWater => &LIQUID_WATER_TABLE,
            Self::CorticalBone => &CORTICAL_BONE_TABLE,
        }
    }
}

/// Index of the first knot whose energy is at least `energy_base`.
///
/// Bounds checking guarantees `energy_base` lies inside the table, so this
/// binary search over the knot energies returns an index in `0..KNOT_COUNT`,
/// replacing the linear scan that converted a knot on every step. The probe
/// uses the same `T` conversion as the exact-knot bypass below, so the two
/// always agree on which knot lies at the query.
fn upper_knot<T: RealField + UnitScalar>(energy_base: T) -> usize {
    let mut low = 0;
    let mut high = KNOT_COUNT - 1;
    while low < high {
        let middle = low + (high - low) / 2;
        if knot_energy_base::<T>(PHOTON_ENERGY_MEV[middle]) < energy_base {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    low
}

/// Natural cubic-spline evaluation in log-energy/log-coefficient space.
///
/// The stored second derivatives remove the per-query solve; only the query
/// logarithm, two knot-coefficient logarithms, and the final exponentiation
/// remain transcendentals.
fn interpolate<T: RealField + UnitScalar>(
    energy_base: T,
    upper: usize,
    table: &NistTable<KNOT_COUNT>,
) -> T {
    let lower = upper - 1;
    let lower_energy = <T as FloatElement>::from_f64(table.ln_knots[lower]);
    let upper_energy = <T as FloatElement>::from_f64(table.ln_knots[upper]);
    let energy = energy_base.ln();
    let span = upper_energy - lower_energy;
    let lower_weight = (upper_energy - energy) / span;
    let upper_weight = (energy - lower_energy) / span;
    let lower_coefficient = <T as FloatElement>::from_f64(table.coefficients[lower]);
    let upper_coefficient = <T as FloatElement>::from_f64(table.coefficients[upper]);
    let lower_second = <T as FloatElement>::from_f64(table.second_derivatives[lower]);
    let upper_second = <T as FloatElement>::from_f64(table.second_derivatives[upper]);
    let six = <T as FloatElement>::from_f64(6.0);
    let curvature = ((lower_weight * lower_weight * lower_weight - lower_weight) * lower_second
        + (upper_weight * upper_weight * upper_weight - upper_weight) * upper_second)
        * span
        * span
        / six;
    (lower_weight * lower_coefficient.ln() + upper_weight * upper_coefficient.ln() + curvature)
        .exp()
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
        for (&energy_mev, &coefficient) in PHOTON_ENERGY_MEV.iter().zip(&table.table().coefficients)
        {
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

    /// Fresh natural cubic-spline solve over the stored `knots` and
    /// `coefficients`, in `f64` log space. Kept beside the stored table so a
    /// knot or coefficient edit that outruns the pasted second derivatives is
    /// caught instead of silently changing every off-knot value.
    fn recompute_second_derivatives(
        knots: &[f64; KNOT_COUNT],
        coefficients: &[f64; KNOT_COUNT],
    ) -> [f64; KNOT_COUNT] {
        let ln_knots: [f64; KNOT_COUNT] = knots.map(f64::ln);
        let ln_coefficients: [f64; KNOT_COUNT] = coefficients.map(f64::ln);
        let mut second = [0.0_f64; KNOT_COUNT];
        let mut work = [0.0_f64; KNOT_COUNT];
        for index in 1..KNOT_COUNT - 1 {
            let left_span = ln_knots[index] - ln_knots[index - 1];
            let right_span = ln_knots[index + 1] - ln_knots[index];
            let total_span = ln_knots[index + 1] - ln_knots[index - 1];
            let sigma = left_span / total_span;
            let pivot = sigma * second[index - 1] + 2.0;
            second[index] = (sigma - 1.0) / pivot;
            let left_slope = (ln_coefficients[index] - ln_coefficients[index - 1]) / left_span;
            let right_slope = (ln_coefficients[index + 1] - ln_coefficients[index]) / right_span;
            work[index] =
                (6.0 * (right_slope - left_slope) / total_span - sigma * work[index - 1]) / pivot;
        }
        for index in (0..KNOT_COUNT - 1).rev() {
            second[index] = second[index] * second[index + 1] + work[index];
        }
        second
    }

    #[test]
    fn stored_spline_data_matches_a_fresh_solve() {
        for table in [&DRY_AIR_TABLE, &LIQUID_WATER_TABLE, &CORTICAL_BONE_TABLE] {
            assert_eq!(
                table.ln_knots.map(f64::to_bits),
                table.knots.map(f64::ln).map(f64::to_bits),
                "stored log-knots must be the exact natural log of the stored knots"
            );
            assert_eq!(
                table.second_derivatives.map(f64::to_bits),
                recompute_second_derivatives(&table.knots, &table.coefficients).map(f64::to_bits),
                "stored spline curvature must match a fresh solve of the stored table"
            );
        }
    }

    /// Off-knot outputs obtained from NIST XCOM 1.5 on 2026-08-14 via
    /// <https://physics.nist.gov/cgi-bin/Xcom/xcom3_3> for liquid water (`H2O`).
    ///
    /// XCOM reports four significant digits; its version history explicitly says
    /// that the fourth digit aids interpolation and is not an accuracy claim.
    /// The fixture therefore tests an independent method trend, not an invented
    /// absolute tolerance. Each tuple is `(energy, XCOM total-with-coherent
    /// coefficient)`; the bracketing knots and coefficients are read from the
    /// shared table so a knot edit cannot leave a stale fixture bracket behind.
    const XCOM_WATER_OFF_KNOTS: [(f64, f64); 10] = [
        (0.125, 0.1593),
        (0.175, 0.1432),
        (0.35, 0.1119),
        (0.7, 0.08362),
        (1.125, 0.06671),
        (1.75, 0.05310),
        (2.5, 0.04376),
        (3.5, 0.03654),
        (7.0, 0.02577),
        (12.5, 0.02051),
    ];

    fn log_linear<T: RealField>(
        energy: f64,
        lower_energy: f64,
        lower_coefficient: f64,
        upper_energy: f64,
        upper_coefficient: f64,
    ) -> T {
        let energy = T::from_f64(energy).ln();
        let lower_energy = T::from_f64(lower_energy).ln();
        let upper_energy = T::from_f64(upper_energy).ln();
        let fraction = (energy - lower_energy) / (upper_energy - lower_energy);
        (T::from_f64(lower_coefficient).ln()
            + (T::from_f64(upper_coefficient).ln() - T::from_f64(lower_coefficient).ln())
                * fraction)
            .exp()
    }

    fn assert_independent_xcom_trend<T: RealField + UnitScalar>() {
        let mut spline_maximum = T::from_f64(0.0);
        let mut log_linear_maximum = T::from_f64(0.0);
        for &(energy, reference) in &XCOM_WATER_OFF_KNOTS {
            let upper = PHOTON_ENERGY_MEV.partition_point(|&knot| knot <= energy);
            let lower = upper - 1;
            let actual = NistMassAttenuationTable::LiquidWater
                .at(
                    PhotonEnergy::new(Energy::from_unit::<MegaElectronVolt>(T::from_f64(energy)))
                        .expect("XCOM fixture energy is finite and positive"),
                )
                .expect("XCOM fixture energy lies inside the table")
                .in_unit::<SquareCentimeterPerGram>();
            let expected = T::from_f64(reference);
            let linear = log_linear::<T>(
                energy,
                PHOTON_ENERGY_MEV[lower],
                LIQUID_WATER_TABLE.coefficients[lower],
                PHOTON_ENERGY_MEV[upper],
                LIQUID_WATER_TABLE.coefficients[upper],
            );
            let spline_error = (actual - expected).abs() / expected.abs();
            let linear_error = (linear - expected).abs() / expected.abs();
            if spline_error > spline_maximum {
                spline_maximum = spline_error;
            }
            if linear_error > log_linear_maximum {
                log_linear_maximum = linear_error;
            }
        }
        assert!(
            spline_maximum < log_linear_maximum,
            "XCOM trend did not improve: spline={spline_maximum:?}, log-linear={log_linear_maximum:?}"
        );
    }

    #[test]
    fn natural_log_spline_tracks_independent_xcom_trend() {
        assert_independent_xcom_trend::<f32>();
        assert_independent_xcom_trend::<f64>();
    }
}
