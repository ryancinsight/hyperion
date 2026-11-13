# Hyperion Phase 0 checklist

## HYPERION-007 — Anchor chromophore source oracle — closed 2026-08-20

- [x] Read the OMLC source rows independently and record the URL, retrieval
      date, table columns, units, and tetramer normalization.
- [x] Replace the retired Kwavers-only oracle description with the independent
      source-knot fixture and retain value-semantic mutation coverage.
- [x] Synchronize the API Rustdoc, provider chromophore guide, book chapter,
      and ADR 0001 revision.
- [x] Run exact-lane format, locked checks, Clippy, Nextest, doctests, and
      Rustdoc for all features and no default features; record the unavailable
      local cargo-deny command without claiming supply-chain coverage.

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
      doctests, warning-denied rustdoc, the example, and cargo-deny.
- [x] Register the Git-first provider and verify hosted CI plus anonymous
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
- [x] Re-verify the merged default at provider head `a33c2f7` on 2026-08-19:
      no-default-feature and all-feature checks, warning-denied Clippy,
      23/23 Nextest, doctest, rustdoc, examples, and all mdBook chapters pass.
      The book test uses the committed dependency directory after the exact
      Hyperion, Aequitas, and Proteus package artifacts are rebuilt.

## Evidence boundary

The lower-case `checklist.md` is the sole tracked provider checklist. Atlas
root `backlog.md` owns cross-repository hashes, hosted-run identifiers, exact
consumer counts, and registration history; this file records only the local
completion state and points inward to that SSOT to avoid duplicated evidence.

## HYPERION-004 — Proteus constitutive consumer contract

- [x] Add a generic f32/f64 contract test that evaluates `proteus::ConstantLaw`
      through `proteus::Material` and feeds the returned density into the
      Hyperion attenuation conversion.
- [x] Assert the analytical unit conversion and retain the existing boundary
      failure tests unchanged.
- [x] Run the focused provider gates and synchronize `backlog.md` and
      `gap_audit.md` with exact revision and hosted evidence.

## HYPERION-005 — Proteus lockstep consumer pin

- [x] Advance the locked Proteus source revision to the fetched provider
      default without changing the manifest's Git dependency boundary.
- [x] Run the locked provider gates and verify the lock contains the exact
      Proteus head.
- [x] Synchronize provider evidence and integrate the merged default head into
      Atlas.

## HYPERION-006 — Provider lock closure refresh

- [x] Refresh `Cargo.lock` to Aequitas `260ad10`, Eunomia `85e590b`, and
      Proteus `f612c99` without changing the Git dependency boundary.
- [x] Run `cargo fmt --all -- --check` and locked all-feature dependency
      metadata against the refreshed graph.
- [x] Push the provider branch and collect hosted `verify` and `supply-chain`
      at exact head `880eb8cce28d1e887942fbeb185a1cf4173c776a`; PR #15 merged
      at provider default `0156f59f78aba1e3b06d4511ffb1ce30d5c0c6d4`, and Atlas
      records that merge commit.
