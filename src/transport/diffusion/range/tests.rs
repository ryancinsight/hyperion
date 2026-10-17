//! Encoding and public-policy contracts around the normal-radicand threshold.
use super::minimum_normal;
use crate::{
    TransportError, TransportLaw, ValueConstraint, ValueKind,
    coefficient::{Absorption, InteractionCoefficient, ReducedScattering},
    transport::DiffusionCoefficients,
};
use aequitas::systems::si::quantities::ReciprocalLength;
use eunomia::{F32, F64, RealField, UnitScalar};

fn threshold<T: RealField>(below: T, minimum: T, above: T) {
    let actual = minimum_normal::<T>();
    assert_eq!(actual, minimum);
    let exponent_bits = T::INFINITY.count_ones();
    assert_eq!(
        actual,
        T::ONE.scale_binary(2 - (1_i32 << (exponent_bits - 1)))
    );
    for (value, expected) in [
        (below, true),
        (minimum, false),
        (above, false),
        (T::ZERO, true),
        (-T::ZERO, true),
        (-T::ONE, true),
        (T::INFINITY, false),
        (-T::INFINITY, true),
        (T::NAN, false),
    ] {
        assert_eq!(
            value < actual,
            expected,
            "threshold comparison for {value:?}"
        );
    }
}
#[test]
fn minimum_normal_encoding_neighbors() {
    let lo = f32::from_bits(0x007f_ffff);
    let mid = f32::MIN_POSITIVE;
    let hi = f32::from_bits(0x0080_0001);
    threshold(lo, mid, hi);
    threshold(F32(lo), F32(mid), F32(hi));
    let lo = f64::from_bits(0x000f_ffff_ffff_ffff);
    let mid = f64::MIN_POSITIVE;
    let hi = f64::from_bits(0x0010_0000_0000_0001);
    threshold(lo, mid, hi);
    threshold(F64(lo), F64(mid), F64(hi));
}
fn state<T: RealField + UnitScalar>(a: T, s: T) -> DiffusionCoefficients<T> {
    DiffusionCoefficients::new(
        InteractionCoefficient::<T, Absorption>::new(ReciprocalLength::from_base(a))
            .expect("admitted absorption"),
        InteractionCoefficient::<T, ReducedScattering>::new(ReciprocalLength::from_base(s))
            .expect("admitted scattering"),
    )
    .expect("finite nonzero transport")
}
fn policies<T: RealField + UnitScalar>() {
    for input in [-T::ONE, T::INFINITY, -T::INFINITY, T::NAN] {
        let a = InteractionCoefficient::<T, Absorption>::new(ReciprocalLength::from_base(input))
            .map(|v| v.into_quantity().into_base());
        let s =
            InteractionCoefficient::<T, ReducedScattering>::new(ReciprocalLength::from_base(input))
                .map(|v| v.into_quantity().into_base());
        for (result, expected) in [
            (a, ValueKind::AbsorptionCoefficient),
            (s, ValueKind::ReducedScatteringCoefficient),
        ] {
            match result.expect_err("invalid input remains rejected before arithmetic") {
                TransportError::InvalidValue {
                    field,
                    value,
                    constraint,
                } => {
                    assert_eq!(field, expected);
                    assert_eq!(constraint, ValueConstraint::FiniteNonNegative);
                    if input.is_nan() {
                        assert!(value.is_nan());
                    } else {
                        assert_eq!(value, input);
                    }
                }
                other => panic!("wrong input error: {other:?}"),
            }
        }
    }
    for zero in [T::ZERO, -T::ZERO] {
        let effective = state(zero, T::ONE)
            .effective_attenuation()
            .expect("legitimate zero absorption");
        let value = effective.quantity().into_base();
        assert_eq!(value, T::ZERO);
        assert_eq!(value.is_sign_positive(), zero.is_sign_positive());
        assert_eq!(
            effective
                .penetration_depth()
                .expect("zero has no reciprocal"),
            None
        );
    }
    let exponent_bits = T::INFINITY.count_ones();
    let large = T::ONE.scale_binary((1_i32 << (exponent_bits - 1)) - 1);
    match state(large, T::ZERO)
        .effective_attenuation()
        .expect_err("nonfinite radicand policy")
    {
        TransportError::DerivedNonFinite { law, value } => {
            assert_eq!(law, TransportLaw::EffectiveAttenuation);
            assert_eq!(value, T::INFINITY);
        }
        other => panic!("wrong derived error: {other:?}"),
    }
}
#[test]
fn threshold_preserves_input_zero_and_nonfinite_policies() {
    policies::<f32>();
    policies::<f64>();
    policies::<F32>();
    policies::<F64>();
}
