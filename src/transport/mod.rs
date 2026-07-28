//! Photon and optical interaction laws.

mod beer_lambert;
mod deposition;
mod diffusion;

pub use beer_lambert::{planar_fluence_at_depth, total_optical_depth};
pub use deposition::{absorbed_energy_density, absorbed_power_density};
pub use diffusion::{DiffusionCoefficients, OpticalDiffusionCoefficient, reduced_scattering};
