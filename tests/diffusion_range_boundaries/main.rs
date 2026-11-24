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
/// `theta` collects every earlier rounding. Addition, multiplication, division
/// and square root round to nearest: each earlier rounding multiplies by
/// `1 + d` with `|d| <= u = 2^-p`, for significand precision `p` of 24 or 53,
/// and is exact when its result is subnormal. `recip` is the division `1 / x`.
///
/// Let `I` send a value to its fractional encoding index, piecewise linear with
/// slope `1 / spacing` and integers at the encodable values. Rounding to
/// nearest rounds `I` to the nearest integer, so two roundings of reals at
/// index distance `delta` differ by a whole number of steps `n <= delta + 1`
/// (equality at ties), hence `n <= floor(delta) + 1`: `delta < 2` gives
/// `n <= 2` and `delta < 1` gives `n <= 1`. A normal value `x` has spacing
/// `2^(e + 1) u` with `|x| < 2^(e + 1)`, so spacing exceeds `u |x|`. When `E'`
/// and `E` straddle a power of two the integral of `1 / spacing` between them
/// takes the smaller spacing of the lower endpoint, and the wider steps above
/// the power of two only shrink the count. Every bound below holds to a factor
/// `1 + O(u)` on `delta`, below `2^-20` for both precisions, and every `delta`
/// bound lies at least `1/8` below the integer that gives its stated count.
///
/// `D = 1 / (3 (a + s))` evaluates `t = fl(a + s)`, `den = fl(3 t)` and
/// `D = fl(1 / den)`, so `E' = E / ((1 + d1) (1 + d2))` with `d1` the sum's
/// rounding and `d2` the tripling's. Independent bounds `|d1|, |d2| <= u` give
/// `delta <= 2` and `n <= 3`; the roundings are coupled through the binade of
/// `3 t = m 2^k`, `1 <= m < 2`, so `n <= 2`:
///
/// - The tripling errs by at most half the spacing `2^k 2u` of its binade,
///   `|d2| <= u / m`.
/// - `t = 3 t / 3` has significand `4m/3` for `m < 3/2` and `2m/3` otherwise,
///   and the sum errs by at most `u` times its binade, so `|d1| <= 3u / (4m)`
///   for `m < 3/2` and `|d1| <= 3u / (2m)` otherwise.
/// - `E'` lies in `[2^(-k-1), 2^-k]`, a binade of spacing `2^-k u`, and `E`
///   within `O(u)` of it, so `delta <= |d1 + d2| / (m u)`: at most
///   `7 / (4 m^2) <= 7/4` for `m < 3/2` and `5 / (2 m^2) <= 10/9` otherwise.
/// - When the tripling rounds up to `2^(k+1)` and `E` lies below `2^(-k-1)`,
///   where the spacing halves, `m >= 2 - O(u)`, `|d1 + d2| <= 5u/4` and
///   `delta <= 5/4`.
/// - A subnormal sum or tripling is exact, which only removes a term: with
///   `d1 = 0`, `delta <= 1 / m^2 <= 1`, and with both exact `n = 0`.
///
/// So `delta <= 7/4 + O(u) < 2` and `n <= 2` on the normal path. The other
/// branches stay below it:
///
/// - A subnormal reciprocal has constant spacing `q = 2^emin 2u` and
///   `E' < 2^emin`, so `delta <= E' |d1 + d2| / q < |d1 + d2| / (2u)`
///   `<= 7/8` and `n <= 1`.
/// - When `3 t` overflows, `D = fl(fl(1 / t) / 3)`. Then
///   `t >= 2^(emax - 1) 4/3 (1 - u)`, so `1 / t` is subnormal, below
///   `3 * 2^(p - 3)` steps `q`, and the sum's rounding moves `E` by at most
///   `u 3 * 2^(p - 3) / 3 = 1/8` step after the division. The reciprocal's
///   rounding is at most half a step, `1/6` after it. `delta < 1/3` and
///   `n <= 1`.
///
/// `mu_eff = sqrt(3 a (a + s))` rounds the sum, the tripling of `a` and the
/// product ahead of the root. The root halves the relative error of the
/// radicand, `|theta| <= (1 - u)^(-3/2) - 1 = 3u/2 + O(u^2)`, so
/// `delta <= 3/2 + O(u)`, `n <= 5/2 + O(u)` and `n <= 2`. The derivation
/// takes the three radicand roundings as independent and makes no coupling
/// argument, so 2 may be unattained: no fixture row exceeds one step and an
/// exhaustive search of every pair in `[1/4, 4)` at 7 and 8 significand bits
/// finds at most one. A coupled derivation of `delta < 1`, or an input pair at
/// two steps, settles it.
///
/// A radicand below the minimum normal never reaches the root for positive
/// absorption: the tiny-absorption branch scales both operands by exact powers
/// of two into `[1, 2)`, where every product and the root are normal; only its
/// final scaling can round into the subnormal range, where the spacing is
/// constant and a result is below `2^(p - 1)` steps, so a relative `theta`
/// spans below `theta / (2u)` steps. The radicand's `3u/2` then spans below
/// `3/4` step and the root's own rounding below `1/2` ahead of the scaling,
/// `n <= 9/4` and `n <= 2`.
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
            Self::Diffusion => 2,
            Self::Effective => 2,
        }
    }
}

