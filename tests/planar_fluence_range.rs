//! Planar fluence must retain a representable product when transmission underflows.

use aequitas::systems::si::quantities::{EnergyPerArea, Length, ReciprocalLength};
use eunomia::{F32, F64, FloatElement, NumericElement, RealField, UnitScalar};
use hyperion::{
    TransportError, TransportLaw,
    coefficient::{EffectiveAttenuation, InteractionCoefficient},
    quantity::{EnergyFluence, PathLength},
    transport::planar_fluence_at_depth,
};

fn fluence<T: RealField + UnitScalar>(incident: T, optical_depth: T) -> T {
    let surface = EnergyFluence::new(EnergyPerArea::from_base(incident))
        .expect("finite nonnegative incident fluence");
    let attenuation = InteractionCoefficient::<T, EffectiveAttenuation>::new(
        ReciprocalLength::from_base(<T as NumericElement>::ONE),
    )
    .expect("positive unit attenuation");
    let depth =
        PathLength::new(Length::from_base(optical_depth)).expect("positive finite path length");
    planar_fluence_at_depth(surface, attenuation, depth)
        .expect("the exact planar fluence is finite")
        .into_quantity()
        .into_base()
}

fn assert_representable_product<T: RealField + UnitScalar>(
    incident: T,
    optical_depth: T,
    expected: T,
    tolerance: T,
) {
    let actual = fluence(incident, optical_depth);
    let relative_error = ((actual - expected) / expected).abs();
    assert!(
        relative_error <= tolerance,
        "incident={incident:?}, tau={optical_depth:?}, actual={actual:?}, expected={expected:?}, relative_error={relative_error:?}"
    );
}

#[test]
fn native_single_precision_retains_representable_planar_fluence() {
    // 220-digit Decimal oracle: 2^120 * exp(-128), rounded to binary32.
    assert_representable_product(
        f32::from_bits(0x7b80_0000),
        128.0,
        f32::from_bits(0x1f21_75d0),
        16.0 * f32::EPSILON,
    );
}

#[test]
fn native_double_precision_retains_representable_planar_fluence() {
    // 220-digit Decimal oracle: 2^1000 * exp(-1024), rounded to binary64.
    assert_representable_product(
        f64::from_bits(0x7e70_0000_0000_0000),
        1024.0,
        f64::from_bits(0x2219_a3a1_32ee_86ba),
        16.0 * f64::EPSILON,
    );
}

fn reference_cases<T: RealField + UnitScalar>(
    width: &str,
    from_bits: impl Fn(u64) -> T,
    to_bits: impl Fn(T) -> u64,
) {
    // Fixtures evaluate the documented equation using independent Decimal
    // exponentials at 160 and 240 digits, then round directly to native bits.
    // The recovery uses one exp result four times and four products: its
    // ordinary relative-rounding model has eight contributions, plus the
    // rounded reference. Sixteen ULPs also cover the native exp's error;
    // this is a measured implementation bound, not a universal exp theorem.
    let mut count = 0;
    for line in include_str!("planar_fluence_cases.txt").lines() {
        if line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields[0] != width {
            continue;
        }
        assert_eq!(fields.len(), 4);
        let incident_bits = u64::from_str_radix(fields[1], 16).expect("source bits");
        let tau_bits = u64::from_str_radix(fields[2], 16).expect("depth bits");
        let expected_bits = u64::from_str_radix(fields[3], 16).expect("reference bits");
        let actual_bits = to_bits(fluence(from_bits(incident_bits), from_bits(tau_bits)));
        if expected_bits == 0 {
            assert_eq!(actual_bits, 0, "legitimate final underflow: {line}");
        } else {
            assert!(actual_bits > 0, "representable positive fluence: {line}");
            assert!(
                actual_bits.abs_diff(expected_bits) <= 16,
                "{line}: actual={actual_bits:x}, distance={}",
                actual_bits.abs_diff(expected_bits)
            );
        }
        assert!(
            actual_bits <= incident_bits,
            "attenuation cannot amplify: {line}"
        );
        count += 1;
    }
    assert_eq!(
        count, 185,
        "the selected native fixture domain was exercised"
    );
}

