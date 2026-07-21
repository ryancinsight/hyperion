use aequitas::{
    systems::si::{dimensions, quantities::AreaPerMass, quantities::ReciprocalLength},
    unit::LinearUnit,
};
use eunomia::RealField;
use proteus::MassDensity;

use super::{InteractionCoefficient, LinearAttenuation};
use crate::{TransportError, TransportLaw, ValueKind, validation};

/// Finite, non-negative photon mass attenuation coefficient.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MassAttenuation<T>(AreaPerMass<T>);

impl<T: RealField> MassAttenuation<T> {
    /// Validate an area-per-mass quantity.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] when the canonical-SI value is
    /// negative or non-finite.
    pub fn new(quantity: AreaPerMass<T>) -> Result<Self, TransportError<T>> {
        let value =
            validation::finite_non_negative(ValueKind::MassAttenuation, quantity.into_base())?;
        Ok(Self(AreaPerMass::from_base(value)))
    }

    /// Convert mass attenuation to linear attenuation using validated density.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the product overflows.
    pub fn to_linear(
        self,
        density: MassDensity<T>,
    ) -> Result<InteractionCoefficient<T, LinearAttenuation>, TransportError<T>> {
        let quantity: ReciprocalLength<T> = self.0 * density.into_quantity();
        let value = validation::derived_finite(
            TransportLaw::MassToLinearAttenuation,
            quantity.into_base(),
        )?;
        Ok(InteractionCoefficient::from_validated(
            ReciprocalLength::from_base(value),
        ))
    }

    /// Return the value expressed in area-per-mass unit `U`.
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<dimensions::AreaPerMass>,
    {
        self.0.in_unit::<U>()
    }
}

impl<T> MassAttenuation<T> {
    /// Borrow the dimensional mass-attenuation quantity.
    #[must_use]
    pub const fn quantity(&self) -> &AreaPerMass<T> {
        &self.0
    }

    /// Move out the dimensional mass-attenuation quantity.
    #[must_use]
    pub fn into_quantity(self) -> AreaPerMass<T> {
        self.0
    }
}
