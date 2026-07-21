//! Photon and optical interaction laws.

mod beer_lambert;
mod diffusion;

pub use beer_lambert::{planar_fluence_at_depth, total_optical_depth};
pub use diffusion::{DiffusionCoefficients, OpticalDiffusionCoefficient, reduced_scattering};
