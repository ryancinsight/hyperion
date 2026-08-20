# Changelog

All externally observable changes are recorded here.

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
