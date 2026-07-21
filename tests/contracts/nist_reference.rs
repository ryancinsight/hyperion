//! NIST reference-table and interpolation oracles.

use crate::support::{assert_relative_close, photon_energy};
use aequitas::systems::si::{quantities::AreaPerMass, units::SquareCentimeterPerGram};
use eunomia::RealField;
use hyperion::{TransportError, reference::NistMassAttenuationTable};

fn assert_reference_knots<T: RealField>() {
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

fn assert_log_interpolation<T: RealField>() {
    let geometric_midpoint = (0.1_f64 * 0.15_f64).sqrt();
    let expected = (T::from_f64(0.1707) * T::from_f64(0.1505)).sqrt();
    let actual = NistMassAttenuationTable::LiquidWater
        .at(photon_energy::<T>(geometric_midpoint))
        .expect("geometric midpoint lies inside the table")
        .in_unit::<SquareCentimeterPerGram>();

    // At a geometric energy midpoint, log-linear interpolation equals the
    // geometric coefficient midpoint. The locked libm f32 log approximation
    // is below 2^-34.24, exp below 2^-27.74, and each path contains fewer than
    // 96 rounded operations; f64 has stricter one-ulp transcendental bounds.
    assert_relative_close(actual, expected, 96.0);
}

#[test]
fn interpolation_is_log_linear_in_every_supported_real_scalar() {
    assert_log_interpolation::<f32>();
    assert_log_interpolation::<f64>();
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
