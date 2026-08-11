# Beer-Lambert transport

Hyperion evaluates local interaction laws from validated coefficients and
quantities. Role-typed `InteractionCoefficient<T, Role>` values carry a
static coefficient role — `Absorption`, `Scattering`, `ReducedScattering`,
`LinearAttenuation`, `EffectiveAttenuation`, or `Transport` — so the same
numeric type cannot be silently mixed across roles.

## Optical depth and transmission

For one homogeneous segment, `InteractionCoefficient::optical_depth(path)`
evaluates `tau = mu L` and returns a validated `OpticalDepth<T>`. The
following is a focused, non-standalone API fragment:

```rust,ignore
let depth = coefficient.optical_depth(path)?;
assert!(depth.into_quantity().into_base() >= 0.0);
```

`total_optical_depth` sums heterogeneous segments additively and returns the
additive identity `tau = 0` for the empty iterator. Transmission follows as
`T = exp(-tau)`; `planar_fluence_at_depth` applies the effective-attenuation
decay `F(z) = F_0 exp(-mu_eff z)` to a validated surface `EnergyFluence`,
returning the fluence at depth. Again a focused, non-standalone fragment:

```rust,ignore
use hyperion::transport::planar_fluence_at_depth;

let fluence = planar_fluence_at_depth(surface, attenuation, depth)?;
println!("fluence at depth: {} J/m^2", fluence.in_unit::<JoulePerSquareMeter>());
```

For `LinearAttenuation` coefficients, `half_value_layer` returns
`ln(2) / mu`; for `EffectiveAttenuation` coefficients, `penetration_depth`
returns `1 / mu_eff`. Both return `None` when the coefficient is zero because
no finite length can be returned.

## Diffusion and derived coefficients

`reduced_scattering(mu_s, g)` derives `mu_s' = mu_s (1 - g)` from the
scattering coefficient and a validated `Anisotropy`. `DiffusionCoefficients`
holds the absorption/reduced-scattering pair and derives the transport
coefficient, `D = 1 / (3 (mu_a + mu_s'))`, transport mean free path, effective
attenuation, and reduced transport albedo. The pair constructor rejects the
degenerate `mu_a + mu_s' = 0` case with `TransportError::DegenerateTransport`
before any derived quantity is exposed.

Mass attenuation is handled separately: `MassAttenuation::to_linear` pairs a
validated area-per-mass value with a `proteus::MassDensity` and converts it to
a `LinearAttenuation` coefficient in one typed step.

Every derived law revalidates its result: a NaN or infinity from otherwise
valid inputs returns `TransportError::DerivedNonFinite` naming the offending
`TransportLaw`. The runnable
[photon-transport example](examples/photon_transport.md) walks the full typed
path from coefficient to fluence, the
[mass-attenuation example](examples/mass_attenuation.md) walks the
`MassAttenuation::to_linear` seam with a Proteus density and evaluates the
resulting half-value layer, and the
[diffusion-and-deposition example](examples/diffusion_deposition.md) derives
the reduced scattering and diffuse-optics coefficient set and evaluates both
local deposition laws.
