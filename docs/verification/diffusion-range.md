# Native diffusion range repair (0.1.1)

Baseline: `4a888e176199065c7f72731742d4b51af6e94a78`.
The public canonical state remains nonnegative finite absorption/reduced
scattering with a finite positive transport sum. D has length dimension;
effective attenuation has reciprocal-length dimension. There is no API,
provider-cohort, publication policy, error-policy or domain-tolerance change.

## Demonstrated failures and preserved policy

For pure absorption s=2^127 (binary32) or 2^1023 (binary64), tripling transport
produces infinity. The old reciprocal then returns Ok(D=0), violating the
positive-only OpticalDiffusionCoefficient constructor. Exact 1/(3s) rounds to
positive subnormal bits 00155555 or 0002aaaaaaaaaaab; those independent reference
bits and both original failing tests remain unchanged.

For pure absorption a=2^-80 or 2^-600, the old 3*a*a radicand underflows to 0.
The true sqrt(3)*a is positive and normal, with independently rounded bits
17ddb3d7 or 1a7bb67ae8584caa. The derived zero makes penetration_depth return
None and predicts full planar transmission at the finite expected attenuation
length, instead of approximately exp(-1). Both original failures remain.

Zero absorption is still valid (including its signed zero), with zero effective
attenuation and None penetration depth. Both-zero transport is degenerate.
Invalid/nonfinite inputs and genuine reciprocal overflow keep typed errors.
The documented nonfinite-radicand rejection is deliberately retained: it was
one of the four original controls, not a third audit finding. No recovery is
claimed for that separate policy. Nonnegative sums/products have no cancellation
in this subsystem; the existing anisotropy endpoint tests remain unchanged.

## Arithmetic argument

For D, retain 1/(3s) when3s is finite. Taking1/s first in this range would
prematurely overflow near the lower admitted inputs. If3s overflows, s is large;
1/s is a positive subnormal, and(1/s)/3 stays far above half the least subnormal
in the provider's binary32/64 formats. Thus the recovered finite D is positive,
without weakening its wrapper. Both operations run in T; no rounded1/3
coefficient, widened scalar or clamp is introduced.

Effective attenuation first executes the existing native radicand and rejects
nonfinite values exactly as before. Only a positive absorption and a subnormal
or zero finite radicand enter the recovery path. The normal threshold is the
exact encoding ONE bitwise-XOR HALF: adjacent odd/even IEEE exponent fields
isolate exponent bit1. HALF is computed exactly as ONE/(ONE+ONE). This removes
the ordinary-path constant scaling call without changing the threshold. The
[full scalar and policy proof](diffusion-threshold.md) covers all supported
scalar implementations and every boundary neighbor, independently of sampling.

Write a=A*2^ea and t=B*2^et using provider-native binary exponent/scale methods.
The normalized A,B are exact in[1,2), even for subnormal source inputs.
Evaluate3*A*B in[3,12). For odd ea+et, multiply that product by2 and use the
floor half exponent, so all root intermediates remain in[3,24). One native
square root and final binary scaling restore the result. The final scaling
alone can round to a subnormal; intermediate product underflow is avoided.
Since t>=a>=the least positive scalar, sqrt(3at)>=sqrt(3) times that scalar;
its rounded result remains positive. Legitimate zero absorption never enters
this branch. Positive sums have no cancellation, and standard relative-error
bounds cover their rounding plus the bounded products/root; the predeclared
corpus acceptance is eight ULP, with zero error-policy or zero-value mismatches.
This is bounded verification, not an exhaustive correct-rounding claim.

## Verification

The original four failures/four controls are retained byte for byte. They first
reproduce against the unchanged baseline. Two added tests cover325 committed
native neighborhood cases and the actual recovered penetration/planar-fluence
consumer. The fixture includes least subnormals, normal/subnormal radicand
transitions, tripled-denominator overflow, reciprocal-overflow midpoints,
separated coefficient scales, signed zero and the documented overflow errors.

Another330 previously unsampled threshold-neighbor cases and two four-scalar
encoding/policy tests cover the constant-construction change.

