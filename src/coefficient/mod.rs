//! Validated photon and optical interaction coefficients.

mod chromophore;
mod interaction;
mod mass;
mod optical;
mod role;

pub use chromophore::{DEOXYHEMOGLOBIN, ExtinctionSpectrum, OXYHEMOGLOBIN, hemoglobin_absorption};
pub use interaction::InteractionCoefficient;
pub use mass::MassAttenuation;
pub use optical::OpticalCoefficients;
pub use role::{
    Absorption, AttenuatingRole, CoefficientKind, CoefficientRole, EffectiveAttenuation,
    LinearAttenuation, ReducedScattering, Scattering, Transport,
};
