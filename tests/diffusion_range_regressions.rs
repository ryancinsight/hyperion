//! Bounded native-range audit of current Hyperion diffusion contracts.
use aequitas::systems::si::quantities::ReciprocalLength;
use eunomia::{RealField, UnitScalar};
use hyperion::{
    TransportError, TransportLaw, ValueConstraint,
    coefficient::{Absorption, InteractionCoefficient, ReducedScattering},
    transport::{DiffusionCoefficients, OpticalDiffusionCoefficient},
};

fn pair<T: RealField + UnitScalar>(a: T, s: T) -> DiffusionCoefficients<T> {
    DiffusionCoefficients::new(
        InteractionCoefficient::<T, Absorption>::new(ReciprocalLength::from_base(a))
            .expect("finite nonnegative absorption"),
        InteractionCoefficient::<T, ReducedScattering>::new(ReciprocalLength::from_base(s))
            .expect("finite nonnegative reduced scattering"),
    )
    .expect("finite positive transport coefficient")
}
fn three<T: RealField>() -> T {
    T::ONE + T::ONE + T::ONE
}

fn positive_diffusion<T: RealField + UnitScalar>(power: T) {
    // power=2^emax, so its reciprocal is exact and /3 rounds only once.
    // True D is representable and positive although the intermediate3*power
    // overflows. Returning Ok(0) also violates the wrapper constructor.
    let expected = power.recip() / three::<T>();
    assert!(expected.is_finite() && expected > T::ZERO);
    let actual = pair(power, T::ZERO)
        .diffusion_coefficient()
        .expect("representable positive diffusion coefficient")
        .into_quantity();
    assert_eq!(*actual.as_base(), expected);
    let revalidated = OpticalDiffusionCoefficient::new(actual)
        .expect("a derived validated value must satisfy its public constructor");
    assert_eq!(*revalidated.quantity().as_base(), expected);
}
#[test]
fn diffusion_positive_single() {
    positive_diffusion(f32::from_bits(254_u32 << 23));
}
#[test]
fn diffusion_positive_double() {
    positive_diffusion(f64::from_bits(2046_u64 << 52));
}

fn finite_effective<T: RealField + UnitScalar>(power: T) {
    // Pure absorption has mu_eff=sqrt(3)*mu_a. A normal power-of-two scale
    // makes the reference an exact exponent shift of correctly rounded sqrt3.
    let expected = three::<T>().sqrt() * power;
    assert!(expected.is_finite() && expected > T::ZERO);
    let actual = pair(power, T::ZERO)
        .effective_attenuation()
        .expect("positive finite effective attenuation")
        .into_quantity()
        .into_base();
    assert_eq!(actual, expected);
}
#[test]
fn effective_underflow_single() {
    finite_effective(2.0_f32.powi(-80));
}
#[test]
fn effective_underflow_double() {
    finite_effective(2.0_f64.powi(-600));
}

fn ordinary<T: RealField + UnitScalar>() {
    let p = pair(T::ONE, T::ZERO);
    let d = p
        .diffusion_coefficient()
        .expect("ordinary diffusion")
        .into_quantity();
    assert_eq!(*d.as_base(), T::ONE / three::<T>());
    assert_eq!(
        *OpticalDiffusionCoefficient::new(d)
            .expect("valid D")
            .quantity()
            .as_base(),
        T::ONE / three::<T>()
    );
    assert_eq!(
        p.effective_attenuation()
            .expect("ordinary attenuation")
            .into_quantity()
            .into_base(),
        three::<T>().sqrt()
    );
    assert_eq!(
        p.transport_mean_free_path()
            .expect("ordinary path")
            .into_quantity()
            .into_base(),
        T::ONE
    );
    assert_eq!(
        p.transport_albedo()
            .expect("ordinary albedo")
            .into_quantity()
            .into_base(),
        T::ZERO
    );
}
#[test]
fn ordinary_native_controls() {
    ordinary::<f32>();
    ordinary::<f64>();
}

fn zero_absorption<T: RealField + UnitScalar>() {
    let p = pair(T::ZERO, T::ONE);
    assert_eq!(
        p.diffusion_coefficient()
            .expect("positive D")
            .into_quantity()
            .into_base(),
        T::ONE / three::<T>()
    );
    let attenuation = p
        .effective_attenuation()
        .expect("legitimate zero attenuation");
    assert_eq!(*attenuation.quantity().as_base(), T::ZERO);
    assert_eq!(
        attenuation
            .penetration_depth()
            .expect("defined zero boundary"),
        None
    );
}
#[test]
fn legitimate_zero_absorption_controls() {
    zero_absorption::<f32>();
    zero_absorption::<f64>();
}

fn invalid<T: RealField + UnitScalar>() {
    for value in [T::NAN, T::INFINITY, -T::ONE] {
        let e = InteractionCoefficient::<T, Absorption>::new(ReciprocalLength::from_base(value))
            .expect_err("invalid input rejected");
        match e {
            TransportError::InvalidValue {
                value: got,
                constraint,
                ..
            } => {
                assert_eq!(constraint, ValueConstraint::FiniteNonNegative);
                assert!(if value.is_nan() {
                    got.is_nan()
                } else {
                    got == value
                });
            }
            _ => panic!("input guard must retain its typed error"),
        }
    }
    let a = InteractionCoefficient::<T, Absorption>::new(ReciprocalLength::from_base(T::ZERO))
        .expect("valid zero");
    let s =
        InteractionCoefficient::<T, ReducedScattering>::new(ReciprocalLength::from_base(T::ZERO))
            .expect("valid zero");
    assert_eq!(
        DiffusionCoefficients::new(a, s),
        Err(TransportError::DegenerateTransport)
    );
}
#[test]
fn invalid_and_degenerate_controls() {
    invalid::<f32>();
    invalid::<f64>();
}

fn documented_overflow<T: RealField + UnitScalar>(power: T) {
    let e = pair(power, T::ZERO)
        .effective_attenuation()
        .expect_err("documented nonfinite radicand rejection");
    match e {
        TransportError::DerivedNonFinite { law, value } => {
            assert_eq!(law, TransportLaw::EffectiveAttenuation);
            assert!(!value.is_finite());
        }
        _ => panic!("documented radicand guard must keep its typed error"),
    }
}
#[test]
fn documented_radicand_overflow_control() {
    documented_overflow(2.0_f32.powi(80));
    documented_overflow(2.0_f64.powi(600));
}
