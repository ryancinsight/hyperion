# Planar fluence across the native attenuation range

The public contract is `F = F0 exp(-tau)`, with `tau = mu_eff depth` and
finite nonnegative inputs. Computing the transmission first can round it to
zero or a low-precision subnormal even when the combined fluence is normal.
At baseline `f899cd56abc6460142c3448ad377004f7c193944`, both public-API tests
returned zero: binary32 `F0 = 2^120, tau = 128` should give approximately
`3.4190528e-20`, and binary64 `F0 = 2^1000, tau = 1024` should give
`2.053270630029784e-144`. Native execution established both failures before
the implementation changed; no dependency update was needed to reproduce.

## Native range identity

Normal transmission keeps the original multiplication. For positive incident
fluence and subnormal or zero transmission, evaluate `c = exp(-tau/4)` and
`F = (((F0 c) c) c) c` in the caller's precision. The exponential identity
is exact over the reals. Division by four is exact for the relevant native
optical depths and avoids subtracting a large `log(F0)` from `tau`.

For a result that can round nonzero, `tau` cannot exceed
`ln(max_finite / (min_subnormal/2))`. In binary32 and binary64, one quarter
of this upper bound is below `-ln(min_normal)`, so `c` is normal. Sequential
products decrease from the finite incident value; they cannot overflow, and
their exact magnitudes are at least the final magnitude. The final product
may still legitimately round to zero. This range argument is not a claim
that the underlying exponential or the full product is correctly rounded.

The threshold uses the existing four-scalar minimum-normal encoding identity,
moved unchanged to the shared internal validation module. Its existing
diffusion threshold tests and proofs remain in place. The transmission API,
explicit `EnergyFluence::attenuate` operation, signed-zero behavior, and typed
optical-depth overflow rejection retain their contracts.

## Independent reference and coverage

`python3 tests/planar_fluence_oracle.py` regenerates the 370 fixture rows.
Inputs are exact native bit patterns. Python Decimal evaluates the defining
equation at 160 and 240 digits; a direct bit-order search rounds each result
to nearest-even binary32/64 without an intermediate binary64 rounding for
binary32. Both reference precisions must agree on every resulting bit pattern.

Each precision has 185 cases covering transmission normal/subnormal and
zero-rounding transitions, final-fluence normal/subnormal and zero-rounding
transitions, and incident values from the minimum normal through maximum
finite. Native types and their F32/F64 wrappers execute all 740 comparisons.
Within this corpus, positive reference results must stay positive and true
underflows must be zero; attenuation must not amplify incident fluence. The measured error requirement
is 16 ULPs, accounting for repeated exponential error and product rounding;
it is not asserted as a universal accuracy theorem for every platform's exp.
Separate tests preserve signed zeros, ordinary native values, and the exact
typed overflow error. The two original failing cases remain explicit tests.

Independent review also constructed final products immediately around half
the minimum subnormal. Even a correctly rounded quarter exponential followed
by native multiplications can cross this final rounding tie. Native evaluation
confirmed all eight supplied witnesses: for example, f32 incident bits
`0x67e3bbef` at tau 160 and f64 incident bits `0x68f2d16f00849447` at tau 1200
return zero where the full equation rounds to minimum-subnormal. Reverse
classification differences also occur. Correctly rounded final underflow is
therefore explicitly not promised by this range repair.

A separate 68-row corpus retains those cases and neighboring controls, with
136 comparisons across the four representations. It requires at most one
minimum-subnormal of absolute error against the independently rounded result.
It also rounds an independent `exact * (1 +/- 16 epsilon)` interval: results
must lie inside it, and controls away from the tie require exact zero/nonzero
classification. Conditionally, if the normal quarter exponential has relative
error at most epsilon, its four uses and three normal intermediate products
contribute at most seven factors bounded by epsilon. The conservative bound
`gamma_8 = 8 epsilon / (1 - 8 epsilon) < 16 epsilon` holds for both precisions.
The interval endpoints then include the final rounding to subnormal or zero.
This conditional budget is verified on the current implementation; it is not
asserted as a platform-independent exp accuracy theorem. All original 740
comparisons and their strict zero/positive assertions remain unchanged.

## Bounded performance observation

A local ARM64 Rust 1.97/O3 comparison compiles the original public expression
and repaired API against identical current providers. Nine alternating pairs
each evaluate 1,024 inputs 256 times after warmup; setup is outside timing.
Ordinary inputs use incident fluence 10–17 and optical depth 0.125–8. Recovery
inputs use the original failing scales, with eight nearby depths. Mixed input
has one recovery case among 1,024. Median nanoseconds per public operation:

| Scalar / population | Original expression | Repaired |
| --- | ---: | ---: |
| f32 ordinary | 3.006 | 3.431 |
| f32 recovery | 2.189 | 5.298 |
| f32 mixed | 3.047 | 3.451 |
| f64 ordinary | 4.156 | 4.297 |
| f64 recovery | 2.273 | 6.191 |
| f64 mixed | 4.128 | 4.289 |

The ordinary-path check costs approximately 0.43 ns for f32 and 0.14 ns for
f64 in this instrument. The recovery path performs the additional exponential
and multiplies needed to retain the result; the faster original zero outputs
are incorrect. No general throughput or whole-application claim follows.
The committed thread-local allocation test also checks both recovery paths
against their reference values and requires zero allocation delta.

## Delivery boundary

This repair preserves the Git-first `publish = false` identity. Provider
compatibility, complete normal gates, performance observations, independent
final-revision review, hosted checks and documentation deployment remain
required before a delivery is accepted. A successful focused run alone does
not establish those results or qualify downstream consumers.
