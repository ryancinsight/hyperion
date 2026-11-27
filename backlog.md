# Hyperion backlog

<a id="hyperion-stable-qualification"></a>
## HYPERION-STABLE-QUALIFICATION — hyperion stable-release qualification [patch] — todo
- priority: verification; milestone: M-correctness; basis: `e4dbb88fef35`.
- Outcome: Qualify the stable release. Follow the [release scope](docs/adr/0002-release-scope-1-0.md) and [milestone exits](backlog/milestones.md). Planning targets for `hyperion` are 0.1.2, then 0.2.0, then 1.0.0. Verification alone can advance the current version. Use Proteus Mat 0.3.0 for the [verified downstream contract](gap_audit.md#verified-downstream-provider-migration).
- Representative manifest `hyperion 0.1.1`. Target 1.0.0. The current Cargo manifest disables publication.
- [ ] [patch] Exercise representability near underflow and the positive-range boundaries with Proteus inputs. Record the measured resource cost and the exact-revision review for the consumer contract.
- [ ] [major] Verify the documented Git-first package through a fresh pinned Git consumer, Rustdoc and the M-1.0 value suite. Keep the distribution decision in ADR 0002.
- [x] [patch] **0.1.2.** Effective attenuation attains its two-step bound in both formats; its witness rows are in the range corpus and the attainment test covers both quantities. Diffusion's bound, witnesses and rejection tests are in place. [Derivation and witnesses](docs/verification/diffusion-range.md). [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.2.0.** Compare constitutive outputs with the reference tables through the public Git consumer. Pin the provider commit and record table provenance, units and domain exclusions.
- [x] [patch] Verify Proteus Mat 0.3.0 through the real consumer and independent dependency resolution. [Executed coverage](gap_audit.md#verified-downstream-provider-migration).
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. Rebuild the Git distribution and consumer at exact revisions and repeat both range and reference-table checks. Repeat the applicable exits on the exact RC before stable promotion. Attach exact-revision evidence for the local exits and this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json). Require complete implementations and passing product-path acceptance for all critical features.

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

<a id="HYPERION-THRESHOLD-NOTE-DRIFT"></a>
## HYPERION-THRESHOLD-NOTE-DRIFT — Correct the threshold note's stale text [patch] — todo
- priority: verification; milestone: M-correctness; needs: none; basis: b840f71cabb398bd43c8ab5e93e5646380f3c065.
- scope: `docs/verification/diffusion-threshold.md`.
- Outcome: the note states the step limit as `Quantity::max_steps` (two steps)
  instead of "8ULP", names the eunomia version `Cargo.lock` resolves or none,
  restores the spaces dropped before numbers, and removes the stale clause
  "owner PR82" on line 85 (hyperion#82 was closed unmerged; respacing keeps it).
- Oracle: `git grep -nE "8[- ]?ULP|Existing8|earlier24128|contains16|versus1024|are00800000|sign0|fraction0|Eunomia0|exponent2-|field1|subnormal,2|and15|0\.10|PR ?#?82" -- docs/verification/diffusion-threshold.md`
  prints nothing (11 lines at the basis); `git grep -n "max_steps" -- docs/verification/diffusion-threshold.md`
  prints a line (nothing at the basis); any eunomia version the note names
  equals the `version` of `eunomia` in `Cargo.lock`.
- Next step: edit lines 10, 21-26, 67-68, 72 and 84; on line 85 replace
  "publish=false, owner PR82 and every" with "publish=false and every".
- Source: [post-merge review of #120](https://github.com/ryancinsight/hyperion/pull/120#issuecomment-7228806850).

<a id="HYPERION-RADICAND-THREE"></a>
## HYPERION-RADICAND-THREE — Build the radicand's three without from_f64 [patch] — todo
- priority: tightening; milestone: M-correctness; needs: none; basis: b840f71cabb398bd43c8ab5e93e5646380f3c065.
- scope: `src/transport/diffusion.rs`, `src/transport/diffusion/range.rs`.
- Outcome: `effective_attenuation` takes its tripling from `three::<T>()`
  (`range.rs:5`, private today, so it becomes `pub(super)`) instead of
  converting a literal inside the generic body.
- Oracle: `git grep -nE "from_f(32|64)" -- src/transport/diffusion.rs src/transport/diffusion/`
  prints nothing (prints `diffusion.rs:147` at the basis; the directory is in
  the path so the conversion cannot move into `range.rs`);
  `python scripts/verify_diffusion_range.py` and
  `cargo nextest run -p hyperion --all-features` pass with the witness bits of
  the attainment test unchanged.
- Next step: widen `three`, call it at `diffusion.rs:147`, keep `(3*a)*t` order.
- Source: [post-merge review of #120](https://github.com/ryancinsight/hyperion/pull/120#issuecomment-7228806850).

<a id="HYPERION-LITERAL-CONVERSIONS"></a>
## HYPERION-LITERAL-CONVERSIONS — Build exact constants in T without from_f64 [patch] — todo
- priority: tightening; milestone: M-correctness; needs: none; basis: b840f71cabb398bd43c8ab5e93e5646380f3c065.
- scope: `src/coefficient/chromophore.rs`, `src/reference/nist.rs` (`nist.rs:273-274` is test code in `mod tests`).
- Outcome: no generic body in these files converts a literal through
  `from_f64`: `100.0` (`chromophore.rs:186`), `6.0` (`nist.rs:136`) and `0.0`
  (`nist.rs:273-274`) come from the scalar trait's constants or a unit
  conversion, as `three::<T>()` does in `range.rs`. Table and fixture data stay;
  `LN_10` is its own item; `tests/contracts` (26 lines of the literal grep:
  inputs, expected values, decimals such as `5.329`) is outside.
- Oracle: `git grep -cE "from_f(32|64)" -- src/coefficient/chromophore.rs src/reference/nist.rs`
  prints at most 9 and 21 (10 and 24 at the basis; chromophore.rs keeps the `LN_10`
  line until HYPERION-LN10-CONSTANT lands); `cargo nextest run -p hyperion --all-features` passes.
- Next step: replace the four sites (6 and 100 are exact in every format; derived).
- Source: [#123 judge verdict 1](https://github.com/ryancinsight/hyperion/pull/123#issuecomment-7229000290) (F2), [#123 re-judge](https://github.com/ryancinsight/hyperion/pull/123#issuecomment-7229061714) (R2).

<a id="HYPERION-LN10-CONSTANT"></a>
## HYPERION-LN10-CONSTANT — Take ln 10 from the scalar trait [patch] — blocked
- priority: tightening; milestone: M-correctness; needs: none; basis: b840f71cabb398bd43c8ab5e93e5646380f3c065.
- scope: `src/coefficient/chromophore.rs`.
- Blocker: `RealField` of the locked eunomia 0.12.1 has `PI`, `TAU`, `FRAC_PI_2`, `E`, `LN_2`, `SQRT_2`, `EPSILON` and no `LN_10`
  (`src/traits/field.rs:19-31` in the registry copy). Re-open: a Eunomia release exposing `LN_10`.
- Outcome: `hemoglobin_absorption` takes ln 10 from `RealField::LN_10` instead of
  `T::from_f64(core::f64::consts::LN_10)` (`chromophore.rs:185`).
- Oracle: `git grep -nE "consts::|from_f(32|64)\(.*LN_10" -- src` prints nothing (the one
  line at the basis); `git grep -nE "(T|Self|RealField>)::LN_10" -- src/coefficient/chromophore.rs`
  prints the use (nothing at the basis); `cargo nextest run -p hyperion --all-features` passes.
- Next step: replace line 185 with `T::LN_10` and compare outputs bit for bit in all four formats.
- Source: [#123 judge verdict 1](https://github.com/ryancinsight/hyperion/pull/123#issuecomment-7229000290) (F2), [#123 re-judge](https://github.com/ryancinsight/hyperion/pull/123#issuecomment-7229061714) (R2).

<a id="HYPERION-ATTAINMENT-DOC-WORDING"></a>
## HYPERION-ATTAINMENT-DOC-WORDING — Separate the corpus bound from the witness rows [patch] — todo
- priority: verification; milestone: M-correctness; needs: none; basis: b840f71cabb398bd43c8ab5e93e5646380f3c065.
- scope: `docs/verification/diffusion-range.md`.
- Outcome: lines 96-100 attach "(two at the five witness rows above)" to the
  one-ULP bound for the external 4,421-pair corpus, so the note either counts
  the committed rows in that corpus, which nothing establishes, or contradicts
  "at most one". The corpus bound and the committed rows' two-ULP are stated apart.
- Oracle: `tr '\n' ' ' < docs/verification/diffusion-range.md | grep -oE "external corpus covers[^.]*\.[^.]*\." | grep -c witness`
  prints 0 (1 at the basis): the two sentences on the corpus do not mention
  the witness rows. Wrap-insensitive; that the committed rows' bound is then
  stated correctly is review-judged.
- Next step: split the sentence, keeping the corpus figures as the note states them.
- Source: [post-merge review of #120](https://github.com/ryancinsight/hyperion/pull/120#issuecomment-7228806850) (F3).

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
