//! Planar-consumer error bounds for the recovered effective attenuation.
use super::{Native, pair};
use aequitas::systems::si::quantities::{EnergyPerArea, Length};
use eunomia::{F32, F64};
use hyperion::{
    quantity::{EnergyFluence, PathLength},
    transport::planar_fluence_at_depth,
};

/// Higham's `gamma_n = n u / (1 - n u)` for a product of `n` factors.
///
/// `u = EPSILON / 2` is the unit roundoff of round-to-nearest. A product or
/// quotient of `n` factors `(1 + d)^(+-1)` with `|d| <= u` is `1 + theta` with
/// `|theta| <= gamma_n` (Higham, Accuracy and Stability of Numerical
/// Algorithms, 2nd ed., Lemma 3.1). For the small counts used here `n u` is
/// `n` scaled by a power of two and `1 - n u = (2^p - n) / 2^p` has `p`
/// significant bits, so both are exact in `T`; the quotient rounds once, and
/// the product with the expected length at the assertion site rounds once
/// more.
fn gamma<T: Native>(operations: usize) -> T {
    let n_u = T::from_count(operations) * (T::EPSILON / (T::ONE + T::ONE));
    n_u / (T::ONE - n_u)
}
/// One unit in the last place of the binade `[1/4, 1/2)`, `EPSILON / 4`.
///
/// `exp(-1) = 0.3679` and `exp(-y)` for `|y - 1| <= 2 gamma_7` (below `1e-6`)
/// all lie in that binade.
fn exp_binade_ulp<T: Native>() -> T {
    T::EPSILON / T::from_count(4)
}
/// `|exp(-y) - e^-y|` for the scalar's `exp`, with `e^-y` evaluated beyond `T`.
///
/// With `d = y - 1`, exact in `T` for `y` in `[1/2, 2]` (Sterbenz), the
/// reference is `e^-y = (hi + lo)(1 + s)` for `s = e^-d - 1 = d^2 / 2 - d + r`
/// and `|r| <= |d|^3 / 6 (1 + |d|)`. For `|d| < 1e-6` the dropped `|r| e^-1`
/// is below `1e-11` units in the last place in binary32 and far below in
/// binary64. `got - hi` is exact (two values of one binade), and the seven
/// roundings of the remaining evaluation (`d * d`, `- d`, `hi * s`, its
/// subtraction, `- lo`, `lo * s`, its subtraction; the halving is exact) are
/// each at most `u` times a magnitude below `1e-6`, which is `2e-6` units, so
/// below `1.4e-5` units in total, which `exp_error_over_planar_interval`
/// charges as `ulp / 2^15`.
fn exp_error<T: Native>(y: T) -> T {
    let (hi, lo) = T::exp_neg_one_split();
    let two = T::ONE + T::ONE;
    let d = y - T::ONE;
    let s = d * d / two - d;
    let got = (-y).exp();
    (got - hi - hi * s - lo - lo * s).abs()
}
/// Largest [`exp_error`] over every value `y` of `T` with `|y - 1| <= 2
/// gamma_7`.
///
/// `|y - 1| <= gamma_7` is the domain of the planar fluence exponent (see
/// [`planar_fluence_tolerance`]). Twice that radius covers it with room for
/// the one rounding in the computed `gamma_7`. The sweep visits each encoding
/// of `T` in the interval, so the maximum is exact for the domain rather than
/// sampled.
fn worst_exp_error<T: Native>() -> T {
    let one = T::ONE;
    let radius = gamma::<T>(7) * (one + one);
    let below = (1..)
        .map(|step| T::from_raw_bits(one.bits() - step))
        .take_while(|y| one - *y <= radius);
    let above = (0..)
        .map(|step| T::from_raw_bits(one.bits() + step))
        .take_while(|y| *y - one <= radius);
    below
        .chain(above)
        .map(|y| {
            let error = exp_error(y);
            assert!(error.is_finite(), "exp error is finite over the interval");
            error
        })
        .fold(
            T::ZERO,
            |worst, error| if error > worst { error } else { worst },
        )
}
/// Bound on `|fluence - want|` for the planar consumer at one penetration
/// depth `1 / mu_eff` of unit incident fluence, with `want = fl(exp(-1))`.
///
/// The exponent is `x = fl(mu_eff r)` for the depth `r = fl(1 / expected)`.
/// `mu_eff` carries 3 rounding factors and `expected` 2 (the count in
/// [`downstream`]), `r` one and the product one, so `x = 1 + theta` with
/// `|theta| <= gamma_7` (Higham, Accuracy and Stability of Numerical
/// Algorithms, 2nd ed., Lemma 3.1). The product of attenuation and depth is a
/// single multiplication (aequitas `aa9e89f`,
/// `src/quantity/arithmetic/multiplicative.rs:8-19`) and the unit incident
/// fluence multiplies the transmission exactly. With `E` the error of the
/// scalar's `exp` on this interval,
///
/// `|fl(exp(-x)) - want| <= E + e^-1 (e^gamma_7 - 1) + E`
///
/// where the middle term is the conditioning of `exp` at 1, the derivative
/// `-e^-1` times the displacement, with `e^gamma - 1 <= gamma (1 + gamma)`;
/// the outer terms are the `exp` errors at `-x` and at `-1`.
///
/// `E` is one unit in the last place of the binade of `e^-1`
/// ([`exp_binade_ulp`]). libm 0.2.16 `exp` states "less than 1 ulp"
/// (`src/math/exp.rs:59-60`); its `expf` states only the minimax bound of its
/// polynomial (`src/math/expf.rs:23-24`) and no end-to-end ulp bound, so
/// `exp_error_over_planar_interval` measures both over every encoding the
/// exponent can take (eunomia `acb3f3a7` routes `f32` through `expf` and `f64`
/// through `exp`). It measured maxima of `0.75` unit in binary32 and `0.66` in
/// binary64 and enforces the margin the bound needs: the evaluated expression
/// falls short of the exact bound by [`tolerance_shortfall`], which the two
/// `E` terms must absorb.
///
/// The reference value is the analytical Beer-Lambert decay
/// `F(z) = F_0 exp(-mu_eff z)` (`docs/book/beer_lambert.md:22-23`), which at
/// `z = 1 / mu_eff` is `F_0 / e` (`docs/adr/0001-photon-optical-interaction-boundary.md:146`).
fn planar_fluence_tolerance<T: Native>(want: T) -> T {
    want * gamma::<T>(7) + exp_binade_ulp::<T>() * (T::ONE + T::ONE)
}
/// Amount by which [`planar_fluence_tolerance`] can fall below its exact
/// expression `e^-1 (e^gamma_7 - 1) + 2 E`, at most `6 EPSILON^2`.
///
/// With `gamma_7 <= 3.5 EPSILON (1 + 4 EPSILON)` and `u = EPSILON / 2`, in
/// units of `EPSILON^2`: `want` differs from `e^-1` by at most one unit in the
/// last place `EPSILON / 4`, giving `0.875`; the dropped `e^-1 gamma_7^2 / 2`
/// gives `2.26`; rounding the quotient in `gamma`, the product with `want` and
/// the sum with the `E` terms each cost `u` of their value, `0.644`, `0.644`
/// and `0.9`. The total `5.32` is below `6`.
fn tolerance_shortfall<T: Native>() -> T {
    T::from_count(6) * T::EPSILON * T::EPSILON
}
/// Feed `a` through `effective_attenuation`, the reciprocal path and the real
/// planar consumer, returning whether the computed attenuation differs from
/// `expected = fl(fl(sqrt(3)) a)`.
fn downstream<T: Native>(a: T) -> bool {
    let state = pair(a, T::ZERO);
    let expected = (T::ONE + T::ONE + T::ONE).sqrt() * a;
    let attenuation = state
        .effective_attenuation()
        .expect("finite positive effective attenuation");
    let differs = *attenuation.quantity().as_base() != expected;
    let path = attenuation
        .penetration_depth()
        .expect("finite penetration")
        .expect("positive attenuation has finite penetration");
    // Factor form for Lemma 3.1, `(1 + d)^(+-1)` with `|d| <= u`. The radicand
    // `3 a (a + 0)` rounds three times (sum `d1`, tripling `d2`, product
    // `d3`) and the root once (`d4`). The root halves the radicand factors:
    // `((1 + d1)(1 + d2))^(1/2)` and `(1 + d3)^(1/2)` each lie in
    // `[1 - u, 1 + u]`, so `mu_eff` carries 3 factors with its own rounding.
    // `path` is its reciprocal, rounded once (1 more). `expected` rounds the
    // root and the product (2), and `expected.recip()` once more (1). The
    // ratio `path / expected.recip()` is a product of 3 + 1 + 2 + 1 = 7
    // factors, so it is `1 + theta` with `|theta| <= gamma_7` and
    // `|path - expected.recip()| <= gamma_7 expected.recip()`. Evaluating the
    // tolerance in `T` rounds the quotient inside `gamma` and the final
    // product, lowering it by about `2 u` relative (`14 u^2` of `E` for
    // `gamma_7`); `gamma_7` exceeds the worst 7-factor error `(1 - u)^-7 - 1`
    // by `21 u^2` relative, which absorbs that.
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
    assert!((got - want).abs() <= planar_fluence_tolerance(want));
    differs
}
/// A normal-range input whose radicand does not underflow, so the computed
/// attenuation follows the square-root path rather than the exact-scaling
/// recovery. For `5` the root of the rounded radicand differs from
/// `fl(fl(sqrt(3)) a)` in all four formats (checked in the assertions below).
fn five<T: Native>() -> T {
    T::from_count(5)
}
#[test]
fn recovered_attenuation_feeds_real_planar_consumer() {
    // The subnormal-radicand inputs recover `fl(sqrt(3)) a` exactly, so the
    // path assertion sees a zero difference there.
    assert!(!downstream(2.0_f32.powi(-80)));
    assert!(!downstream(2.0_f64.powi(-600)));
    assert!(!downstream(F32(2.0_f32.powi(-80))));
    assert!(!downstream(F64(2.0_f64.powi(-600))));
    // On the normal path the computed attenuation differs from `expected`,
    // so the `gamma_7` assertion bounds a nonzero difference.
    assert!(downstream(five::<f32>()));
    assert!(downstream(five::<f64>()));
    assert!(downstream(five::<F32>()));
    assert!(downstream(five::<F64>()));
}
#[test]
fn exp_error_over_planar_interval() {
    fn within_one_unit<T: Native>() {
        let worst = worst_exp_error::<T>();
        let ulp = exp_binade_ulp::<T>();
        // `exp_error` evaluates below `2e-5` unit (below `ulp / 2^15`), and
        // the tolerance needs half of its shortfall from each `E` term.
        let measurement = ulp / T::from_count(1 << 15);
        let half_shortfall = tolerance_shortfall::<T>() / (T::ONE + T::ONE);
        assert!(
            worst > T::ZERO,
            "the sweep measures a nonzero rounding error"
        );
        assert!(
            worst + measurement + half_shortfall <= ulp,
            "exp error leaves no margin under one unit"
        );
    }
    within_one_unit::<f32>();
    within_one_unit::<f64>();
    within_one_unit::<F32>();
    within_one_unit::<F64>();
}
