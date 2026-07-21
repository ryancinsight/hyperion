# Hyperion backlog

## HYPERION-001 — Photon and optical interaction Phase 0 [arch] [minor] — in progress

- Owner: Codex `/root`; last-update: 2026-07-21.
- Outcome: one allocation-free, `no_std` interaction-law provider replaces
  duplicated attenuation, optical-depth, reduced-scattering, and diffusion
  formulas in Helios, Kwavers, and CFDrs.
- Scope: typed coefficients and quantities, Beer-Lambert composition, diffusion
  coefficients, NIST mass-attenuation lookup, conformance tests, documentation,
  CI, and direct first-wave consumer migrations.
- Non-goals: material identity, tissue presets, chromophore spectra, CT/HU
  calibration, geometry, solvers, dose deposition, GPU dispatch, Maxwell or
  radiative-transfer solvers, and workflow policy.
- Acceptance: the provider gate passes; consumers delete the named duplicate
  laws without compatibility shims; differentials satisfy derived bounds; the
  public remote default is anonymously fetchable; Atlas registers only that
  verified revision.
