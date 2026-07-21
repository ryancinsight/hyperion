use aequitas::{
    systems::si::{
        dimensions,
        quantities::{Energy, Length},
    },
    unit::LinearUnit,
};
use eunomia::RealField;

use crate::{TransportError, ValueKind, validation};

/// Finite, non-negative geometric path length.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PathLength<T>(Length<T>);

impl<T: RealField> PathLength<T> {
    /// Validate a physical length.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] when the canonical-SI value is
    /// negative or non-finite.
    pub fn new(quantity: Length<T>) -> Result<Self, TransportError<T>> {
        let value = validation::finite_non_negative(ValueKind::PathLength, quantity.into_base())?;
        Ok(Self(Length::from_base(value)))
    }

    /// Return the path expressed in linear length unit `U`.
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<dimensions::Length>,
    {
        self.0.in_unit::<U>()
    }
}

impl<T> PathLength<T> {
    /// Borrow the dimensional length quantity.
    #[must_use]
    pub const fn quantity(&self) -> &Length<T> {
        &self.0
    }

    /// Move out the dimensional length quantity.
    #[must_use]
    pub fn into_quantity(self) -> Length<T> {
        self.0
    }

    pub(crate) const fn from_validated(quantity: Length<T>) -> Self {
        Self(quantity)
    }
}

/// Finite, positive photon energy.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PhotonEnergy<T>(Energy<T>);

impl<T: RealField> PhotonEnergy<T> {
    /// Validate a photon energy.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] when the canonical-SI value is
    /// zero, negative, or non-finite.
    pub fn new(quantity: Energy<T>) -> Result<Self, TransportError<T>> {
        let value = validation::finite_positive(ValueKind::PhotonEnergy, quantity.into_base())?;
        Ok(Self(Energy::from_base(value)))
    }

    /// Return the energy expressed in linear energy unit `U`.
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<dimensions::Energy>,
    {
        self.0.in_unit::<U>()
    }
}

impl<T> PhotonEnergy<T> {
    /// Borrow the dimensional energy quantity.
    #[must_use]
    pub const fn quantity(&self) -> &Energy<T> {
        &self.0
    }

    /// Move out the dimensional energy quantity.
    #[must_use]
    pub fn into_quantity(self) -> Energy<T> {
        self.0
    }
}
