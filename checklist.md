# Hyperion Phase 0 checklist

- [x] Verify the GitHub repository name and document the occupied crates.io
      registry name with `publish = false` as the Git-first decision.
- [x] Land the required Aequitas dimensions and align Proteus to one quantity
      source identity.
- [x] Implement the complete coefficient, quantity, transport, reference-table,
      and typed-error contracts from ADR 0001.
- [x] Pass formatting, both feature checks, warning-denied Clippy, nextest,
      doctests, warning-denied rustdoc, the example, and cargo-deny.
- [ ] Publish the provider and verify hosted CI plus anonymous remote-default
      identity.
- [ ] Migrate Helios, Kwavers, and CFDrs directly and delete every superseded
      law named by the Atlas deletion ledger.
- [ ] Complete consumer SemVer, differential, residue, and full publish gates.
