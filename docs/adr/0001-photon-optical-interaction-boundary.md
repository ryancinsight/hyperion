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
mu_t = mu_a + mu_s
ell = 1 / mu_t
omega = mu_s / mu_t
tau = sum_i mu_i L_i
T = exp(-tau)
mu = (mu/rho) rho
D = 1 / (3 (mu_a + mu_s'))
ell_tr = 1 / (mu_a + mu_s')
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

`OpticalCoefficients<T>` owns the unreduced pair and derives total attenuation,
ordinary mean free path, and ordinary single-scattering albedo.
`DiffusionCoefficients<T>` owns the absorption/reduced-scattering pair and
derives transport coefficient, transport mean free path, diffusion coefficient,
effective attenuation, and reduced transport albedo. Vacuum is valid for the
unreduced pair and returns `None` for ratios or reciprocal lengths that have no
finite value; the diffusion pair rejects a zero transport coefficient.

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
intervals use a native-`T` natural cubic spline in log-energy/log-coefficient
space, matching the interpolation family described by
[NIST XCOM §3](https://physics.nist.gov/PhysRefData/Xcom/Text/chap3.html).
The published four-significant-digit values are an interpolation aid, not an
accuracy claim, so this sparse table makes no global error guarantee between
knots. Natural endpoint second derivatives are zero because the embedded table
does not publish endpoint slopes; this is an explicit local boundary choice,
not a NIST accuracy claim. The contract suite uses independently queried XCOM
off-knot values to detect method regressions; it does not turn rounded output
into a fabricated tolerance. The table role does not create a material catalog: Proteus owns
material identity, while Hyperion owns photon-energy-to-interaction data.

## Revision 2026-08-20: Chromophore spectra ownership

The chromophore spectra introduced after the original decision are part of
Hyperion's optical-coefficient boundary. They are reference data that feed the
owned Beer–Lambert absorption law, not consumer-specific tissue presets or
photoacoustic workflows. Hyperion therefore owns the tabulated oxy- and
deoxyhemoglobin spectra and their concentration-validation contract; Proteus
continues to own material identity and Kwavers retains photoacoustic source and
diagnostic workflow policy.

The source is Scott Prahl's 1999 OMLC compilation, which identifies Gratzer and
Kollias as source contributors:
<https://omlc.org/spectra/hemoglobin/summary.html> (retrieved 2026-08-20),
specifically its preformatted `lambda`/`Hb02`/`Hb` table columns. OMLC states
that its tabulated molar extinction uses 64,500 g/mol hemoglobin. Because that
molecular mass is the hemoglobin tetramer, the values pair directly with
tetramer-molar concentrations. The earlier implementation's additional factor
of four was a per-heme interpretation contradicted by the source's
molecular-mass contract; the provider now stores the source values directly.
The validity boundary is the compiled 450–1000 nm subset, continuous linear
interpolation between its source knots, and rejection outside that measured
range.

The alternatives were to retain the relocated Kwavers table as the oracle, or
to preserve the factor of four and document it as a local convention. Both
would leave the source normalization unverified. The independent source-knot
test and the explicit source citation are the acceptance evidence for this
revision.

## Verification

The generic conformance suite instantiates `f32` and `f64` and covers:

- all finite/range validity boundaries and non-finite derived results;
- `mu_s' = mu_s(1-g)`, its endpoint laws, and monotonicity in `g`;
- additive optical depth, `T(0)=1`, and Beer-Lambert concatenation;
- `mu=(mu/rho)rho` through a Proteus mass density;
- `mu_t`, ordinary mean free path and albedo, including the vacuum boundary;
- `D`, transport mean free path, `mu_eff`, `mu_eff^2=mu_a/D`, and finite-depth
  contracts;
- planar fluence equal to `F_0/e` at one penetration depth;
- exact NIST knots, range rejection, and independent XCOM off-knot method
  regression values;
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
