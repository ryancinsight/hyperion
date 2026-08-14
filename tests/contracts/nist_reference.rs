//! NIST reference-table and interpolation oracles.

use crate::support::photon_energy;
use aequitas::systems::si::{quantities::AreaPerMass, units::SquareCentimeterPerGram};
use eunomia::{RealField, UnitScalar};
use hyperion::{TransportError, reference::NistMassAttenuationTable};

fn assert_reference_knots<T: RealField + UnitScalar>() {
    for (table, expected) in [
        (NistMassAttenuationTable::DryAir, 0.06358),
        (NistMassAttenuationTable::LiquidWater, 0.07072),
        (NistMassAttenuationTable::CorticalBone, 0.06566),
    ] {
        let actual = table
            .at(photon_energy::<T>(1.0))
            .expect("one MeV is an exact embedded knot")
            .into_quantity()
            .into_base();
        let stored =
            AreaPerMass::from_unit::<SquareCentimeterPerGram>(T::from_f64(expected)).into_base();
        assert_eq!(actual, stored);
    }

    let first = NistMassAttenuationTable::LiquidWater
        .at(photon_energy::<T>(0.01))
        .expect("lower endpoint is inclusive")
        .into_quantity()
        .into_base();
    let last = NistMassAttenuationTable::LiquidWater
        .at(photon_energy::<T>(20.0))
        .expect("upper endpoint is inclusive")
        .into_quantity()
        .into_base();
    assert_eq!(
        first,
        AreaPerMass::from_unit::<SquareCentimeterPerGram>(T::from_f64(5.329)).into_base()
    );
    assert_eq!(
        last,
        AreaPerMass::from_unit::<SquareCentimeterPerGram>(T::from_f64(0.01813)).into_base()
    );
}

#[test]
fn official_knots_are_exact_in_every_supported_real_scalar() {
    assert_reference_knots::<f32>();
    assert_reference_knots::<f64>();
}

/// Off-knot outputs obtained from NIST XCOM 1.5 on 2026-08-14 via
/// <https://physics.nist.gov/cgi-bin/Xcom/xcom3_3> for liquid water (`H2O`).
///
/// XCOM reports four significant digits; its version history explicitly says
/// that the fourth digit aids interpolation and is not an accuracy claim. The
/// fixture therefore tests an independent method trend, not an invented
/// absolute tolerance. Each tuple is `(energy, lower knot, lower coefficient,
/// upper knot, upper coefficient, XCOM total-with-coherent coefficient)`.
const XCOM_WATER_OFF_KNOTS: [(f64, f64, f64, f64, f64, f64); 10] = [
    (0.125, 0.1, 0.1707, 0.15, 0.1505, 0.1593),
    (0.175, 0.15, 0.1505, 0.2, 0.1370, 0.1432),
    (0.35, 0.3, 0.1186, 0.4, 0.1061, 0.1119),
    (0.7, 0.6, 0.08956, 0.8, 0.07865, 0.08362),
    (1.125, 1.0, 0.07072, 1.25, 0.06323, 0.06671),
    (1.75, 1.5, 0.05754, 2.0, 0.04942, 0.05310),
    (2.5, 2.0, 0.04942, 3.0, 0.03969, 0.04376),
    (3.5, 3.0, 0.03969, 4.0, 0.03403, 0.03654),
    (7.0, 6.0, 0.02770, 8.0, 0.02429, 0.02577),
    (12.5, 10.0, 0.02219, 15.0, 0.01941, 0.02051),
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
        + (T::from_f64(upper_coefficient).ln() - T::from_f64(lower_coefficient).ln()) * fraction)
        .exp()
}

fn assert_independent_xcom_trend<T: RealField + UnitScalar>() {
    let mut spline_maximum = T::from_f64(0.0);
    let mut log_linear_maximum = T::from_f64(0.0);
    for &(energy, lower_energy, lower_coefficient, upper_energy, upper_coefficient, reference) in
        &XCOM_WATER_OFF_KNOTS
    {
        let actual = NistMassAttenuationTable::LiquidWater
            .at(photon_energy::<T>(energy))
            .expect("XCOM fixture energy lies inside the table")
            .in_unit::<SquareCentimeterPerGram>();
        let expected = T::from_f64(reference);
        let linear = log_linear::<T>(
            energy,
            lower_energy,
            lower_coefficient,
            upper_energy,
            upper_coefficient,
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

#[test]
fn energies_outside_the_embedded_interval_report_bounds() {
    for value in [0.009_f64, 20.1] {
        assert_eq!(
            NistMassAttenuationTable::LiquidWater.at(photon_energy(value)),
            Err(TransportError::PhotonEnergyOutOfRange {
                value,
                minimum: 0.01,
                maximum: 20.0,
            })
        );
    }
}
