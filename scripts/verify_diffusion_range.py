"""Check diffusion fixture bits with rational IEEE nearest-even arithmetic.

Run: python3 scripts/verify_diffusion_range.py
Columns: width, absorption bits, reduced-scattering bits, D bits/error,
mu_eff bits/error. Preserve the documented nonfinite-radicand error policy.
No host floating-point arithmetic supplies a reference value.
"""
from fractions import Fraction as F
from pathlib import Path
import math


def spec(width):
    """Precision and minimum/maximum normal binary exponents."""
    return (24, -126, 127) if width == 32 else (53, -1022, 1023)


def exact(bits, width):
    """Interpret finite native bits as a rational; special values return None."""
    precision, minimum, _ = spec(width)
    mantissa_bits = precision - 1
    exponent = (bits >> mantissa_bits) & ((1 << (width - precision)) - 1)
    mantissa = bits & ((1 << mantissa_bits) - 1)
    if exponent == (1 << (width - precision)) - 1:
        return None
    significand = mantissa + (0 if exponent == 0 else 1 << mantissa_bits)
    power = (minimum if exponent == 0 else exponent + minimum - 1) - mantissa_bits
    value = F(significand) * F(2) ** power
    return -value if bits >> (width - 1) else value


def nearest(value, width):
    """Integer quotient/remainder rounding, including the overflow midpoint."""
    precision, minimum, maximum = spec(width)
    sign = 1 << (width - 1) if value < 0 else 0
    value = abs(value)
    if value == 0:
        return sign
    exponent = value.numerator.bit_length() - value.denominator.bit_length()
    if value < F(2) ** exponent:
        exponent -= 1
    quantum = max(exponent - (precision - 1), minimum - (precision - 1))
    scaled = value / F(2) ** quantum
    significand, remainder = divmod(scaled.numerator, scaled.denominator)
    if 2 * remainder > scaled.denominator or (
        2 * remainder == scaled.denominator and significand & 1
    ):
        significand += 1
    if significand == 0:
        return sign
    if significand.bit_length() > precision:
        significand //= 2
        quantum += 1
    result_exponent = quantum + significand.bit_length() - 1
    if result_exponent > maximum:
        return sign + (((1 << (width - precision)) - 1) << (precision - 1))
    if result_exponent < minimum:
        return sign + significand
    return (
        sign
        + ((result_exponent - minimum + 1) << (precision - 1))
        + significand - (1 << (precision - 1))
    )



def sqrt_nearest(value, width):
    """Compare the exact squared value to each integer square-root midpoint."""
    if value == 0:
        return 0
    precision, minimum, _ = spec(width)
    exponent = value.numerator.bit_length() - value.denominator.bit_length()
    if value < F(2) ** exponent:
        exponent -= 1
    quantum = max(exponent // 2 - (precision - 1), minimum - (precision - 1))
    scaled = value / F(2) ** (2 * quantum)
    significand = math.isqrt(scaled.numerator // scaled.denominator)
    comparison = (4 * scaled.numerator
                  - scaled.denominator * (2 * significand + 1) ** 2)
    if comparison > 0 or (comparison == 0 and significand & 1):
        significand += 1
    return nearest(F(significand) * F(2) ** quantum, width)


def reference(width, absorption, scattering):
    a, s = exact(absorption, width), exact(scattering, width)
    if a is None or s is None or a < 0 or s < 0:
        return None
    transport = exact(nearest(a + s, width), width)
    if transport is None or transport == 0:
        return None
    diffusion = nearest(1 / (3 * (a + s)), width)
    d_bits = 'err' if exact(diffusion, width) is None else f'{diffusion:0{width//4}x}'
    three_a = exact(nearest(3 * a, width), width)
    radicand = None if three_a is None else exact(nearest(three_a * transport, width), width)
    if radicand is None:
        e_bits = 'err'
    else:
        effective = sqrt_nearest(3 * a * (a + s), width)
        if a == 0 and absorption >> (width - 1):
            effective = 1 << (width - 1)
        e_bits = f'{effective:0{width//4}x}'
    return f'{width} {absorption:0{width//4}x} {scattering:0{width//4}x} {d_bits} {e_bits}'


if __name__ == '__main__':
    fixture = Path(__file__).resolve().parent.parent / 'tests/diffusion_range_cases.txt'
    lines = fixture.read_text().splitlines()
    for line in lines:
        fields = line.split()
        expected = reference(int(fields[0]), int(fields[1], 16), int(fields[2], 16))
        assert expected == line, (line, expected)
    print(f'{len(lines)} diffusion references match exact rational/integer rounding')
