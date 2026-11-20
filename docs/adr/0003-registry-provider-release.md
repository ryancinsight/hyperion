# ADR 0003: Publish Hyperion under its own registry identity

- Status: Accepted
- Date: 2026-11-20
- Class: `[major]`

## Context

Atlas applications exchange public Rust types across independently published crates. Cargo gives nominal types different identities when their package source or incompatible version differs. A fresh registry lock must select one identity for each type shared across the boundary.

## Decision

Publish hyperion-transport 0.2.0 from the verified provider source. The package name is hyperion-transport and the Rust library name is hyperion. Use the renamed registry package in Cargo manifests and keep hyperion imports. Runtime dependencies use the Aequitas 0.4, Eunomia 0.12 and Proteus 0.2 registry identities.

## Verification

Run package, API, and feature checks on the release revision. Verify the archived source and checksum and compile a consumer from an empty lockfile against the indexed versions.

## Aequitas 0.5 release

hyperion-transport 0.3.0 replaces the public Aequitas 0.4 identity of 0.2.0
with the published Aequitas 0.5 identity. Use Proteus Mat 0.3 with the same Aequitas 0.5 quantity source. Keep the hyperion Rust import.
Eunomia remains at 0.12.1. The release uses the reviewed source that already
selects Aequitas 0.5. The package version marks the breaking change.

Verify the normalized package with Rust 1.95, then build a registry consumer
which passes values of the shared types across the public boundary.
