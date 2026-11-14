# Hyperion backlog

<a id="HYPERION-PLANAR-FLUENCE-TOLERANCE"></a>
## HYPERION-PLANAR-FLUENCE-TOLERANCE — Derive the planar fluence tolerance [patch] — todo
- priority: verification; milestone: M-correctness; needs: none; basis: 61968b1c5f26.
- scope: `tests/diffusion_range_boundaries.rs`.
- Outcome: the fluence assertion of `recovered_attenuation_feeds_real_planar_consumer`
  uses `32 * EPSILON` with no derivation at the assertion site; the path assertion
  derives its argument error (`gamma_7`), the `exp` term is not derived.
- Gap: libm 0.2.16 `exp` states "less than 1 ulp" (`src/math/exp.rs:59-60`);
  `expf` states no bound, and F32 routes through it (eunomia acb3f3a7,
  `traits/float.rs:195-196`).
- Oracle: the tolerance is a named expression of `gamma_7` and the per-scalar
  `exp` error, each term cited; the `expf` term rests on a measured maximum-ulp
  sweep over `[-1 - gamma_7, -1 + gamma_7]` or an eunomia accuracy contract; a
  mutant moving the fluence by twice the bound fails.
- Next step: sweep `expf` against exact references over that interval.

<a id="HYPERION-DIFFUSION-RANGE-RESOURCE"></a>
## HYPERION-DIFFUSION-RANGE-RESOURCE — Accept diffusion range resource cost [patch] — todo
- priority: verification; milestone: M-correctness; needs: none; basis: 8a1f7e3c8af2.
- scope: `src/transport/diffusion/range.rs`, `docs/verification/diffusion-range.md`.
- Outcome: resource acceptance of the repairs in [#84](https://github.com/ryancinsight/hyperion/pull/84)
  (merge `57dc43348b7f`); the note holds author timings only, with peak stack,
  RSS, no_std timing and other platforms unmeasured.
- Oracle: stored paired comparison of head against `4a888e176199065c7f72731742d4b51af6e94a78`
  on an isolated-core local host (ISA and cores recorded; aarch64 and x86-64),
  O2/O3, f32/f64/F32/F64, the note's three call contexts: median ns/call delta
  with confidence interval, upper edge at most +2.206 ns/call (the largest
  delta the note records); peak stack with callees and RSS, neither above the
  baseline; no_std timing. An excess is root-caused in production code.
- Next step: commit a measurement harness for these metrics, output under the
  run-output root, and record its result in the note.

<a id="HYPERION-M-CORRECTNESS-EXIT-DRIFT"></a>
## HYPERION-M-CORRECTNESS-EXIT-DRIFT — Restate M-correctness exit [patch] — todo
- priority: verification; milestone: M-correctness; needs: none; basis: 8a1f7e3c8af2.
- scope: `backlog/milestones.md`, `docs/adr/0002-release-scope-1-0.md`.
- Outcome: the ladder and ADR match the board once the #84 items are closed:
  `milestones.md:3` names `HYPERION-DIFFUSION-POSITIVE-RANGE` and
  `HYPERION-EFFECTIVE-UNDERFLOW`, which no item carries; the open resource item
  has priority verification, so "no open correctness items" reads met while
  `HYPERION-DIFFUSION-RANGE-RESOURCE` is open; ADR 0002:20 ("open correctness
  repairs") and :33-34 ("blocked correctness items") are stale.
- Oracle: every ID in the exit resolves on the board or is replaced by its
  runnable check; the exit states whether resource acceptance gates M-correctness;
  the ADR lines agree; a dated ADR revision note names the exit change and its
  evidence; a judge verdict covers the ladder edit (sprint: roadmap).
- Next step: rewrite the exit and the ADR lines in one PR, apart from the item closeout.

## HYPERION-007 — Anchor chromophore source oracle [patch] — done 2026-08-20

- Owner: current Atlas session; scope is the chromophore source citation,
  source-knot test oracle, and synchronized provider documentation. Consumer
  repositories and the dirty detached primary checkout are non-goals.
- Acceptance met: the independent OMLC table URL, retrieval date, and
  preformatted `lambda`/`Hb02`/`Hb` column locator are recorded. The test
  fixture is independently transcribed from representative source rows and
  checks every embedded knot. OMLC's 64,500 g/mol value describes hemoglobin
  molecules (tetramers), so the provider applies no second factor of four;
  applying one would double-count the heme groups.
- Evidence: clean lane `fix/hyperion-chromophore-source-oracle`, based on
  fetched `origin/main` `4df62f6`, publishes `4542bc8`. The exact source rows
  at 450, 500, 532, 550, 600, 650, 800, and 1000 nm were reread from the
  OMLC table on 2026-08-20. A temporary source-knot mutation failed the
  source-oracle test and was restored.
- Provider format, locked all-target checks, warning-denied Clippy, all-feature
  and no-default Nextest (23/23 each), doctests (1/1 each), and warning-denied
  Rustdoc pass. `cargo-deny` is unavailable in the local toolchain; prior
  hosted supply-chain evidence remains the applicable external check.

## ATLAS-HYPERION-WORKFLOW-PIN-2026-08-20 — Refresh shared book workflow pin [patch] — done 2026-08-20

- Owner: Atlas integration. Scope is the Pages caller and this PM/changelog
  record; the Hyperion implementation and lockfile remain unchanged.
- Acceptance: pin the caller to the current Atlas reusable workflow while
  preserving the existing executable book-test inputs, then pass the exact
  hosted source and Pages gates before advancing the Atlas gitlink.
- Evidence: source `5e8d47008e01f401c8d1b464c30e2909ff1a56c8`, PR #19, and
  merged default `719d84e80163b958cc5500b5fa44a5b01095d6d1`. Exact PR
  `verify`, `supply-chain`, and Pages book-build checks pass; post-merge CI
  `32341198738`, Deploy mdBook `32341199236`, and dynamic Pages `32341197889`
  pass. Live Pages returns HTTP 200 with the expected Hyperion title.
- Delivery: the Atlas gitlink records the merged default and this provider
  item is closed.

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

## HYPERION-005 — Proteus lockstep consumer pin [patch] — done 2026-08-16

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
- Evidence: `7f36069` advances Proteus `3d6021e7` to `cb70021b` and
  co-evolves Aequitas `681042b` to `5114cd1` plus Eunomia `69ff96d` to
  `88c685f`. The locked provider gates pass against the resulting graph.

## HYPERION-006 — Provider lock closure refresh [patch] — done 2026-08-18

- Owner: current Atlas session; scope: `Cargo.lock` and this provider's local
  PM records. Hyperion source, Proteus source, and peer-owned checkouts are
  non-goals.
- Acceptance met: the locked Aequitas, Eunomia, and Proteus revisions equal the
  fetched provider defaults `260ad10dd5480eef8c82958d1d148199656db59e`,
  `85e590b789505c66f5174043c2e7e851c20547a5`, and
  `f612c9981547d56021db3a1be7f75631fd78ff4c`; hosted `verify` and
  `supply-chain` pass at the exact pushed head.
- The manifest remains Git-sourced with its existing semver requirements. No
  path dependency, compatibility layer, or source change is permitted.
- Local evidence: `cargo fmt --all -- --check` and locked,
  all-feature dependency metadata pass. `cargo check --locked
  --all-features --all-targets` reaches the Atlas overlay and is rejected
  before compilation because the local patches are absent from this standalone
  lock. Full standalone locked compilation, tests, documentation, and
  supply-chain evidence passed in hosted PR #15 at exact head
  `880eb8cce28d1e887942fbeb185a1cf4173c776a`; the provider default is now
  `0156f59f78aba1e3b06d4511ffb1ce30d5c0c6d4`.
