# ADR 0001: Own photon and optical interaction laws in Hyperion

- Status: Accepted
- Date: 2026-07-21
- Class: `[arch]` `[minor]`

## Context

Helios, Kwavers, and CFDrs independently implement exponential attenuation.
Helios additionally owns mass-to-linear photon attenuation and NIST reference
tables. Kwavers repeats reduced-scattering, diffusion-coefficient,
effective-attenuation, penetration-depth, and fluence-decay formulas across its
optics, medium, physics, and solver crates. The repeated laws differ in unit
convention and invalid-input behavior, so copying one consumer API would retain
drift rather than establish a provider.

The governing relations are:

```text
mu_s' = mu_s (1 - g)
tau = sum_i mu_i L_i
T = exp(-tau)
mu = (mu/rho) rho
D = 1 / (3 (mu_a + mu_s'))
mu_eff = sqrt(3 mu_a (mu_a + mu_s'))
delta = 1 / mu_eff
F(z) = F_0 exp(-mu_eff z)
```

NIST defines narrow-beam photon transmission by the exponential attenuation
law and relates mass thickness to density and geometric thickness in
[Section 2 of NISTIR 5632](https://physics.nist.gov/PhysRefData/XrayMassCoef/chap2.html),
equations 1–2. The reduced-scattering and diffusion relations follow the
definitions summarized in Jacques and Pogue,
[Tutorial on methods for estimation of optical absorption and scattering
properties of tissue](https://pmc.ncbi.nlm.nih.gov/articles/PMC11166171/),
Table 1. These relations apply to passive coefficients and, for the diffusion
laws, the diffusion approximation; they do not establish a general
radiative-transfer solver.

The GitHub repository name `ryancinsight/hyperion` was unassigned on
2026-07-21. The crates.io package name is already occupied by an unrelated
L-system crate. Atlas consumes first-party providers through reviewed Git
revisions, so Hyperion uses the repository and Cargo package name with
`publish = false`; registry publication is not part of this contract.

## Decision

Create one independent, public, single-crate provider. Hyperion is `no_std`,
allocation-free, warning-clean, and generic over the sealed
`eunomia::RealField` implementations. It depends inward only:

```text
eunomia ─┐
aequitas ├── hyperion ── helios / kwavers / CFDrs
proteus ─┘
```

Aequitas owns reciprocal-length, area-per-mass, energy-per-area, and all unit
conversion laws. Proteus owns validated material density. Hyperion owns the
domain validity and interaction laws composed from those foundations.

One `InteractionCoefficient<T, Role>` representation carries an Aequitas
reciprocal-length quantity and a sealed zero-sized role. Absorption,
scattering, reduced scattering, linear attenuation, effective attenuation, and
transport are roles, not duplicated numeric wrappers. Bounds live on methods;
the representation remains bound-free and `#[repr(transparent)]` over the
quantity. Static role dispatch monomorphizes without a vtable.

Separate transparent newtypes enforce the domains of anisotropy, path length,
photon energy, energy fluence, optical depth, transmission, optical diffusion
coefficient, and reduced transport albedo. `TransportError<T>` preserves the
rejected value and typed constraint or derived law. Negative and non-finite
physical inputs are rejected. Degenerate diffusion transport is rejected.
Zero attenuation has no finite half-value or penetration length and returns
`None`; it is not represented by an infinite path. Derived non-finite values
are errors rather than clamped or defaulted results.

`NistMassAttenuationTable` owns the bounded, allocation-free 28-knot datasets
for dry air, liquid water, and cortical bone over 0.01–20 MeV. Exact knots
bypass interpolation and convert the stored coefficient through Aequitas;
intervals use native-`T` log-linear interpolation. The table role does not create a material catalog: Proteus owns
material identity, while Hyperion owns photon-energy-to-interaction data.

## Verification

The generic conformance suite instantiates `f32` and `f64` and covers:

- all finite/range validity boundaries and non-finite derived results;
- `mu_s' = mu_s(1-g)`, its endpoint laws, and monotonicity in `g`;
- additive optical depth, `T(0)=1`, and Beer-Lambert concatenation;
- `mu=(mu/rho)rho` through a Proteus mass density;
- `D`, `mu_eff`, `mu_eff^2=mu_a/D`, and finite-depth contracts;
- planar fluence equal to `F_0/e` at one penetration depth;
- exact NIST knots, range rejection, and log-interpolation reference values;
- transparent layout, zero-sized roles, and allocation-free operations.

Floating-point algebraic bounds use `gamma_n = n epsilon / (1 - n epsilon)`
for the counted elementary operations. Transcendental comparisons include an
independently measured reference term and are value-semantic rather than
bitwise after unit conversion.

## Migration

1. Publish and verify Hyperion before changing consumer dependencies.
2. Move Helios coefficient types, NIST tables, optical-depth reduction, and
   Beer-Lambert reference laws; retain CT calibration, geometry, dose, and GPU
   mechanics.
3. Move Kwavers reduced-scattering and diffusion derivations; retain material
   presets, spectra, solvers, and photoacoustic source laws. Delete fabricated
   defaults and conflicting penetration-depth meanings rather than adapting
   them.
4. Replace only the CFDrs exponential law; retain its empirical coefficient,
   hematocrit, path-selection, and scoring policy.
5. Run consumer differentials and residue scans, then register the exact public
   default in Atlas.

## Rejected alternatives

- A broad electromagnetics or radiation package would absorb consumer-owned
  solvers, imaging, dose, and workflow policy.
- Consumer-local unit constructors would duplicate Aequitas and preserve the
  current metre/centimetre ambiguity.
- Raw scalar coefficients would make unit mismatch representable.
- Re-exporting the former Helios or Kwavers symbols would be a compatibility
  layer and retain two public owners.
- A workspace or backend crate has no second deliverable or infrastructure
  boundary in Phase 0.
