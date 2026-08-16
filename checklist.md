# Hyperion Phase 0 checklist

## ATLAS-HYPERION-AUDIT-074 — Isolated provider re-verification — closed 2026-08-16

- Owner: current Atlas session.
- Scope: `checklist.md` and `gap_audit.md`; no source or consumer changes.
- [x] Record locked isolated-provider gates, the umbrella-overlay lock
      boundary, and the remaining release/consumer triggers with exact
      commands and evidence limits. Revision `1da0da0`; isolated format,
      check, Clippy, 22/22 Nextest, 1 doctest, rustdoc, and cargo-deny pass.

- [x] Verify the GitHub repository name and document the occupied crates.io
      registry name with `publish = false` as the Git-first decision.
- [x] Land the required Aequitas dimensions and align Proteus to one quantity
      source identity.
- [x] Implement the complete coefficient, quantity, transport, reference-table,
      and typed-error contracts from ADR 0001.
- [x] Pass formatting, both feature checks, warning-denied Clippy, nextest,
      doctests, warning-denied rustdoc, the example, and cargo-deny.- [x] Register the Git-first provider and verify hosted CI plus anonymous
      remote-default identity; the occupied crates.io name remains intentionally
      unpublished (`publish = false`). See the Atlas root execution evidence.
- [x] Migrate Helios, Kwavers, and CFDrs directly and delete every superseded law
      named by the Atlas deletion ledger. Consumer details remain in the Atlas
      root board rather than being copied into this provider checklist.
- [x] Complete the recorded consumer SemVer, differential, residue, and full
      publish gates for the first-wave extraction; future release work is a
      separate deferred watchpoint, not a reopened Phase 0 task.
- [x] Replace the self-referential NIST geometric-midpoint identity with a
      native-`T` natural log-log cubic spline and independently queried XCOM
      off-knot regression values.
- [x] Add retrieval-date and table provenance to every embedded NIST source
      record; document that XCOM's fourth displayed digit is not an accuracy
      guarantee and retain that limit in the provider ADR.
- [x] Regenerate the ADR index from the existing `Status: Accepted` header and
      verify the generated index records the canonical status.

## Evidence boundary

The lower-case `checklist.md` is the sole tracked provider checklist. Atlas
root `backlog.md` owns cross-repository hashes, hosted-run identifiers, exact
consumer counts, and registration history; this file records only the local
completion state and points inward to that SSOT to avoid duplicated evidence.

## HYPERION-004 — Proteus constitutive consumer contract

- [ ] Add a generic f32/f64 contract test that evaluates `proteus::ConstantLaw`
      through `proteus::Material` and feeds the returned density into the
      Hyperion attenuation conversion.
- [ ] Assert the analytical unit conversion and retain the existing boundary
      failure tests unchanged.
- [ ] Run the focused provider gates and synchronize `backlog.md` and
      `gap_audit.md` with exact revision and hosted evidence.
