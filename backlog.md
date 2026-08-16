# Hyperion backlog

## HYPERION-001 — Photon and optical interaction Phase 0 [arch] [minor] — done 2026-07-22

- Owner: Codex `/root`; completion evidence is maintained by the Atlas root
  board and the registered provider revision.
- Outcome: one allocation-free, `no_std` interaction-law provider replaces
  duplicated attenuation, optical-depth, reduced-scattering, and diffusion
  formulas in Helios, Kwavers, and CFDrs.
- Scope: typed coefficients and quantities, Beer-Lambert composition, diffusion
  coefficients, NIST mass-attenuation lookup, conformance tests, documentation,
  CI, and direct first-wave consumer migrations.
- Non-goals: material identity, tissue presets, chromophore spectra, CT/HU
  calibration, geometry, solvers, dose deposition, GPU dispatch, Maxwell or
  radiative-transfer solvers, and workflow policy.
- Acceptance: **met**. Hyperion's provider gate is green; the public remote
  default is anonymously fetchable; Atlas records provider `7b4561b` and the
  first-wave deletion evidence for Helios `105a093`, Kwavers `5fc6f0419`, and
  CFDrs merge `69323418`. The root `backlog.md` is the single source for those
  cross-repository test, differential, residue, and registration results; this
  file intentionally does not duplicate their detailed counts.
- Follow-up: registry publication remains intentionally out of scope because
  the occupied crates.io name requires `publish = false`; future consumer or
  provider work must be recorded as a new item rather than reopening this
  completed phase.
- Last update: 2026-08-06.

## HYPERION-002 — Post-registration release residuals [release] [minor] — deferred

- The Git-first provider is registered and consumed through reviewed revisions.
- Any future crates.io or hosted-release work requires a new package identity
  decision and must not introduce a parallel package or compatibility owner.
- No implementation is claimed by this deferred watchpoint.

## HYPERION-003 — NIST interpolation contract [minor] — done 2026-08-14

- Outcome: replace the self-referential geometric-midpoint check with the
  native-`T` natural cubic spline described by the NIST XCOM interpolation
  method and an independent off-knot regression fixture.
- Evidence: ten liquid-water off-knot values were queried from XCOM
  1.5 on 2026-08-14. The f32 and f64 contract test confirms the spline's
  maximum relative residual is below the former log-linear method's residual
  over the independent fixture set.
- Boundary: XCOM states that its fourth displayed digit aids interpolation but
  is not an accuracy claim. The sparse 28-knot provider therefore makes no
  global interpolation-error claim; the fixture is method-regression evidence,
  not a fabricated tolerance.

## HYPERION-004 — Proteus constitutive consumer contract [patch] — done 2026-08-16

- Owner: current Atlas session; scope: Hyperion's transport contract tests and
  provider-local adoption records. Proteus implementation and peer-dirty
  checkout state are non-goals.
- Acceptance: evaluate a real Proteus constitutive law for both supported real
  scalar types, pass its validated density directly into Hyperion's
  mass-to-linear attenuation conversion, and assert the dimensional result.
  The test must fail if the consumer bypasses the constitutive surface or the
  provider-to-consumer unit contract changes.
- Local implementation evidence: `31d3bb3` adds the generic f32/f64 contract;
  locked check, 23/23 Nextest, strict Clippy, doctest, Rustdoc, no-default-
  feature check, examples build, and cargo-deny all pass outside the Atlas
- overlay. Hosted PR verification `31962953235` passes `verify` and
  `supply-chain` at exact PR head `31d3bb3`; CodeRabbit reports pass. The
  merged-default workflow remains the post-merge integration check.

## HYPERION-005 — Proteus lockstep consumer pin [patch] — in-progress

- Owner: current Atlas session; scope: Hyperion's `Cargo.lock` and provider
  dependency-coherence records. No Proteus source or peer-dirty checkout files
  are in scope.
- Acceptance: the locked Proteus source revision equals the fetched Proteus
  default head `cb70021b104743010492c6ec76858eef6177c083`, and Hyperion's
  locked compile, tests, Clippy, documentation, and supply-chain checks pass
  against that graph.
- The dependency update is intentional co-evolution state after the merged
  Proteus provider head; no API compatibility shim or local path dependency is
  permitted.