Run python3 scripts/verify_diffusion_range.py to check all655 fixture references
with exact Fraction arithmetic, integer nearest-even division and square-root
midpoint comparison. An expanded external corpus covers 4,421 input pairs per
O2/O3/default/no-default mode. All four API outputs agree bit for bit; D differs
from exact-input rounding by at most two ULP, effective attenuation by one ULP,
with zero policy mismatches. All 36 native tests pass in debug, release and
no-default modes; all 24 original tests and their assertions remain unchanged.
Format, strict Clippy/Rustdoc, doctests in both modes, all three examples and
an explicit Rust 1.95 all-target/all-feature check pass.

## Cost, resources and release boundary

Representative baseline/candidate timing evidence covers ordinary, weak,
recovered-small, large and mixed input streams; constructor, each repaired
accessor, effective attenuation plus penetration, and a construct/derive
workflow, in direct/opaque-indirect/1024-input streaming contexts. Complete
Results are blackboxed; setup and validation are outside timing. Native
O2/O3 and unchanged overflow checks are used. Across4,200 sampled regions, heap requests/bytes are zero. State sizes remain
8/16 bytes and Result sizes16/32 bytes for binary32/64. Ordinary/weak deltas
span-0.254 to+1.767 ns/call. Mixed construct/derive populations differ by
caller context. Direct and opaque-indirect calls cycle over16 inputs:14 ordinary,
1 small and1 large (87.5%,6.25%,6.25%). Each1024-input streaming pass contains
1002 ordinary,11 small and11 large (97.8515625%,1.07421875%,1.07421875%). The
streaming population alone approximates98%/1%/1%; the combined timing range
cannot be interpreted as an estimate for that population.

The original author medians across native scalars and O2/O3 are:

| Context | Baseline ns/call | Repaired ns/call | Paired delta ns/call |
| --- | --- | --- | --- |
| Direct,16 inputs |3.303–3.558|5.075–5.500|+1.638–2.109|
| Opaque-indirect,16 inputs |2.879–3.554|5.085–5.302|+1.630–2.206|
| Streaming,1024 inputs |3.742–4.402|4.653–4.777|+0.374–0.912|

Ranges collect per-scalar/profile medians; paired deltas are calculated before
aggregation. The heavier recovery weighting in the16-input populations must be
retained when judging their absolute costs. Recovered workflows reach9.265 ns/call.
Wrong baseline recovered results are not throughput targets. Peak stack/RSS,
no_std timing, other platforms and solver-level effects remain unmeasured.
These are author measurements, not performance/resource acceptance or a
universal cost bound.

### Integrated constant-construction comparison

A fresh paired run compares the integrated threshold with the prior correct
scaling implementation, using the same4200 timing regions and exact16/1024
populations. Complete Results, native O2/O3, overflow checks and setup exclusions
remain unchanged. These medians concern the complete construct/derive workflow:

| Context | Prior correct ns/call | Integrated ns/call | Paired delta ns/call |
| --- | --- | --- | --- |
| Direct,16 inputs |5.038–5.284|4.219–4.946|-1.005 to-0.092|
| Opaque-indirect,16 inputs |5.141–5.274|4.302–4.915|-0.843 to-0.359|
| Streaming,1024 inputs |4.661–4.771|3.733–4.574|-0.940 to-0.198|

Affected ordinary/weak attenuation, attenuation-plus-penetration and full
workflow medians improve by0.260–1.501 ns/call. Across all APIs, ordinary/weak
deltas span-1.501 to+0.245 ns/call; the positive endpoint is the unchanged D
accessor. The largest positive delta across all regimes is+0.376 ns/call on an
all-large workflow, where radicand rejection precedes threshold evaluation.
These observations support removing the call, not a universal no-regression
bound or independent performance/resource acceptance.

Generated ARM64 O2/O3 code removes the constant scalbn call from all affected
monomorphs. Local accessor frames shrink64 to32 bytes for binary32 and80 to48
for binary64; full-workflow frames shrink144 to96 and208 to192 bytes. Cold
recovery remains. All measured heap requests/bytes remain zero, and public
state/Result sizes stay unchanged. Local CFI frames exclude total peak stack,
foreign callees and RSS; those remain unmeasured. Other platforms and solver
workloads still require their own assessment.

Only Hyperion's own patch version advances. External Cargo.lock records and
Git provider identities are unchanged. The active dependency PR is not resumed.
The Git-first publish=false decision stays, and the registry name is not a
publication target. No merge, release/tag, registry authentication, publication
or service deployment is authorized by this draft. Proteus's reviewed head and
all protected release/authentication/service holds remain unchanged.
