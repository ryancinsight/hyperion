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