#[test]
fn high_precision_range_oracles_cover_all_supported_scalars() {
    reference_cases(
        "32",
        |bits| f32::from_bits(u32::try_from(bits).expect("32 bits")),
        |value| u64::from(value.to_bits()),
    );
    reference_cases("64", f64::from_bits, f64::to_bits);
    reference_cases(
        "32",
        |bits| F32::from_f32(f32::from_bits(u32::try_from(bits).expect("32 bits"))),
        |value| u64::from(value.to_f32().to_bits()),
    );
    reference_cases(
        "64",
        |bits| F64::from_f64(f64::from_bits(bits)),
        |value| value.to_f64().to_bits(),
    );
}

fn underflow_rounding_cases<T: RealField + UnitScalar>(
    width: &str,
    from_bits: impl Fn(u64) -> T,
    to_bits: impl Fn(T) -> u64,
) {
    let mut count = 0;
    let mut exact_controls = 0;
    for line in include_str!("planar_fluence_underflow_cases.txt").lines() {
        if line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields[0] != width {
            continue;
        }
        assert_eq!(fields.len(), 6);
        let bits: Vec<_> = fields[1..]
            .iter()
            .map(|field| {
                u64::from_str_radix(field, 16).expect("independent underflow fixture bits")
            })
            .collect();
        let actual = to_bits(fluence(from_bits(bits[0]), from_bits(bits[1])));
        assert!(
            actual.abs_diff(bits[2]) <= 1,
            "absolute minimum-subnormal error: {line}"
        );
        assert!(
            (bits[3]..=bits[4]).contains(&actual),
            "outside independent roundoff interval: {line}; actual={actual:x}"
        );
        if bits[3] == bits[4] {
            exact_controls += 1;
        }
        count += 1;
    }
    assert_eq!(count, 34);
    assert!(
        exact_controls >= 4,
        "controls away from the rounding tie stay exact"
    );
}

#[test]
fn final_underflow_has_bounded_absolute_rounding_error() {
    underflow_rounding_cases(
        "32",
        |bits| f32::from_bits(u32::try_from(bits).expect("32 bits")),
        |value| u64::from(value.to_bits()),
    );
    underflow_rounding_cases("64", f64::from_bits, f64::to_bits);
    underflow_rounding_cases(
        "32",
        |bits| F32::from_f32(f32::from_bits(u32::try_from(bits).expect("32 bits"))),
        |value| u64::from(value.to_f32().to_bits()),
    );
    underflow_rounding_cases(
        "64",
        |bits| F64::from_f64(f64::from_bits(bits)),
        |value| value.to_f64().to_bits(),
    );
}

fn preserved_contracts<T: RealField + UnitScalar>(finite_maximum: T, minimum_subnormal: T) {
    for incident in [T::ZERO, -T::ZERO] {
        for tau in [T::ZERO, T::ONE, T::from_f64(1500.0), finite_maximum] {
            let actual = fluence(incident, tau);
            assert_eq!(actual, incident);
            assert_eq!(actual.is_sign_positive(), incident.is_sign_positive());
        }
    }
    for incident in [minimum_subnormal, T::ONE, T::from_f64(10.0), finite_maximum] {
        for tau in [
            T::ZERO,
            -T::ZERO,
            T::from_f64(0.125),
            T::ONE,
            T::from_f64(8.0),
        ] {
            let actual = fluence(incident, tau);
            assert_eq!(
                actual,
                incident * (-tau).exp(),
                "ordinary path retains native bits"
            );
        }
    }
    let attenuation = InteractionCoefficient::<T, EffectiveAttenuation>::new(
        ReciprocalLength::from_base(finite_maximum),
    )
    .expect("finite coefficient");
    let depth = PathLength::new(Length::from_base(finite_maximum)).expect("finite path");
    for incident in [T::ZERO, -T::ZERO, T::ONE] {
        let surface = EnergyFluence::new(EnergyPerArea::from_base(incident))
            .expect("finite incident fluence");
        assert_eq!(
            planar_fluence_at_depth(surface, attenuation, depth),
            Err(TransportError::DerivedNonFinite {
                law: TransportLaw::OpticalDepth,
                value: T::INFINITY,
            })
        );
    }
}

#[test]
fn zero_ordinary_and_optical_depth_error_contracts_are_preserved() {
    // NumericElement::MAX_VALUE is the min-reduction identity (infinity).
    preserved_contracts(f32::MAX, f32::from_bits(1));
    preserved_contracts(f64::MAX, f64::from_bits(1));
    preserved_contracts(F32::from_f32(f32::MAX), F32::from_f32(f32::from_bits(1)));
    preserved_contracts(F64::from_f64(f64::MAX), F64::from_f64(f64::from_bits(1)));
}
