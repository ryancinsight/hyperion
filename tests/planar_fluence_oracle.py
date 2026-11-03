"""Independent Decimal oracle for native planar-fluence range fixtures."""
from decimal import Decimal, localcontext
from pathlib import Path
import hashlib
import json
import struct

OUT = Path(__file__).with_name('planar_fluence_cases.txt')


def native(bits, width):
    return struct.unpack('>f' if width == 32 else '>d', bits.to_bytes(width // 8, 'big'))[0]


def bits_of(value, width):
    return int.from_bytes(struct.pack('>f' if width == 32 else '>d', value), 'big')


def rounded_bits(value, width):
    """Round the Decimal value directly to nearest-even native, without f64 staging."""
    maximum = 0x7f7fffff if width == 32 else 0x7fefffffffffffff
    lo, hi = 0, maximum
    while lo < hi:
        middle = (lo + hi + 1) // 2
        if Decimal.from_float(native(middle, width)) <= value:
            lo = middle
        else:
            hi = middle - 1
    if lo == maximum:
        return lo
    lower = Decimal.from_float(native(lo, width))
    upper = Decimal.from_float(native(lo + 1, width))
    distance_down, distance_up = value - lower, upper - value
    if distance_down < distance_up or (distance_down == distance_up and lo % 2 == 0):
        return lo
    return lo + 1


rows = []
for width, emin, emax, exponent, maximum in [
    (32, -126, 127, 120, 0x7f7fffff),
    (64, -1022, 1023, 1000, 0x7fefffffffffffff),
]:
    with localcontext() as context:
        context.prec = 240
        minimum_normal = Decimal(2) ** emin
        minimum_subnormal = Decimal.from_float(native(1, width))
        surfaces = [minimum_normal, Decimal(1), Decimal(2) ** 40,
                    Decimal(2) ** exponent, Decimal.from_float(native(maximum, width))]
        for surface in surfaces:
            source_bits = rounded_bits(surface, width)
            source = Decimal.from_float(native(source_bits, width))
            tau_bits = {bits_of(v, width) for v in [0.0, 0.125, 1.0, 8.0, 80.0, 100.0, 128.0, 700.0, 750.0, 1024.0, 1400.0, 1500.0]}
            for boundary, radius in [
                (-minimum_normal.ln(), 4),
                (-(minimum_subnormal / 2).ln(), 4),
                ((source / minimum_normal).ln(), 2),
                ((2 * source / minimum_subnormal).ln(), 2),
            ]:
                if boundary <= 0:
                    continue
                middle = rounded_bits(boundary, width)
                tau_bits.update(range(middle - radius, middle + radius + 1))
            for depth_bits in sorted(tau_bits):
                tau = Decimal.from_float(native(depth_bits, width))
                expected = []
                for precision in [160, 240]:
                    with localcontext() as oracle_context:
                        oracle_context.prec = precision
                        expected.append(rounded_bits(source * (-tau).exp(), width))
                assert expected[0] == expected[1], (width, source_bits, depth_bits)
                rows.append(f'{width} {source_bits:x} {depth_bits:x} {expected[0]:x}')

OUT.write_text('# width incident_bits tau_bits nearest_even_result_bits\n' + '\n'.join(rows) + '\n')

# Independent review supplied these near-half-subnormal inputs. Preserve the
# original corpus above; this separate corpus makes final rounding limits
# explicit and includes neighbors far enough from the tie to require the exact
# zero classification. Its conditional relative roundoff budget is 16 epsilon,
# the same conservative magnitude used by the original normal-output tests.
edge_rows = []
seeds = [(32, 0x33f882b6, 88), (32, 0x50caf2a6, 128),
         (32, 0x67e3bbef, 160), (32, 0x67e3bbf0, 160),
         (64, 0x68f2d16f00849447, 1200), (64, 0x7afb579a32310a87, 1400),
         (64, 0x7fd99bf3916a0bf2, 1454), (64, 0x7fd99bf3916a0bf3, 1454)]
cases = sorted({(width, surface + offset, tau) for width, surface, tau in seeds
                for offset in [-4096, -16, -4, -1, 0, 1, 4, 16, 4096]})
for width, surface_bits, tau in cases:
    with localcontext() as context:
        context.prec = 240
        exact = Decimal.from_float(native(surface_bits, width)) * (-Decimal(tau)).exp()
        epsilon = Decimal(2) ** (-23 if width == 32 else -52)
        error = 16 * epsilon * exact
        expected = rounded_bits(exact, width)
        with localcontext() as check_context:
            check_context.prec = 160
            check = Decimal.from_float(native(surface_bits, width)) * (-Decimal(tau)).exp()
            assert rounded_bits(check, width) == expected
        low = rounded_bits(exact - error, width)
        high = rounded_bits(exact + error, width)
        assert high - low <= 1
        edge_rows.append(f'{width} {surface_bits:x} {bits_of(float(tau), width):x} {expected:x} {low:x} {high:x}')
edge_path = OUT.with_name('planar_fluence_underflow_cases.txt')
edge_path.write_text('# width incident_bits tau_bits nearest_even_bits low_bits high_bits\n' + '\n'.join(edge_rows) + '\n')
receipt = {
    'formula': 'F = F0 * exp(-tau), tau = mu_eff * depth',
    'input_values': 'Exact Decimal representations of native IEEE inputs',
    'rounding': 'Direct positive-bit-order binary search; nearest, ties to even',
    'independent_precisions': [160, 240],
    'rounding_agreement': True,
    'cases': len(rows),
    'final_underflow_cases': len(edge_rows),
    'sha256': hashlib.sha256(OUT.read_bytes()).hexdigest(),
    'boundaries': ['transmission minimum normal', 'transmission rounds to zero',
                   'final fluence minimum normal', 'final fluence rounds to zero'],
}
print(json.dumps(receipt))
