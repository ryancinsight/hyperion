# Exact normal-radicand threshold (0.1.1)

The private threshold expression changes from scaling ONE by the minimum-normal
exponent to ONE bitwise-XOR (ONE/(ONE+ONE)). This replaces constant construction
only. Constructor validation, the finite-radicand guard, positive-absorption
comparison, subnormal recovery and all result arithmetic remain unchanged.

## Complete supported scalar argument

The locked Eunomia0.10 provider implements RealField for exactly f32,f64,F32,F64.
FloatElement has a private sealed supertrait; downstream code cannot introduce
a new scalar implementation. UnitScalar covers additional formats, but those
formats do not implement RealField and cannot instantiate this algorithm.
F32/F64 are transparent native wrappers. The provider implements bitxor by XOR
of the underlying encoding, and wrapper arithmetic/bitops forward to native
operations. The proof applies to the entire current RealField cohort, not just
the finite values selected by an oracle. A future provider cohort must recheck
these encoding/operation assumptions before accepting the optimization.

Let k be the IEEE exponent-field width, p the precision and b=2^(k-1)-1 the
exponent bias. ONE has sign0, exponent b and fraction0; HALF has sign0,
exponent b-1 and fraction0. Since b is odd, b XOR(b-1)=1. ONE XOR HALF therefore
has exponent field1, sign0 and fraction0: the minimum positive normal value,
2^(1-b). ONE+ONE=2 and ONE/2=HALF are exact representable native operations.
The old expression uses infinity's k exponent bits and exponent2-2^(k-1)=1-b,
so it returns the identical encoding. The native encodings are00800000 and
0010000000000000; the wrapper encodings are identical. No scalar widening,
width-dependent branch, tolerance, clamp or changed input domain is introduced.

This is an identity between constants. For every possible radicand encoding,
including all representable neighbors, the comparison r<threshold has exactly
the same result. In particular, the greatest subnormal is below, the minimum
normal is equal and its successor is above; equality remains on the native
square-root path. The proof does not rely on sampling or a distance from the
boundary. It also preserves comparisons for signed zero, infinities and NaNs.

## Public zero and nonfinite policy

Finite nonnegative coefficient validation and finite positive transport-sum
validation precede the accessor. Negative, infinite and NaN coefficient inputs
retain their typed input errors and payloads. Both-zero transport stays
DegenerateTransport; overflowing transport stays a typed derived error.
The accessor computes the original native radicand and rejects every nonfinite
radicand before threshold evaluation. This policy includes a radicand overflow
whose mathematically rescaled final square root could otherwise be finite.

Positive absorption with finite r below MIN_NORMAL uses the existing recovery;
positive absorption with r>=MIN_NORMAL uses the existing sqrt. For +0/-0
absorption, the positive comparison is false, so native signed-zero sqrt and
None penetration remain. No exceptional input can become newly admitted by
changing an identical constant. Every later arithmetic expression, finite
check, typed wrapper and downstream penetration/fluence operation is unchanged.
This proves branch/value/error equivalence; it does not strengthen the existing
bounded-ULP algorithm into a correctly-rounded claim.

## Boundary and policy regression evidence

The two new unit tests check exact threshold encodings, immediate neighbors,
signed zeros, infinities and NaN comparisons for all four scalar implementations.
They also exercise public invalid coefficient errors/payloads, signed-zero
attenuation/None penetration and the nonfinite-radicand error for all four.
They pass against the original scaling implementation before the change.

An exact rational simulation locates the first representable scattering value
whose native radicand reaches MIN_NORMAL, using five absorption scales for each
native width. Sixteen consecutive scattering neighbors on each side supply
330 new fixture rows, all absent from the earlier24128-pair oracle. Each group
contains16 subnormal,2 minimum-normal and15 greater-normal radicands. Absorption
covers subnormal and normal values with widely separated coefficient scales.
The existing public range test verifies both this fixture and all325 original
rows. The rational checker verifies all655 expected rows without host floating
point. Existing8ULP and zero policy/zero-value mismatch limits are unchanged.

Generated code and paired timing assess the removal of constant libm::scalbn
calls separately from numerical equivalence. Local frame/heap observations are
not a total peak-stack/RSS bound or performance/resource acceptance. The corrected
16-input versus1024-input workload populations remain authoritative. Git-first
publish=false, owner PR82 and every existing release/authentication/service hold
remain unchanged; this proof does not authorize merge or publication.
