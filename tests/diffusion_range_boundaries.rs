//! Native range, zero policy and downstream diffusion regression coverage.
use aequitas::systems::si::quantities::{EnergyPerArea, Length, ReciprocalLength};
use eunomia::{F32, F64, RealField, UnitScalar};
use hyperion::{
    TransportError, TransportLaw,
    coefficient::{Absorption, InteractionCoefficient, ReducedScattering},
    quantity::{EnergyFluence, PathLength},
    transport::{DiffusionCoefficients, OpticalDiffusionCoefficient, planar_fluence_at_depth},
};
/// Largest admitted distance, in encoding steps, between a computed coefficient
/// and its fixture reference.
///
/// One step is one unit in the last place: adjacent finite values of one sign
/// differ by one in their integer encodings, across binades and into the
/// subnormals alike. Every fixture reference is the nearest-even rounding of
/// the exact real result of the exact inputs
/// (`scripts/verify_diffusion_range.py`), so it lies within half a step of
/// that result.
///
/// The computation under test uses IEEE addition, multiplication, division
/// and square root, each rounding to nearest: the rounded result is the exact
/// one times `1 + d` with `|d| <= u = 2^-p`, for significand precision `p` of
/// 24 or 53. `D = 1 / (3 (a + s))` executes three
/// roundings: sum, tripling, reciprocal. `mu_eff = sqrt(3 a (a + s))` rounds
/// its radicand three times (sum, tripling `a`, product) and the root once;
/// the root halves the relative effect of the radicand roundings, so the
/// relative error is at most `(3/2 + 1) u` to first order. Both stay within the
/// three-rounding bound `3 u / (1 - 3 u)`.
///
/// A normal value `x` has encoding spacing `2^(e + 1 - p) = 2^(e + 1) u`
/// with `|x| < 2^(e + 1)`, so spacing exceeds `u |x|` and a relative error `r`
/// spans at most `r / u` steps. The result is within `3 / (1 - 3 u)`, below
/// `3 + 10^-6`, steps of the exact value and below `3.5 + 10^-6` steps of the
/// reference. Both are encodings, so their distance is an integer: at most 3.
///
/// The spacing premise holds at the exact value `x`. When the computed value
/// `y` falls below a power of two while `x` lies above it, the spacing at `y`
/// is half that at `x`, so the premise fails at `x` but holds at `y`: spacing
/// at `y` exceeds `u |y|`, and `|y - x| <= r |x|` with `|x| <= |y| / (1 - r)`
/// spans at most `r / (u (1 - r))` steps counted at the spacing of `y`. The
/// steps above the power of two are wider, so the count is no larger, and the
/// bound across a binade crossing is `r / u + O(u)` as in the normal range.
///
/// The relative model needs a normal rounded result. A subnormal sum or
/// tripling is exact. A radicand below the minimum normal never reaches the
/// root for positive absorption: the tiny-absorption branch scales both
/// operands by exact powers of two into `[1, 2)`, where every product and the
/// root are normal; only its final scaling can round into the subnormal range.
/// The reciprocal and the large-transport quotient can also round there.
///
/// A subnormal result has constant spacing and magnitude below `2^(p - 1)`
/// spacings, so a relative error `r` spans at most `r / (2 u)` steps, and a
/// rounding into the subnormal range is an absolute half step. `D` with a
/// subnormal reciprocal: sum and tripling span 1 step, the reciprocal half a
/// step, the reference half a step, 2 in total. `D` when tripling overflows:
/// the sum's error spans 1/6 step after the division by 3, the reciprocal's
/// rounding 1/6 more, the division's rounding 1/2 and the reference 1/2, below
/// 1.4. `mu_eff` from the tiny-absorption branch: `2.5 / 2` steps from the
/// relative roundings, 1/2 for the final scaling and 1/2 for the reference,
/// 2.25. Every case stays below the normal-range bound.
const MAX_ULP_DISTANCE: u64 = 3;

