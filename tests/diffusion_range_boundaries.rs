//! Native range, zero policy and downstream diffusion regression coverage.
use aequitas::systems::si::quantities::{EnergyPerArea, Length, ReciprocalLength};
use eunomia::{RealField, UnitScalar};
use hyperion::{
    TransportError, TransportLaw,
    coefficient::{Absorption, InteractionCoefficient, ReducedScattering},
    quantity::{EnergyFluence, PathLength},
    transport::{DiffusionCoefficients, OpticalDiffusionCoefficient, planar_fluence_at_depth},
};
trait Native: RealField + UnitScalar {
    const WIDTH: &'static str;
    fn parse_bits(s: &str) -> Self;
    fn bits(self) -> u64;
}
impl Native for f32 {
    const WIDTH: &'static str = "32";
    fn parse_bits(s: &str) -> Self {
        Self::from_bits(u32::from_str_radix(s, 16).expect("hex bits"))
    }
    fn bits(self) -> u64 {
        u64::from(self.to_bits())
    }
}
impl Native for f64 {
    const WIDTH: &'static str = "64";
    fn parse_bits(s: &str) -> Self {
        Self::from_bits(u64::from_str_radix(s, 16).expect("hex bits"))
    }
    fn bits(self) -> u64 {
        self.to_bits()
    }
}
fn pair<T: Native>(a: T, s: T) -> DiffusionCoefficients<T> {
    DiffusionCoefficients::new(
        InteractionCoefficient::<T, Absorption>::new(ReciprocalLength::from_base(a))
            .expect("finite nonnegative a"),
        InteractionCoefficient::<T, ReducedScattering>::new(ReciprocalLength::from_base(s))
            .expect("finite nonnegative s"),
    )
    .expect("positive finite transport")
}
fn neighborhoods<T: Native>() {
    let mut count = 0;
    for line in [
        include_str!("diffusion_range_cases.txt"),
        include_str!("diffusion_threshold_cases.txt"),
    ]
    .into_iter()
    .flat_map(str::lines)
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields[0] != T::WIDTH {
            continue;
        }
        let state = pair(T::parse_bits(fields[1]), T::parse_bits(fields[2]));
        let d = state.diffusion_coefficient();
        if fields[3] == "err" {
            match d.expect_err("true reciprocal overflow") {
                TransportError::DerivedNonFinite { law, value } => {
                    assert_eq!(law, TransportLaw::DiffusionCoefficient);
                    assert!(!value.is_finite());
                }
                _ => panic!("preserve typed derived error"),
            }
        } else {
            let got = d.expect("representable positive D").into_quantity();
            let want = T::parse_bits(fields[3]);
            assert!(*got.as_base() > T::ZERO, "positive invariant: {line}");
            assert!(
                got.as_base().bits().abs_diff(want.bits()) <= 8,
                "D: {line}; got {got:?}"
            );
            assert_eq!(
                OpticalDiffusionCoefficient::new(got)
                    .expect("derived value revalidates")
                    .into_quantity(),
                got
            );
        }
        let effective = state.effective_attenuation();
        if fields[4] == "err" {
            match effective.expect_err("documented radicand overflow") {
                TransportError::DerivedNonFinite { law, value } => {
                    assert_eq!(law, TransportLaw::EffectiveAttenuation);
                    assert!(!value.is_finite());
                }
                _ => panic!("preserve typed radicand policy"),
            }
        } else {
            let got = effective
                .expect("finite radicand and result")
                .into_quantity()
                .into_base();
            let want = T::parse_bits(fields[4]);
            assert_eq!(got == T::ZERO, want == T::ZERO, "zero policy: {line}");
            assert!(
                got.bits().abs_diff(want.bits()) <= 8,
                "mu_eff: {line}; got {got:?}"
            );
        }
        count += 1;
    }
    assert!(count > 300, "range fixture covers both native formats");
}
#[test]
fn native_range_neighborhoods() {
    neighborhoods::<f32>();
    neighborhoods::<f64>();
}
fn downstream<T: Native>(a: T) {
    let state = pair(a, T::ZERO);
    let expected = (T::ONE + T::ONE + T::ONE).sqrt() * a;
    let attenuation = state
        .effective_attenuation()
        .expect("finite positive effective attenuation");
    let path = attenuation
        .penetration_depth()
        .expect("finite penetration")
        .expect("positive attenuation has finite penetration");
    let eight = T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE;
    assert!(
        (*path.quantity().as_base() - expected.recip()).abs()
            <= eight * T::EPSILON * expected.recip()
    );
    let depth = PathLength::new(Length::from_base(expected.recip()))
        .expect("finite expected attenuation length");
    let incident =
        EnergyFluence::new(EnergyPerArea::from_base(T::ONE)).expect("unit incident fluence");
    let got = planar_fluence_at_depth(incident, attenuation, depth)
        .expect("finite attenuated fluence")
        .into_quantity()
        .into_base();
    let want = (-T::ONE).exp();
    // Existing transport suite uses gamma32 for libm exp and composition.
    let thirty_two = eight * (T::ONE + T::ONE + T::ONE + T::ONE);
    assert!((got - want).abs() <= thirty_two * T::EPSILON * want);
}
#[test]
fn recovered_attenuation_feeds_real_planar_consumer() {
    downstream(2.0_f32.powi(-80));
    downstream(2.0_f64.powi(-600));
}
