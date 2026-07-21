use core::fmt;

mod private {
    pub trait Sealed {}
}

/// Semantic role carried by an [`InteractionCoefficient`](super::InteractionCoefficient).
pub trait CoefficientRole: private::Sealed + Copy + fmt::Debug {
    /// Runtime diagnostic label for this compile-time role.
    const KIND: CoefficientKind;
}

/// Coefficient roles that define exponential attenuation over a path.
pub trait AttenuatingRole: CoefficientRole {}

/// Diagnostic classification of an interaction coefficient.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CoefficientKind {
    /// Absorption coefficient `mu_a`.
    Absorption,
    /// Scattering coefficient `mu_s`.
    Scattering,
    /// Reduced-scattering coefficient `mu_s'`.
    ReducedScattering,
    /// Linear attenuation coefficient `mu`.
    LinearAttenuation,
    /// Effective attenuation coefficient `mu_eff`.
    EffectiveAttenuation,
    /// Diffusion transport coefficient `mu_a + mu_s'`.
    Transport,
}

/// Absorption-coefficient role.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Absorption;
impl private::Sealed for Absorption {}
impl CoefficientRole for Absorption {
    const KIND: CoefficientKind = CoefficientKind::Absorption;
}
impl AttenuatingRole for Absorption {}

/// Scattering-coefficient role.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Scattering;
impl private::Sealed for Scattering {}
impl CoefficientRole for Scattering {
    const KIND: CoefficientKind = CoefficientKind::Scattering;
}
impl AttenuatingRole for Scattering {}

/// Reduced-scattering-coefficient role.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReducedScattering;
impl private::Sealed for ReducedScattering {}
impl CoefficientRole for ReducedScattering {
    const KIND: CoefficientKind = CoefficientKind::ReducedScattering;
}

/// Linear-attenuation-coefficient role.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LinearAttenuation;
impl private::Sealed for LinearAttenuation {}
impl CoefficientRole for LinearAttenuation {
    const KIND: CoefficientKind = CoefficientKind::LinearAttenuation;
}
impl AttenuatingRole for LinearAttenuation {}

/// Effective-attenuation-coefficient role.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EffectiveAttenuation;
impl private::Sealed for EffectiveAttenuation {}
impl CoefficientRole for EffectiveAttenuation {
    const KIND: CoefficientKind = CoefficientKind::EffectiveAttenuation;
}
impl AttenuatingRole for EffectiveAttenuation {}

/// Diffusion transport-coefficient role.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Transport;
impl private::Sealed for Transport {}
impl CoefficientRole for Transport {
    const KIND: CoefficientKind = CoefficientKind::Transport;
}
