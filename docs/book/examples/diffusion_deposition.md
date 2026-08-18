# Example: Diffusion Coefficients and Deposition

**Source**: examples/book_diffusion_deposition.rs

Derives the reduced scattering `mu_s' = mu_s (1 - g)`, the diffuse-optics
coefficient set from `DiffusionCoefficients` (transport coefficient,
`D = 1 / (3 (mu_a + mu_s'))`, transport mean free path, effective attenuation,
transport albedo), and evaluates both deposition laws
`Q = mu_a phi` and `q = mu_a Phi`.

## Source

```rust
# extern crate aequitas;
# extern crate hyperion;
{{#include ../../../examples/book_diffusion_deposition.rs}}
```

## Output

```text
mu_s' = 15.000000 m^-1 (from mu_s = 20 m^-1, g = 0.25)
mu_t = mu_a + mu_s' = 17.000000 m^-1
D = 1 / (3 mu_t) = 0.019607843 m
transport mean free path = 0.058823529 m
mu_eff = 10.099504938 m^-1 (sqrt(3 mu_a mu_t))
transport albedo = mu_s' / mu_t = 0.882352941
Q = mu_a phi = 1000.000 W/m^3
q = mu_a Phi = 10.000 J/m^3
all diffusion and deposition assertions passed
```