trait Native: RealField + UnitScalar {
    const WIDTH: &'static str;
    fn from_raw_bits(bits: u64) -> Self;
    fn bits(self) -> u64;
    fn parse_bits(s: &str) -> Self {
        Self::from_raw_bits(u64::from_str_radix(s, 16).expect("hex bits"))
    }
}
impl Native for f32 {
    const WIDTH: &'static str = "32";
    fn from_raw_bits(bits: u64) -> Self {
        Self::from_bits(u32::try_from(bits).expect("binary32 bits fit in 32 bits"))
    }
    fn bits(self) -> u64 {
        u64::from(self.to_bits())
    }
}
impl Native for f64 {
    const WIDTH: &'static str = "64";
    fn from_raw_bits(bits: u64) -> Self {
        Self::from_bits(bits)
    }
    fn bits(self) -> u64 {
        self.to_bits()
    }
}
impl Native for F32 {
    const WIDTH: &'static str = "32";
    fn from_raw_bits(bits: u64) -> Self {
        Self(f32::from_raw_bits(bits))
    }
    fn bits(self) -> u64 {
        self.0.bits()
    }
}
impl Native for F64 {
    const WIDTH: &'static str = "64";
    fn from_raw_bits(bits: u64) -> Self {
        Self(f64::from_raw_bits(bits))
    }
    fn bits(self) -> u64 {
        self.0.bits()
    }
}
/// Whether `got` is within [`MAX_ULP_DISTANCE`] encoding steps of `want`.
fn within_ulp_bound<T: Native>(got: T, want: T) -> bool {
    got.bits().abs_diff(want.bits()) <= MAX_ULP_DISTANCE
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
/// A fixture reference: the documented overflow error or the reference value.
enum Reference<T> {
    Overflow,
    Value(T),
}
impl<T: Native> Reference<T> {
    fn parse(field: &str) -> Self {
        if field == "err" {
            Self::Overflow
        } else {
            Self::Value(T::parse_bits(field))
        }
    }
}
struct Case<T> {
    line: &'static str,
    state: DiffusionCoefficients<T>,
    diffusion: Reference<T>,
    effective: Reference<T>,
}
/// Fixture rows of `T`'s binary width, with inputs validated into coefficients.
fn cases<T: Native>() -> impl Iterator<Item = Case<T>> {
    [
        include_str!("diffusion_range_cases.txt"),
        include_str!("diffusion_threshold_cases.txt"),
    ]
    .into_iter()
    .flat_map(str::lines)
    .filter_map(|line| {
        let fields: Vec<_> = line.split_whitespace().collect();
        (fields[0] == T::WIDTH).then(|| Case {
            line,
            state: pair(T::parse_bits(fields[1]), T::parse_bits(fields[2])),
            diffusion: Reference::parse(fields[3]),
            effective: Reference::parse(fields[4]),
        })
    })
}
fn neighborhoods<T: Native>() {
    let mut count = 0;
    for Case {
        line,
        state,
        diffusion,
        effective,
    } in cases::<T>()
    {
        let d = state.diffusion_coefficient();
        match diffusion {
            Reference::Overflow => match d.expect_err("true reciprocal overflow") {
                TransportError::DerivedNonFinite { law, value } => {
                    assert_eq!(law, TransportLaw::DiffusionCoefficient);
                    assert!(!value.is_finite());
                }
                _ => panic!("preserve typed derived error"),
            },
            Reference::Value(want) => {
                let got = d.expect("representable positive D").into_quantity();
                assert!(*got.as_base() > T::ZERO, "positive invariant: {line}");
                assert!(
                    within_ulp_bound(*got.as_base(), want),
                    "D: {line}; got {got:?}"
                );
                assert_eq!(
                    OpticalDiffusionCoefficient::new(got)
                        .expect("derived value revalidates")
                        .into_quantity(),
                    got
                );
            }
        }
        let effective_result = state.effective_attenuation();
        match effective {
            Reference::Overflow => {
                match effective_result.expect_err("documented radicand overflow") {
                    TransportError::DerivedNonFinite { law, value } => {
                        assert_eq!(law, TransportLaw::EffectiveAttenuation);
                        assert!(!value.is_finite());
                    }
                    _ => panic!("preserve typed radicand policy"),
                }
            }
            Reference::Value(want) => {
                let got = effective_result
                    .expect("finite radicand and result")
                    .into_quantity()
                    .into_base();
                assert_eq!(got == T::ZERO, want == T::ZERO, "zero policy: {line}");
                assert!(within_ulp_bound(got, want), "mu_eff: {line}; got {got:?}");
            }
        }
        count += 1;
    }
    assert!(count > 300, "range fixture covers both native formats");
}
#[test]
fn native_range_neighborhoods() {
    neighborhoods::<f32>();
    neighborhoods::<f64>();
    neighborhoods::<F32>();
    neighborhoods::<F64>();
}
/// Higham's `gamma_n = n u / (1 - n u)` for `n` correctly rounded operations.
///
/// `u = EPSILON / 2` is the unit roundoff of round-to-nearest. Any product or
/// quotient of `n` factors `1 + d` with `|d| <= u` is `1 + theta` with
/// `|theta| <= gamma_n` (Higham, Accuracy and Stability of Numerical
/// Algorithms, 2nd ed., Lemma 3.1). Evaluating the bound in `T` rounds the
/// difference and the quotient once each.
fn gamma<T: Native>(operations: u32) -> T {
    let two = T::ONE + T::ONE;
    let n = (0..operations).fold(T::ZERO, |count, _| count + T::ONE);
    let n_u = n * (T::EPSILON / two);
    n_u / (T::ONE - n_u)
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
    // `expected` is the root rounded once and the product rounded once. The
    // attenuation is within `gamma_3` of the exact `sqrt(3) a` (see
    // `MAX_ULP_DISTANCE`). Each reciprocal rounds once, so the two reciprocals
    // differ relatively by at most `gamma_(1 + 2 + 3 + 1)`.
    let tolerance = gamma::<T>(7) * expected.recip();
    assert!((*path.quantity().as_base() - expected.recip()).abs() <= tolerance);
    let depth = PathLength::new(Length::from_base(expected.recip()))
        .expect("finite expected attenuation length");
    let incident =
        EnergyFluence::new(EnergyPerArea::from_base(T::ONE)).expect("unit incident fluence");
    let got = planar_fluence_at_depth(incident, attenuation, depth)
        .expect("finite attenuated fluence")
        .into_quantity()
        .into_base();
    let want = (-T::ONE).exp();
    // Carried from the transport suite's libm exp composition tolerance; the
    // exp error term is not derived here.
    let eight = T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE;
    let thirty_two = eight * (T::ONE + T::ONE + T::ONE + T::ONE);
    assert!((got - want).abs() <= thirty_two * T::EPSILON * want);
}
#[test]
fn recovered_attenuation_feeds_real_planar_consumer() {
    downstream(2.0_f32.powi(-80));
    downstream(2.0_f64.powi(-600));
    downstream(F32(2.0_f32.powi(-80)));
    downstream(F64(2.0_f64.powi(-600)));
}