trait Native: RealField + UnitScalar {
    const WIDTH: &'static str;
    fn from_raw_bits(bits: u64) -> Self;
    fn bits(self) -> u64;
    /// The encoding of the sign bit.
    fn sign_bit() -> u64 {
        if Self::WIDTH == "32" {
            1 << 31
        } else {
            1 << 63
        }
    }
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
/// A computed value and its fixture reference. Only the negative zero of a
/// signed-zero input has its sign bit set.
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
    /// The computed value moved by `steps` encodings, absent when it has its
    /// sign bit set, where encodings move away from zero, or when no
    /// sign-clear encoding exists there.
    fn displaced(&self, steps: i64) -> Option<T> {
        let bits = self.got.bits();
        bits.checked_add_signed(steps)
            .filter(|moved| (bits | moved) & T::sign_bit() == 0)
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
/// exactly when `e >= extreme`.
///
/// This pins `within_ulp_bound` to `Quantity::max_steps`: a comparator loosened
/// by one step accepts the displacement at `slack + 1`, so no row fails where
/// at least one must. It does not pin the `max_steps` values, because `slack`
/// follows them; `diffusion_witnesses_attain_the_bound` pins the bound of `D`
/// and the value for `mu_eff` rests on the derivation alone.
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

/// Fixture rows at which `D` lies two steps from its reference, with the
/// computed bits and the signed step count, one above and one below the
/// reference in each format. Their references are verified by
/// `scripts/verify_diffusion_range.py`.
const DIFFUSION_WITNESSES: [(&str, u64, i128); 4] = [
    ("32 31000000 3d2aaaae 40fffffa 37800001", 0x40ff_fffc, 2),
    ("32 405b5879 3400375c 3dc72fba 40bdf578", 0x3dc7_2fb8, -2),
    (
        "64 3c50000000000000 3fa5555555555556 401ffffffffffffe 3e06a09e667f3bcd",
        0x4020_0000_0000_0000,
        2,
    ),
    (
        "64 3ffcd00826e2dae6 40109c6ab5004d58 3facaa9a6dc82591 4016af5f2fe6621d",
        0x3fac_aa9a_6dc8_258f,
        -2,
    ),
];

/// `D` attains its bound in both directions, so the bound is no looser than
/// the corpus requires.
///
/// Each witness of `T`'s width computes the recorded bits at the recorded
/// distance from its reference, and the largest distance toward each
/// direction over every `D` row equals `Quantity::max_steps`. A bound loosened
/// by one step leaves that extreme short of it, and one tightened by one step
/// rejects the witnesses.
fn diffusion_witnesses_attain_the_bound<T: Native>() {
    let rows: Vec<_> = observe::<T>()
        .into_iter()
        .filter(|row| row.quantity == Quantity::Diffusion)
        .collect();
    let mut found = 0;
    for (line, computed, excess) in DIFFUSION_WITNESSES {
        if line.split_whitespace().next() != Some(T::WIDTH) {
            continue;
        }
        let matching: Vec<_> = rows.iter().filter(|row| row.line == line).collect();
        assert_eq!(matching.len(), 1, "witness row is in the fixture: {line}");
        assert_eq!(matching[0].got.bits(), computed, "computed bits: {line}");
        assert_eq!(matching[0].excess(), excess, "distance: {line}");
        found += 1;
    }
    assert_eq!(found, 2, "one witness per direction at width {}", T::WIDTH);
    for direction in [1_i128, -1] {
        let extreme = rows
            .iter()
            .map(|row| direction * row.excess())
            .max()
            .expect("D rows");
        assert_eq!(
            extreme,
            i128::from(Quantity::Diffusion.max_steps()),
            "D rows reach the bound toward {direction}"
        );
    }
}
#[test]
fn diffusion_witnesses_attain_the_bound_for_every_scalar() {
    diffusion_witnesses_attain_the_bound::<f32>();
    diffusion_witnesses_attain_the_bound::<f64>();
    diffusion_witnesses_attain_the_bound::<F32>();
    diffusion_witnesses_attain_the_bound::<F64>();
}

/// A negative zero has no displaced encoding, since encodings move away from
/// zero on the negative side and below it reach the NaN encodings.
fn negative_zero_rows_are_not_displaced<T: Native>() {
    let rows: Vec<_> = observe::<T>()
        .into_iter()
        .filter(|row| row.got.bits() & T::sign_bit() != 0)
        .collect();
    assert!(!rows.is_empty(), "the fixture has signed-zero rows");
    for row in rows {
        assert_eq!(row.got.bits(), T::sign_bit(), "negative zero: {}", row.line);
        assert_eq!(row.want.bits(), T::sign_bit(), "reference: {}", row.line);
        for steps in [-2_i64, -1, 1, 2] {
            assert!(row.displaced(steps).is_none(), "{steps}: {}", row.line);
        }
    }
}
#[test]
fn negative_zero_is_not_displaced_for_every_scalar() {
    negative_zero_rows_are_not_displaced::<f32>();
    negative_zero_rows_are_not_displaced::<f64>();
    negative_zero_rows_are_not_displaced::<F32>();
    negative_zero_rows_are_not_displaced::<F64>();
}
