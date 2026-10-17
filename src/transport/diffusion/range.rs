//! Native evaluation of the finite diffusion range identities.
use eunomia::RealField;

#[inline]
fn three<T: RealField>() -> T {
    T::ONE + T::ONE + T::ONE
}

/// When 3s overflows, 1/s is a positive subnormal and dividing it by3
/// cannot underflow to zero for the provider's binary32/64 formats. Keep
/// 1/(3s) otherwise: taking 1/s first can overflow for small admitted s.
#[inline]
pub(super) fn diffusion<T: RealField>(transport: T) -> T {
    let denominator = three::<T>() * transport;
    if denominator.is_finite() {
        denominator.recip()
    } else {
        large_diffusion(transport)
    }
}

#[cold]
#[inline(never)]
fn large_diffusion<T: RealField>(transport: T) -> T {
    transport.recip() / three::<T>()
}

/// In the provider's IEEE binary encodings, one and one-half have zero
/// fraction/sign fields and adjacent exponent fields: bias and bias-1.
/// The bias is odd, so their XOR isolates the least exponent bit, exactly
/// the minimum normal value. No scaling call or scalar widening is needed.
#[inline]
fn minimum_normal<T: RealField>() -> T {
    T::ONE.bitxor(T::ONE / (T::ONE + T::ONE))
}

/// The caller retains the documented nonfinite-radicand rejection first.
/// Zero absorption keeps its native signed zero. Only positive absorption
/// with a subnormal/zero radicand needs recovery from product underflow.
#[inline]
pub(super) fn effective<T: RealField>(absorption: T, transport: T, radicand: T) -> T {
    if absorption > T::ZERO && radicand < minimum_normal::<T>() {
        small_effective(absorption, transport)
    } else {
        radicand.sqrt()
    }
}

#[cold]
#[inline(never)]
fn small_effective<T: RealField>(absorption: T, transport: T) -> T {
    let a_exponent = absorption
        .binary_exponent()
        .expect("positive finite absorption");
    let t_exponent = transport
        .binary_exponent()
        .expect("positive finite transport");
    let a = absorption.scale_binary(-a_exponent);
    let t = transport.scale_binary(-t_exponent);
    let exponent = a_exponent + t_exponent;
    // a,t are exact in [1,2). The product is in [3,12); an odd exponent
    // contributes another2, giving [6,24). All root intermediates are normal.
    let mut product = three::<T>() * a * t;
    if exponent & 1 != 0 {
        product *= T::ONE + T::ONE;
    }
    // Euclidean division also handles odd negative exponents correctly.
    product.sqrt().scale_binary(exponent.div_euclid(2))
}

#[cfg(test)]
mod tests;
