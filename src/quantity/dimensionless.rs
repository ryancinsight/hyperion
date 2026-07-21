use aequitas::systems::si::quantities::Dimensionless;
use eunomia::{NumericElement, RealField};

use crate::{TransportError, TransportLaw, ValueKind, validation};

/// Finite, non-negative optical depth `tau`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OpticalDepth<T>(Dimensionless<T>);

impl<T: RealField> OpticalDepth<T> {
    /// Validate a dimensionless optical depth.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] for a negative or non-finite
    /// value.
    pub fn new(quantity: Dimensionless<T>) -> Result<Self, TransportError<T>> {
        let value = validation::finite_non_negative(ValueKind::OpticalDepth, quantity.into_base())?;
        Ok(Self(Dimensionless::from_base(value)))
    }

    /// The empty-path identity `tau = 0`.
    #[must_use]
    pub fn zero() -> Self {
        Self(Dimensionless::from_base(<T as NumericElement>::ZERO))
    }

    /// Add optical depths while preserving the finite invariant.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the sum overflows.
    pub fn checked_add(self, other: Self) -> Result<Self, TransportError<T>> {
        let sum = self.0.into_base() + other.0.into_base();
        let valid = validation::derived_finite(TransportLaw::OpticalDepth, sum)?;
        Ok(Self::from_validated(Dimensionless::from_base(valid)))
    }

    /// Evaluate Beer-Lambert transmission `exp(-tau)`.
    #[must_use]
    pub fn transmission(self) -> Transmission<T> {
        let value = (-self.0.into_base()).exp();
        Transmission::from_validated(Dimensionless::from_base(value))
    }
}

impl<T> OpticalDepth<T> {
    /// Borrow the dimensionless optical-depth quantity.
    #[must_use]
    pub const fn quantity(&self) -> &Dimensionless<T> {
        &self.0
    }

    /// Move out the dimensionless optical-depth quantity.
    #[must_use]
    pub fn into_quantity(self) -> Dimensionless<T> {
        self.0
    }

    pub(crate) const fn from_validated(quantity: Dimensionless<T>) -> Self {
        Self(quantity)
    }
}

/// Finite Beer-Lambert transmission fraction in `[0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Transmission<T>(Dimensionless<T>);

impl<T: RealField> Transmission<T> {
    /// Validate a transmission fraction.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] outside `[0, 1]` or for a
    /// non-finite value.
    pub fn new(quantity: Dimensionless<T>) -> Result<Self, TransportError<T>> {
        let value =
            validation::closed_unit_interval(ValueKind::Transmission, quantity.into_base())?;
        Ok(Self(Dimensionless::from_base(value)))
    }
}

impl<T> Transmission<T> {
    /// Borrow the dimensionless transmission quantity.
    #[must_use]
    pub const fn quantity(&self) -> &Dimensionless<T> {
        &self.0
    }

    /// Move out the dimensionless transmission quantity.
    #[must_use]
    pub fn into_quantity(self) -> Dimensionless<T> {
        self.0
    }

    pub(crate) const fn from_validated(quantity: Dimensionless<T>) -> Self {
        Self(quantity)
    }
}

/// Finite ordinary single-scattering albedo `mu_s / (mu_a + mu_s)` in `[0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SingleScatteringAlbedo<T>(Dimensionless<T>);

impl<T: RealField> SingleScatteringAlbedo<T> {
    /// Validate an ordinary single-scattering albedo.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] outside `[0, 1]` or for a
    /// non-finite value.
    pub fn new(quantity: Dimensionless<T>) -> Result<Self, TransportError<T>> {
        let value = validation::closed_unit_interval(
            ValueKind::SingleScatteringAlbedo,
            quantity.into_base(),
        )?;
        Ok(Self(Dimensionless::from_base(value)))
    }
}

impl<T> SingleScatteringAlbedo<T> {
    /// Borrow the dimensionless albedo quantity.
    #[must_use]
    pub const fn quantity(&self) -> &Dimensionless<T> {
        &self.0
    }

    /// Move out the dimensionless albedo quantity.
    #[must_use]
    pub fn into_quantity(self) -> Dimensionless<T> {
        self.0
    }

    pub(crate) const fn from_validated(quantity: Dimensionless<T>) -> Self {
        Self(quantity)
    }
}

/// Finite reduced transport albedo `mu_s' / (mu_a + mu_s')` in `[0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TransportAlbedo<T>(Dimensionless<T>);

impl<T: RealField> TransportAlbedo<T> {
    /// Validate a reduced transport albedo.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] outside `[0, 1]` or for a
    /// non-finite value.
    pub fn new(quantity: Dimensionless<T>) -> Result<Self, TransportError<T>> {
        let value =
            validation::closed_unit_interval(ValueKind::TransportAlbedo, quantity.into_base())?;
        Ok(Self(Dimensionless::from_base(value)))
    }
}

impl<T> TransportAlbedo<T> {
    /// Borrow the dimensionless albedo quantity.
    #[must_use]
    pub const fn quantity(&self) -> &Dimensionless<T> {
        &self.0
    }

    /// Move out the dimensionless albedo quantity.
    #[must_use]
    pub fn into_quantity(self) -> Dimensionless<T> {
        self.0
    }

    pub(crate) const fn from_validated(quantity: Dimensionless<T>) -> Self {
        Self(quantity)
    }
}
