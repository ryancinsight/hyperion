//! Validated photon and optical interaction coefficients.

mod interaction;
mod mass;
mod role;

pub use interaction::InteractionCoefficient;
pub use mass::MassAttenuation;
pub use role::{
    Absorption, AttenuatingRole, CoefficientKind, CoefficientRole, EffectiveAttenuation,
    LinearAttenuation, ReducedScattering, Scattering, Transport,
};
