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
