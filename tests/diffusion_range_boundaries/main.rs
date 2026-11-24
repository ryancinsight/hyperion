//! Native range, zero policy and downstream diffusion regression coverage.
use aequitas::systems::si::quantities::ReciprocalLength;
use eunomia::{F32, F64, RealField, UnitScalar};
use hyperion::{
    TransportError, TransportLaw,
    coefficient::{Absorption, InteractionCoefficient, ReducedScattering},
    transport::{DiffusionCoefficients, OpticalDiffusionCoefficient},
};

mod downstream;

/// Quantity whose computed value is compared with its fixture reference, each
/// with the distance bound its own rounding sequence admits.
///
/// One step is one unit in the last place: adjacent finite values of one sign
/// differ by one in their integer encodings, across binades and into the
/// subnormals alike. Every fixture reference is the nearest-even rounding of
/// the exact real result `E` of the exact inputs
/// (`scripts/verify_diffusion_range.py`).
///
/// The computed value is the nearest rounding of `E' = E (1 + theta)`, the
/// exact result of the final IEEE operation on its rounded operands, where
/// `|theta| <= rho` collects every earlier rounding. Addition, multiplication,
/// division and square root round to nearest: each earlier rounding multiplies
/// by `1 + d` with `|d| <= u = 2^-p`, for significand precision `p` of 24 or
/// 53.
///
/// Let `I` send a value to its fractional encoding index, piecewise linear with
/// slope `1 / spacing` and integers at the encodable values. Rounding to
/// nearest rounds `I` to the nearest integer, so two roundings of reals at
/// index distance `delta` differ by a whole number of steps `n < delta + 1`.
/// A normal value `x` has spacing `2^(e + 1) u` with `|x| < 2^(e + 1)`, so
/// spacing exceeds `u |x|`, and `delta <= rho / u` to a factor `1 + O(u)`.
/// When `E'` and `E` straddle a power of two the integral of `1 / spacing`
/// between them takes the smaller spacing of the lower endpoint `y`, with
/// `|E' - E| <= rho |y| / (1 - rho)`, so the factor stays `1 + O(u)` and the
/// wider steps above the power of two only shrink the count.
///
/// `D = 1 / (3 (a + s))` rounds the sum and the tripling ahead of the
/// reciprocal, `rho = (1 - u)^-2 - 1 = 2u + O(u^2)`, so `n < 3 + O(u)` and
/// `n <= 3`.
///
/// `mu_eff = sqrt(3 a (a + s))` rounds the sum, the tripling of `a` and the
/// product ahead of the root. The root halves the relative error of the
/// radicand, `rho = (1 - u)^(-3/2) - 1 = 3u/2 + O(u^2)`, so `n < 5/2 + O(u)`
/// and `n <= 2`.
///
/// The relative model needs a normal rounded result. A subnormal sum or
/// tripling is exact. A radicand below the minimum normal never reaches the
/// root for positive absorption: the tiny-absorption branch scales both
/// operands by exact powers of two into `[1, 2)`, where every product and the
/// root are normal; only its final scaling can round into the subnormal range.
/// The reciprocal and the large-transport quotient can also round there.
///
/// A subnormal result has constant spacing and magnitude below `2^(p - 1)`
/// steps, so a relative `rho` spans below `rho 2^(p - 1) = rho / (2 u)` steps.
/// `D` with a subnormal reciprocal: `rho = 2u` spans below 1 step ahead of the
/// final reciprocal, `n < 2`. `D` when tripling overflows divides the
/// reciprocal of the sum by 3 last: the sum spans below 1/6 step after the
/// division, the reciprocal's rounding 1/6 more, `n < 4/3`. `mu_eff` from the
/// tiny-absorption branch: the radicand spans below `3/4` step and the root's
/// own rounding below `1/2` ahead of the final scaling, `n < 9/4`, `n <= 2`.
/// Every case stays within the bound of the normal range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Quantity {
    Diffusion,
    Effective,
}

impl Quantity {
    const ALL: [Self; 2] = [Self::Diffusion, Self::Effective];

    /// Largest admitted distance in encoding steps, from the derivation on
    /// [`Quantity`].
    const fn max_steps(self) -> u64 {
        match self {
            Self::Diffusion => 3,
            Self::Effective => 2,
        }
    }
}

