use aequitas::{
    systems::si::{dimensions, quantities::EnergyPerArea},
    unit::LinearUnit,
};
use eunomia::RealField;

use super::Transmission;
use crate::{TransportError, TransportLaw, ValueKind, validation};

/// Finite, non-negative energy fluence.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EnergyFluence<T>(EnergyPerArea<T>);

impl<T: RealField> EnergyFluence<T> {
    /// Validate an energy-per-area quantity.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] when the canonical-SI value is
    /// negative or non-finite.
    pub fn new(quantity: EnergyPerArea<T>) -> Result<Self, TransportError<T>> {
        let value =
            validation::finite_non_negative(ValueKind::EnergyFluence, quantity.into_base())?;
        Ok(Self(EnergyPerArea::from_base(value)))
    }

    /// Apply a validated transmission fraction.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] if multiplication produces
    /// a non-finite result.
    pub fn attenuate(self, transmission: Transmission<T>) -> Result<Self, TransportError<T>> {
        let value = self.0.into_base() * transmission.into_quantity().into_base();
        let valid = validation::derived_finite(TransportLaw::PlanarFluence, value)?;
        Ok(Self::from_validated(EnergyPerArea::from_base(valid)))
    }

    /// Return the fluence expressed in linear energy-per-area unit `U`.
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<dimensions::EnergyPerArea>,
    {
        self.0.in_unit::<U>()
    }
}

impl<T> EnergyFluence<T> {
    /// Borrow the dimensional fluence quantity.
    #[must_use]
    pub const fn quantity(&self) -> &EnergyPerArea<T> {
        &self.0
    }

    /// Move out the dimensional fluence quantity.
    #[must_use]
    pub fn into_quantity(self) -> EnergyPerArea<T> {
        self.0
    }

    pub(crate) const fn from_validated(quantity: EnergyPerArea<T>) -> Self {
        Self(quantity)
    }
}
