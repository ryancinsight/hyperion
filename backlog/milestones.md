# Milestone ladder ([ADR 0002](../docs/adr/0002-release-scope-1-0.md))
<a id="M-correctness"></a> **M-correctness** (0.1.2) needs: (none) — correctness repairs.
- exit: `HYPERION-DIFFUSION-POSITIVE-RANGE` and `HYPERION-EFFECTIVE-UNDERFLOW` resolved with landed evidence on the default branch; `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets --all-features -- -D warnings`, `cargo nextest run --locked --all-features`, `cargo test --locked --doc --all-features`, and `cargo doc --locked --no-deps --all-features` pass; no open correctness items on M-correctness.
<a id="M-consumer-contract"></a> **M-consumer-contract** (0.2.0) needs: M-correctness — consumer contract coherence.
- exit: constitutive/consumer contract tests (including Proteus contract coverage) pass on `cargo nextest run --locked --all-features`; provider dependency pins/lock are coherent with fetched stack defaults for in-scope providers; no open correctness items.
<a id="M-1.0"></a> **M-1.0** (1.0.0) needs: M-consumer-contract — release-ready (authority-gated).
- exit: README and rustdoc describe the public boundary and examples; `cargo clippy --locked --all-targets --all-features -- -D warnings`, `cargo nextest run --locked --all-features`, `cargo test --locked --doc --all-features`, and `cargo doc --locked --no-deps --all-features` pass; no open correctness items.