trait Native: RealField + UnitScalar {
    const WIDTH: &'static str;
    fn from_raw_bits(bits: u64) -> Self;
    fn bits(self) -> u64;
    fn parse_bits(s: &str) -> Self {
        Self::from_raw_bits(u64::from_str_radix(s, 16).expect("hex bits"))
    }
    /// `e^-1` as `(hi, lo)`: `hi` is the nearest value of the format and `lo`
    /// the nearest value to `e^-1 - hi`, so `hi + lo` carries about twice the
    /// precision of `T`.
    ///
    /// Evaluated with mpmath 1.3.0 at 300 bits from
    /// `e^-1 = 0.36787944117144232159552377016146086744581113103176783450783...`;
    /// what `lo` leaves unrepresented has magnitude `4.3166e-16` in binary32
    /// and `5.8300e-34` in binary64, both far below one unit in the last place
    /// of `e^-1`.
    fn exp_neg_one_split() -> (Self, Self) {
        let (hi, lo) = if Self::WIDTH == "32" {
            ("3ebc5ab2", "b21d310d")
        } else {
            ("3fd78b56362cef38", "bc6ca8a4270fadf5")
        };
        (Self::parse_bits(hi), Self::parse_bits(lo))
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
/// Whether `got` is within the encoding-step bound of `quantity` from `want`.
fn within_ulp_bound<T: Native>(quantity: Quantity, got: T, want: T) -> bool {
    got.bits().abs_diff(want.bits()) <= quantity.max_steps()
}
/// A computed value and its fixture reference, both nonnegative.
struct Observation<T> {
    quantity: Quantity,
    line: &'static str,
    got: T,
    want: T,
}
impl<T: Native> Observation<T> {
    /// Encoding steps by which `got` lies above `want`; negative below.
    fn excess(&self) -> i128 {
        i128::from(self.got.bits()) - i128::from(self.want.bits())
    }
    /// The computed value moved by `steps` encodings, absent when no encoding
    /// exists there.
    fn displaced(&self, steps: i64) -> Option<T> {
        self.got
            .bits()
            .checked_add_signed(steps)
            .map(T::from_raw_bits)
    }
    /// Whether that displaced value exists and lies beyond the bound.
    fn beyond_bound_displaced(&self, steps: i64) -> bool {
        self.displaced(steps)
            .is_some_and(|got| !within_ulp_bound(self.quantity, got, self.want))
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
        include_str!("../diffusion_range_cases.txt"),
        include_str!("../diffusion_threshold_cases.txt"),
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
/// Evaluate every fixture row of `T`, asserting the error policy, positivity,
/// revalidation and zero policy, and return each computed value beside its
/// reference.
fn observe<T: Native>() -> Vec<Observation<T>> {
    let mut count = 0;
    let mut observations = Vec::new();
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
                observations.push(Observation {
                    quantity: Quantity::Diffusion,
                    line,
                    got: *got.as_base(),
                    want,
                });
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
                observations.push(Observation {
                    quantity: Quantity::Effective,
                    line,
                    got,
                    want,
                });
            }
        }
        count += 1;
    }
    assert!(count > 300, "range fixture covers both native formats");
    observations
}
/// Every computed value lies within its quantity's derived step bound.
fn neighborhoods<T: Native>() {
    for Observation {
        quantity,
        line,
        got,
        want,
    } in observe::<T>()
    {
        assert!(
            within_ulp_bound(quantity, got, want),
            "{quantity:?}: {line}; got {got:?}, want {want:?}"
        );
    }
}
#[test]
fn native_range_neighborhoods() {
    neighborhoods::<f32>();
    neighborhoods::<f64>();
    neighborhoods::<F32>();
    neighborhoods::<F64>();
}
/// A computed value moved past its quantity's bound fails, in both directions.
///
/// Moving every computed value of a quantity by `k` steps toward `direction`
/// is what a production defect displacing that quantity by `k` steps does.
/// The largest step count by which a row already lies toward `direction` is
/// `extreme`, so the bound leaves `slack = bound - extreme`. Every row passes
/// at `slack` steps, and exactly the rows lying at `extreme` fail at
/// `slack + 1`: a row at `e` steps lands at `e + slack + 1`, beyond the bound
/// exactly when `e >= extreme`. A bound loosened by one step passes the
/// displacement at `slack + 1`, so the count of failing rows is zero where
/// at least one is required.
fn over_bound_displacement_fails<T: Native>() {
    let observations = observe::<T>();
    for quantity in Quantity::ALL {
        let rows: Vec<_> = observations
            .iter()
            .filter(|row| row.quantity == quantity)
            .collect();
        assert!(!rows.is_empty(), "{quantity:?} has fixture rows");
        for direction in [1_i64, -1] {
            let toward = |row: &Observation<T>| i128::from(direction) * row.excess();
            let extreme = rows.iter().map(|row| toward(row)).max().expect("rows");
            let slack = i64::try_from(i128::from(quantity.max_steps()) - extreme)
                .expect("slack fits in 64 bits");
            assert!(slack >= 0, "{quantity:?} rows lie within the bound");
            assert!(
                rows.iter()
                    .all(|row| !row.beyond_bound_displaced(direction * slack)),
                "{quantity:?} at the slack of {slack} steps toward {direction}"
            );
            let at_extreme = rows
                .iter()
                .filter(|row| {
                    toward(row) == extreme && row.displaced(direction * (slack + 1)).is_some()
                })
                .count();
            assert!(at_extreme > 0, "{quantity:?} has a row at its extreme");
            let beyond = rows
                .iter()
                .filter(|row| row.beyond_bound_displaced(direction * (slack + 1)))
                .count();
            assert_eq!(
                beyond, at_extreme,
                "{quantity:?} one step past the slack toward {direction}"
            );
        }
    }
}
#[test]
fn over_bound_displacement_is_rejected_per_quantity() {
    over_bound_displacement_fails::<f32>();
    over_bound_displacement_fails::<f64>();
    over_bound_displacement_fails::<F32>();
    over_bound_displacement_fails::<F64>();
}
