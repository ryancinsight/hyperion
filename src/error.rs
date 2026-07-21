use core::fmt;

/// Domain value whose validity boundary failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueKind {
    /// Absorption coefficient.
    AbsorptionCoefficient,
    /// Scattering coefficient.
    ScatteringCoefficient,
    /// Reduced-scattering coefficient.
    ReducedScatteringCoefficient,
    /// Linear attenuation coefficient.
    LinearAttenuation,
    /// Effective attenuation coefficient.
    EffectiveAttenuation,
    /// Transport coefficient.
    TransportCoefficient,
    /// Mass attenuation coefficient.
    MassAttenuation,
    /// Scattering anisotropy.
    Anisotropy,
    /// Physical path length.
    PathLength,
    /// Photon energy.
    PhotonEnergy,
    /// Energy fluence.
    EnergyFluence,
    /// Optical depth.
    OpticalDepth,
    /// Transmission fraction.
    Transmission,
    /// Optical diffusion coefficient.
    OpticalDiffusionCoefficient,
    /// Reduced transport albedo.
    TransportAlbedo,
}

/// Constraint imposed at a Hyperion validity boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueConstraint {
    /// The value must be finite and at least zero.
    FiniteNonNegative,
    /// The value must be finite and greater than zero.
    FinitePositive,
    /// The value must be finite and lie in the closed interval `[-1, 1]`.
    ClosedMinusOneToOne,
    /// The value must be finite and lie in the closed interval `[0, 1]`.
    ClosedUnitInterval,
}

/// Derived interaction law whose evaluation produced a non-finite value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransportLaw {
    /// Reduced-scattering derivation.
    ReducedScattering,
    /// Mass-to-linear attenuation conversion.
    MassToLinearAttenuation,
    /// Optical-depth product or reduction.
    OpticalDepth,
    /// Optical diffusion coefficient.
    DiffusionCoefficient,
    /// Reduced transport albedo.
    TransportAlbedo,
    /// Effective attenuation coefficient.
    EffectiveAttenuation,
    /// Penetration-depth reciprocal.
    PenetrationDepth,
    /// Half-value-layer reciprocal.
    HalfValueLayer,
    /// Planar fluence attenuation.
    PlanarFluence,
    /// NIST table interpolation.
    NistInterpolation,
}

/// Typed failure at a photon or optical interaction boundary.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum TransportError<T> {
    /// A caller-provided value violated its domain constraint.
    InvalidValue {
        /// Rejected value role.
        field: ValueKind,
        /// Rejected canonical-SI scalar.
        value: T,
        /// Violated constraint.
        constraint: ValueConstraint,
    },
    /// The sum `mu_a + mu_s'` is zero, so diffusion quantities are undefined.
    DegenerateTransport,
    /// A derived law produced NaN or infinity from otherwise valid inputs.
    DerivedNonFinite {
        /// Law that failed.
        law: TransportLaw,
        /// Non-finite derived canonical-SI scalar.
        value: T,
    },
    /// Photon energy lies outside a bounded reference table.
    PhotonEnergyOutOfRange {
        /// Rejected energy in the table's declared unit.
        value: T,
        /// Inclusive lower bound.
        minimum: T,
        /// Inclusive upper bound.
        maximum: T,
    },
}

impl<T: fmt::Debug> fmt::Display for TransportError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue {
                field,
                value,
                constraint,
            } => write!(
                formatter,
                "{field:?} value {value:?} violates {constraint:?}"
            ),
            Self::DegenerateTransport => {
                formatter.write_str("absorption plus reduced scattering must be positive")
            }
            Self::DerivedNonFinite { law, value } => {
                write!(formatter, "{law:?} produced non-finite value {value:?}")
            }
            Self::PhotonEnergyOutOfRange {
                value,
                minimum,
                maximum,
            } => write!(
                formatter,
                "photon energy {value:?} lies outside [{minimum:?}, {maximum:?}]"
            ),
        }
    }
}

impl<T: fmt::Debug> core::error::Error for TransportError<T> {}
