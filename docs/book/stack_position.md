# Position in the Atlas stack

Hyperion is the Atlas provider for validated photon and optical interaction
coefficients and their local transport laws. It replaces duplicated
Beer-Lambert, optical-depth, reduced-scattering, diffusion, and reference-table
implementations without absorbing consumer solvers or workflows.

```text
Aequitas  -> dimensions and SI units
Eunomia   -> RealField scalar and numeric law
Proteus   -> validated mass density for mass attenuation
    \
     Hyperion -> coefficients, chromophore spectra, local transport laws
      |
Helios / Kwavers / CFDrs -> photon and acoustic consumers
Gaia / Athena / Leto / Hephaestus -> geometry, solvers, arrays, GPU dispatch
```

Hyperion consumes Aequitas dimensions and units, Eunomia's `RealField`
boundary, and Proteus mass density. It does not define a second units
vocabulary, material identity, or scalar abstraction.

The ownership split is deliberate:

- Hyperion owns validated absorption, scattering, reduced-scattering,
  linear/effective/mass-attenuation coefficients; additive optical depth and
  Beer-Lambert transmission; total attenuation, mean free paths, albedos,
  diffusion coefficients, penetration depth, and planar fluence decay; bounded
  NIST mass-attenuation lookup; chromophore extinction spectra and their
  implied absorption; and local absorbed deposition, `Q = mu_a phi` in W/m³
  and `q = mu_a Phi` in J/m³.
- Proteus owns material identity, tissue presets, and validated mass density.
- Helios, Kwavers, and CFDrs own their equations, geometry, meshes, solvers,
  dose deposition, workflow policy, and any radiative-transfer modeling.
- Gaia, Athena, Leto, and Hephaestus remain the geometry, solver, CPU-array,
  and GPU-execution providers respectively.

The crate is `no_std` in both feature configurations and contains no array,
allocator, scheduler, geometry, backend, or consumer dependency. Its default
`std` feature only propagates standard-library support to its providers.

The provider is Git-first: the public remote is
`https://github.com/ryancinsight/hyperion`, and the occupied crates.io name
keeps the manifest intentionally `publish = false`. Atlas consumers pin
reviewed revisions and their lockfiles provide reproducible source identity.
Registry release work remains a separate deferred watchpoint; this chapter
describes the stable ownership boundary rather than claiming publication.
