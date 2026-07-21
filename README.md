# Hyperion

Hyperion is the Atlas provider for validated photon and optical interaction
coefficients and their local transport laws. It consolidates repeated
Beer-Lambert, optical-depth, reduced-scattering, diffusion, and reference-table
implementations without absorbing consumer solvers or workflows.

The name refers to Hyperion, the Titan associated with heavenly light.

## Distribution

Hyperion is a public Git-first provider at
`https://github.com/ryancinsight/hyperion`. The unrelated crates.io name is
already occupied, so this package is intentionally `publish = false`. Atlas
consumers pin reviewed Git revisions and their lockfiles provide reproducible
source identity.

## Boundary

Hyperion owns:

- validated absorption, scattering, reduced-scattering, linear-attenuation,
  effective-attenuation, and mass-attenuation coefficients;
- additive optical depth and Beer-Lambert transmission;
- total attenuation, ordinary and transport mean free paths, ordinary and
  reduced transport albedos, diffusion coefficient, effective attenuation,
  penetration depth, and planar fluence decay;
- bounded NIST mass-attenuation lookup for the first-wave photon materials.

Hyperion does not own material identity or tissue presets, chromophore spectra,
CT/HU calibration, ray or mesh geometry, numerical solvers, dose deposition,
GPU dispatch, Maxwell or radiative-transfer solvers, photoacoustic source
generation, or workflow policy. These remain with `Proteus`, `Helios`,
`Kwavers`, `CFDrs`, `Gaia`, `Athena`, and `Hephaestus`.

## Example

```rust
use aequitas::systems::si::{
    quantities::{EnergyPerArea, ReciprocalLength},
    units::{JoulePerSquareMeter, PerMeter},
};
use hyperion::{
    coefficient::{EffectiveAttenuation, InteractionCoefficient},
    quantity::{EnergyFluence, PathLength},
    transport::planar_fluence_at_depth,
};
use aequitas::systems::si::{quantities::Length, units::Meter};

let attenuation = InteractionCoefficient::<_, EffectiveAttenuation>::new(
    ReciprocalLength::from_unit::<PerMeter>(2.0_f64),
)?;
let surface = EnergyFluence::new(
    EnergyPerArea::from_unit::<JoulePerSquareMeter>(10.0_f64),
)?;
let depth = PathLength::new(Length::from_unit::<Meter>(0.5_f64))?;
let fluence = planar_fluence_at_depth(surface, attenuation, depth)?;

let expected = 10.0 / core::f64::consts::E;
let gamma_32 = 32.0 * f64::EPSILON / (1.0 - 32.0 * f64::EPSILON);
assert!((fluence.into_quantity().into_base() - expected).abs()
    <= gamma_32 * expected.max(1.0));
# Ok::<(), hyperion::TransportError<f64>>(())
```

## Architecture

```text
src/
├── coefficient/  # role-typed reciprocal-length and mass coefficients
├── quantity/     # validated paths, energies, fluence, and ratios
├── transport/    # Beer-Lambert and diffusion laws
├── reference/    # bounded NIST photon-interaction tables
├── error.rs      # allocation-free typed failures
└── lib.rs        # documented module and re-export manifest
```

All arithmetic executes in `T: eunomia::RealField`; no computation widens to a
different scalar. Aequitas owns dimensions and units. Proteus supplies
validated mass density. Hyperion contains no array, allocator, scheduler,
geometry, backend, or consumer dependency.

## Mathematical specification

For passive coefficients and non-negative path length:

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

The coefficient and Beer-Lambert definitions follow
[NISTIR 5632, Section 2](https://physics.nist.gov/PhysRefData/XrayMassCoef/chap2.html).
The diffuse-optics definitions and their domain follow the summary in
[Jacques and Pogue, Table 1](https://pmc.ncbi.nlm.nih.gov/articles/PMC11166171/).
The ownership decision, validity boundaries, numerical evidence, and migration
ledger are recorded in
[ADR 0001](docs/adr/0001-photon-optical-interaction-boundary.md).

## Verification

The committed gate is:

```sh
cargo fmt --all -- --check
cargo check --locked --all-features
cargo check --locked --no-default-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo nextest run --locked --all-features
cargo test --locked --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features
cargo run --locked --example photon_transport
cargo deny check
```

## License

Licensed under either the MIT License or Apache License 2.0.
