use aequitas::systems::si::quantities::Dimensionless;
use eunomia::RealField;

use crate::{TransportError, ValueKind, validation};

/// Finite scattering anisotropy factor in the closed interval `[-1, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Anisotropy<T>(Dimensionless<T>);

impl<T: RealField> Anisotropy<T> {
    /// Validate a dimensionless scattering anisotropy.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] outside `[-1, 1]` or for a
    /// non-finite value.
    pub fn new(quantity: Dimensionless<T>) -> Result<Self, TransportError<T>> {
        let value =
            validation::closed_minus_one_to_one(ValueKind::Anisotropy, quantity.into_base())?;
        Ok(Self(Dimensionless::from_base(value)))
    }
}

impl<T> Anisotropy<T> {
    /// Borrow the dimensionless anisotropy quantity.
    #[must_use]
    pub const fn quantity(&self) -> &Dimensionless<T> {
        &self.0
    }

    /// Move out the dimensionless anisotropy quantity.
    #[must_use]
    pub fn into_quantity(self) -> Dimensionless<T> {
        self.0
    }
}
