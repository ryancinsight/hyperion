//! Validity-boundary regressions for every public domain newtype.

use aequitas::systems::si::quantities::{
    AreaPerMass, Dimensionless, Energy, EnergyPerArea, Length, ReciprocalLength,
};
use eunomia::{NumericElement, RealField};
use hyperion::{
    TransportError, ValueConstraint, ValueKind,
    coefficient::{Absorption, InteractionCoefficient, MassAttenuation},
    quantity::{
        Anisotropy, EnergyFluence, OpticalDepth, PathLength, PhotonEnergy, Transmission,
        TransportAlbedo,
    },
    transport::OpticalDiffusionCoefficient,
};

fn assert_invalid_value<T: RealField>(
    result: Result<impl core::fmt::Debug, TransportError<T>>,
    field: ValueKind,
    value: T,
    constraint: ValueConstraint,
) {
    let error = result.expect_err("fixture must violate the declared validity boundary");
    assert_eq!(
        error,
        TransportError::InvalidValue {
            field,
            value,
            constraint,
        }
    );
}

fn assert_boundaries<T: RealField>() {
    let negative = -<T as NumericElement>::ONE;
    let zero = <T as NumericElement>::ZERO;
    let nan = T::nan();

    assert_invalid_value(
        InteractionCoefficient::<T, Absorption>::new(ReciprocalLength::from_base(negative)),
        ValueKind::AbsorptionCoefficient,
        negative,
        ValueConstraint::FiniteNonNegative,
    );
    assert_invalid_value(
        MassAttenuation::new(AreaPerMass::from_base(negative)),
        ValueKind::MassAttenuation,
        negative,
        ValueConstraint::FiniteNonNegative,
    );
    assert_invalid_value(
        PathLength::new(Length::from_base(negative)),
        ValueKind::PathLength,
        negative,
        ValueConstraint::FiniteNonNegative,
    );
    assert_invalid_value(
        PhotonEnergy::new(Energy::from_base(zero)),
        ValueKind::PhotonEnergy,
        zero,
        ValueConstraint::FinitePositive,
    );
    assert_invalid_value(
        EnergyFluence::new(EnergyPerArea::from_base(negative)),
        ValueKind::EnergyFluence,
        negative,
        ValueConstraint::FiniteNonNegative,
    );
    assert_invalid_value(
        OpticalDepth::new(Dimensionless::from_base(negative)),
        ValueKind::OpticalDepth,
        negative,
        ValueConstraint::FiniteNonNegative,
    );
    assert_invalid_value(
        OpticalDiffusionCoefficient::new(Length::from_base(zero)),
        ValueKind::OpticalDiffusionCoefficient,
        zero,
        ValueConstraint::FinitePositive,
    );

    for value in [T::from_f64(-1.0), T::from_f64(1.0)] {
        assert_eq!(
            Anisotropy::new(Dimensionless::from_base(value))
                .expect("anisotropy endpoints are physical")
                .into_quantity()
                .into_base(),
            value
        );
    }
    assert_invalid_value(
        Anisotropy::new(Dimensionless::from_base(T::from_f64(1.01))),
        ValueKind::Anisotropy,
        T::from_f64(1.01),
        ValueConstraint::ClosedMinusOneToOne,
    );

    for value in [zero, <T as NumericElement>::ONE] {
        assert_eq!(
            Transmission::new(Dimensionless::from_base(value))
                .expect("transmission endpoints are physical")
                .into_quantity()
                .into_base(),
            value
        );
        assert_eq!(
            TransportAlbedo::new(Dimensionless::from_base(value))
                .expect("transport-albedo endpoints are physical")
                .into_quantity()
                .into_base(),
            value
        );
    }
    assert_invalid_value(
        Transmission::new(Dimensionless::from_base(T::from_f64(1.01))),
        ValueKind::Transmission,
        T::from_f64(1.01),
        ValueConstraint::ClosedUnitInterval,
    );

    match PathLength::new(Length::from_base(nan)) {
        Err(TransportError::InvalidValue {
            field,
            value,
            constraint,
        }) => {
            assert_eq!(field, ValueKind::PathLength);
            assert!(value.is_nan(), "the rejected NaN must be preserved");
            assert_eq!(constraint, ValueConstraint::FiniteNonNegative);
        }
        result => panic!("expected a path-length NaN error, got {result:?}"),
    }
}

#[test]
fn validity_contracts_hold_for_every_supported_real_scalar() {
    assert_boundaries::<f32>();
    assert_boundaries::<f64>();
}
