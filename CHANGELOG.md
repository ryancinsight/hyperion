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
