# Hyperion ownership gap audit

## HYPERION-004 — Proteus constitutive consumer contract — closed 2026-08-16

Hyperion now exercises the direct provider path rather than only constructing
Proteus's `MassDensity` wrapper. The generic contract test evaluates a real
`proteus::Material<proteus::ConstantLaw<T>>` for `f32` and `f64`, obtains its
validated density through `Material::properties`, and passes that value into
Hyperion's mass-attenuation conversion. The analytical oracle is the declared
SI conversion `0.07072 cm^2/g * 1000 kg/m^3 = 7.072 m^-1`; the assertion is
value-semantic and independent of the test's construction path.

Local evidence at `31d3bb3`:

- `cargo check --locked --workspace --all-targets --all-features`: pass.
- `cargo nextest run --locked --workspace --all-features`: 23/23 pass.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`:
  pass.
- `cargo test --locked --workspace --all-features --doc`: 1/1 pass.
- `RUSTDOCFLAGS=-D warnings cargo doc --locked --workspace --all-features
  --no-deps`: pass.
- `cargo check --locked --no-default-features`: pass; examples build: pass.
- `cargo deny ... --metadata-path ... --locked check`: advisories, bans,
  licenses, and sources pass.

Hosted PR verification `31962953235` passes repository-owned `verify` and
`supply-chain` at exact head `31d3bb3`; CodeRabbit reports pass. The merged-
default workflow remains the post-merge integration check. The Atlas root
record owns the cross-repository gitlink and hosted-run identifiers.

## HYPERION-005 — Proteus lockstep consumer pin — closed 2026-08-16

Hyperion's locked graph now follows the fetched Proteus default. Commit
`7f36069` changes only `Cargo.lock`: Proteus advances from `3d6021e7` to
`cb70021b`, while its locked Aequitas and Eunomia provider revisions advance to
`5114cd1` and `88c685f`, respectively. The manifest remains Git-sourced with
the declared semver requirements; no path dependency or compatibility layer
was introduced.

Final local evidence against that lock:

- locked workspace check with all targets/features: pass;
- locked Nextest: 23/23 pass;
- strict all-target Clippy: pass;
- locked no-default-features check: pass;
- locked doctest: 1/1 pass;
- warning-denied Rustdoc: pass;
- locked examples build: pass;
- cargo-deny advisories, bans, licenses, and sources: pass.

The provider default and Pages workflow are the remaining integration boundary
for this dependency-only change; the Atlas root owns the resulting gitlink.

## ATLAS-HYPERION-AUDIT-074 — Isolated provider re-verification — closed 2026-08-16

The provider default `1da0da0` was re-verified from outside the Atlas umbrella
directory so the parent `.cargo/config.toml` development overlay could not
rewrite the standalone lockfile. The exact locked commands were:

- `cargo fmt --check --manifest-path D:\atlas\worktrees\hyperion-audit-20260816\Cargo.toml`
- `cargo check --locked --all-features --all-targets --manifest-path D:\atlas\worktrees\hyperion-audit-20260816\Cargo.toml`
- `cargo clippy --locked --all-targets --all-features --manifest-path D:\atlas\worktrees\hyperion-audit-20260816\Cargo.toml -- -D warnings`
- `cargo nextest run --locked --all-features --manifest-path D:\atlas\worktrees\hyperion-audit-20260816\Cargo.toml` — 22/22 passed
- `cargo test --locked --doc --all-features --manifest-path D:\atlas\worktrees\hyperion-audit-20260816\Cargo.toml` — 1 passed
- `cargo doc --locked --no-deps --all-features --manifest-path D:\atlas\worktrees\hyperion-audit-20260816\Cargo.toml`
- `cargo deny check` from the provider lane — advisories, bans, licenses, and
      sources passed; three expected unmatched-source warnings identify the
      local overlay paths for Aequitas, Eunomia, and Proteus.

The provider source and standalone lockfile remain unchanged after
verification. A locked Cargo command invoked from inside `D:\atlas` is a
separate environment: local first-party patches are not represented in the
standalone lockfile, so Cargo rejects the command before compilation and asks
to rewrite `Cargo.lock`. That is an umbrella verification/configuration
boundary, not a reason to commit overlay-generated lockfile churn.

These checks establish formatting, compilation, static diagnostics, supply
chain policy, test, doctest, and documentation behavior only. They make no new
runtime, performance, memory, or hardware-backend claim. No source-level
placeholder markers were found. Remaining release and consumer watchpoints
retain their documented status below.

## Boundary

Hyperion owns validated photon and optical interaction coefficients: typed
coefficients and quantities, Beer-Lambert composition, diffusion/deposition
coefficients, chromophore spectra, and NIST mass-attenuation lookup. It does
not own material identity, tissue presets, CT/HU calibration, geometry,
solvers, dose deposition, GPU dispatch, or Maxwell/radiative-transfer solvers
(non-goals of HYPERION-001).

## Provider verification (2026-08-14)

- Strict all-target check with `-D warnings`: pass.
- Clippy `-D warnings` (all-targets/all-features): pass.
- Nextest (all-features): 22/22 pass, 0 skipped.
- Doctests: pass.
- `--no-default-features` check: pass.
- NIST off-knot contract: ten independent liquid-water XCOM 1.5 queries;
  f32/f64 spline residual maximum is below the former log-linear residual
  maximum. XCOM's published fourth-digit limitation is recorded; no global
  error bound is claimed from the sparse knot set.
- ADR index: the generated index records ADR 0001's existing canonical
  `Accepted` status.
- Local planning trail completed at this audit with the missing `gap_audit.md`
  authored; `backlog.md` and `checklist.md` predate this audit.

## Gap inventory

| ID | Description | Status |
|----|-------------|--------|
| HYPERION-002 | Post-registration release residuals (crates.io identity decision) | Deferred — watchpoint only; no implementation claimed |
| H-001 | Registry publication (`publish = false`, occupied crates.io name) | Open — owner-gated Git-first decision documented in `backlog.md` |
| H-002 | Material identity / tissue presets / chromophore spectra beyond the coefficient layer | Open — consumer-gated; explicitly out of HYPERION-001 scope |
| H-003 | A global interpolation-error theorem for the sparse 28-knot table | Closed as a false precision requirement — NIST XCOM documents the interpolation family and explicitly disclaims accuracy for the fourth displayed digit; independent off-knot method regression is the supported evidence |
| H-005 | Stale ADR index status for ADR 0001 | Closed — regenerated `docs/adr/README.md` from the existing `Status: Accepted` header |

No source-level gaps remain in the delivered surface: no `TODO`/`FIXME`/
`unimplemented!` markers exist in `src/`, and all gates are green at the
audited revision. First-wave consumer migrations (Helios `105a093`, Kwavers
`5fc6f0419`, CFDrs merge `69323418`) are recorded at the Atlas root SSOT.
