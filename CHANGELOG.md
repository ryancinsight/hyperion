# Changelog

All externally observable changes are recorded here.

## 0.2.0 — Unreleased

### Breaking

- Hyperion's public scalar bounds now use Eunomia 0.12. Generic clients using
  Eunomia 0.11 `RealField` / `UnitScalar` bounds cannot call the new provider's
  methods. Adopt Hyperion 0.2 and Eunomia 0.12 together, with compatible
  Aequitas 0.3.2 and Proteus 0.1.1 sources, then regenerate the consumer lock
  normally and qualify its generic and numerical callers. No adapter bridges
  the old trait identity. See [ADR 0002](docs/adr/0002-eunomia-012-provider-boundary.md).

### Fixed

- Recover finite planar fluence when rounding the intermediate transmission
  would erase a representable normal result. Keep ordinary values, signed
  zeros, typed optical-depth errors and standalone transmission unchanged.
  Final subnormal results retain floating-point rounding error, including at
  the zero transition; correct rounding is not guaranteed.

### Changed

- Align the Git dependency graph with Aequitas 0.3.2, Eunomia 0.12.0 and
  Proteus 0.1.1. The package retains its Git-first `publish = false` identity.
  Merge requires a coordinated consumer migration; current Hyperion 0.1
  requirements do not accept this pre-1.0 breaking boundary.

### Verification

- Retain the two public-API red witnesses, 740 high-precision range comparisons,
  136 final-underflow comparisons and zero-allocation recovery checks. See
  [the verification note](docs/verification/planar-fluence-range.md) for the
  exponent-range argument, accuracy contract and measured performance limits.

## 0.1.1 — Unreleased

### Fixed

- Preserve positive finite optical diffusion coefficients when tripling the
  transport coefficient would overflow, retaining the positive wrapper invariant.
- Recover effective attenuation lost to radicand underflow with native binary
  scaling; retain legitimate zero absorption and typed nonfinite-radicand errors.
  Recovered attenuation feeds finite penetration and planar-fluence evaluation.

### Changed

- Construct the exact native minimum-normal threshold by IEEE encoding instead
  of a constant scaling call. Preserve every branch/value/error decision for
  the supported scalar cohort; retain corrected mixed-workload attribution.

### Verification

- Add330 threshold-neighbor rows and four-scalar encoding/zero/nonfinite policy
  checks; document the complete constant-identity proof and measured cost limits.
- Preserve the four original failures/four controls; add rational boundary
  fixtures and actual downstream coverage. Source/ordinary-cost evidence and
  limitations are documented in docs/verification/diffusion-range.md. Independent
  review and resource acceptance remain required; Git-first publish=false stays.

## Unreleased

### Added

- Absorbed-deposition laws `Q = mu_a * phi` and `q = mu_a * Phi`, with the
  validated `FluenceRate`, `AbsorbedPowerDensity`, and `AbsorbedEnergyDensity`
  quantities. Deposition is where a transport result becomes a source term for
  a thermal or damage model, so it belongs to one owner rather than to each
  consumer. Verified against the Beer-Lambert conservation identity
  `integral of mu_a Phi_0 exp(-mu_a x) dx = Phi_0`, an oracle independent of
  the product being computed.
- Validated, role-typed photon and optical interaction coefficients over
  Aequitas quantities and Eunomia real scalars.
- Typed optical depth, Beer-Lambert transmission, diffuse optical coefficients,
  ordinary and transport mean free paths, ordinary and reduced transport
  albedos, penetration depth, planar fluence decay, and mass-to-linear
  attenuation.
- Bounded NIST mass-attenuation reference tables for dry air, liquid water,
  and cortical bone over 0.01–20 MeV.

### Changed

- Correct the hemoglobin spectrum normalization against Prahl's OMLC source:
  the published values already use 64,500 g/mol hemoglobin (the tetramer), so
  the provider no longer applies an additional factor of four before pairing
  them with tetramer-molar concentrations. Source-knot regression coverage and
  the ownership decision are now independent of the retired Kwavers table.

- Refresh the Pages caller to the current Atlas reusable workflow revision;
  retain the existing package-staged executable book gate. Exact hosted CI,
  mdBook, and Pages deployment runs pass at merged default `719d84e`; live
  Pages returns HTTP 200.

- NIST reference-table intervals now use a native-`T` natural cubic spline in
  log-energy/log-coefficient space, matching the interpolation family
  documented by XCOM. The contract suite adds independently queried liquid-
  water off-knot values and records that XCOM's fourth displayed digit is an
  interpolation aid rather than an accuracy guarantee.
- `hemoglobin_absorption` validates both molar concentrations through the
  shared `finite_non_negative` boundary check before combining them, so an
  invalid negative or non-finite input is rejected with
  `ValueKind::ChromophoreConcentration` instead of silently propagating into
  the linear combination. Monomorphization across `f32`/`f64` is pinned by
  test.
- Generic quantity and transport contracts now carry Eunomia's
  provider-owned `UnitScalar` bound wherever they convert through Aequitas
  linear units. This preserves the existing real scalar behavior while
  keeping the provider contract compile-complete.
