# ADR 0002: Hyperion release scope for 1.0

- Status: Accepted
- Date: 2026-11-09
- Class: `[arch]` `[minor]`

## Context

Hyperion (`version = "0.1.1"`, `publish = false`, git-first provider) owns validated
photon/optical interaction coefficients and transport laws for the Atlas stack. The
correctness repairs of PR #84 are landed (merge `57dc43348b7f`); the backlog
carries their resource acceptance (`HYPERION-DIFFUSION-RANGE-RESOURCE`) and
deferred release residuals (`HYPERION-002`). The stack requires a milestone
ladder to 1.0 with runnable exit checks.

## Decision

Adopt a three-milestone ladder to 1.0 recorded in `backlog/milestones.md`:

- **M-correctness** (0.1.2): land and verify the open correctness repairs with
  value-semantic evidence and green provider gates.
- **M-consumer-contract** (0.2.0): maintain verified constitutive/consumer
  contracts and dependency coherence against current stack providers.
- **M-1.0** (1.0.0): release-ready provider state (authority-gated): documented
  public API, examples/doctests, and gates green with no open correctness items.
  Publication remains git-first unless explicit release authority and package
  identity decisions authorize crates.io distribution.

## Alternatives

- Defer a ladder until PR #84 lands: rejected because the roadmap must exist now
  to order work and make progress measurable.
- Target immediate 1.0 without intermediate milestones: rejected because current
  blocked correctness items and consumer-coherence obligations are unclosed.

## Consequences

- Open backlog items carry `milestone:` assignments aligned to the ladder.
- Milestone closure is evidenced by fetched-default commits satisfying each exit
  check; release authority remains explicit for 1.0.
- This ADR is rewritten in place as scope evidence changes.

Revised 2026-11-13: Context states the board after PR #84 landed; the board and merge `57dc43348b7f` are the evidence.
