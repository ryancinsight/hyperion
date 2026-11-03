# ADR 0002: Version the Eunomia 0.12 public scalar boundary

- Status: Proposed
- Date: 2026-11-03
- Scope: Hyperion provider identity and its pre-1.0 version boundary.

## Context

Hyperion exposes Eunomia `RealField` and `UnitScalar` bounds and Aequitas
quantities. Its baseline 0.1.1 locks Eunomia 0.11.0. The coherent current
Aequitas 0.3.2 / Eunomia 0.12.0 / Proteus 0.1.1 graph changes the nominal
identity of those public traits. A generic client compiled with the old
traits cannot call the new `EnergyFluence::attenuate`, even though the Rust
source signature has the same names. A concrete native compile witness
passes against the old pair, fails E0599 against the mixed pair, and passes
against the current pair. A locked Rustdoc comparison passes 223 checks
but does not detect this public dependency identity break.

The separate planar-fluence repair is a numerical correctness fix. Its
ordinary path and public signatures remain unchanged, but delivering it on
current coherent providers cannot be advertised as a compatible 0.1 patch.

## Decision

Advance Hyperion to 0.2.0 as a breaking pre-1.0 dependency boundary, retaining
Git-first `publish = false`. Keep the coherent current provider requirements
and lock; do not back-pin old providers, add dual-trait adapters or loosen
requirements to conceal a mixed cohort. Document the breaking boundary in
CHANGELOG and README. The numerical source and accepted scoped underflow
accuracy contract stay unchanged.

## Consumer migration and ordering

Consumers must adopt Hyperion 0.2 with Eunomia 0.12 and compatible Aequitas
0.3.2 / Proteus 0.1.1 identities. Generic client imports/bounds must refer to
the same Eunomia source as Hyperion. Regenerate each owned lock normally,
then run that consumer's ordinary correctness, public API and delivery gates.
A semver-only pass or one concrete f64 example does not establish this closure.

Existing Helios, Kwavers and CFDrs owners retain their manifests and fixes.
No consumer branch, Atlas pin/overlay or held coordinator work is changed by
this provider repair. Keep delivery as a draft until required provider checks
and independent source review pass. Keep merge held until current consumer
requirements and the protected cohort owner establish a coherent default-
branch adoption order. No new authority over those owners is inferred.

## Acceptance

Retain both original numerical red witnesses, all 740 range comparisons,
136 near-underflow comparisons, typed errors, ordinary-bit identity and
zero-allocation tests. Complete unchanged normal hooks, Rust 1.95, native and
doc checks, supply chain and executable book gates on the final provider
revision. Perform API checks and explicitly retain the old-provider client
compile failure/migrated-client pass as a breaking-change witness. Obtain
independent final-head review and required hosted checks. Only automatic
documentation deployment is authorized; no registry publication or services.
