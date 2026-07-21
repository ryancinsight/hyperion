use core::marker::PhantomData;

use aequitas::{
    systems::si::{dimensions, quantities::ReciprocalLength},
    unit::LinearUnit,
};
use eunomia::RealField;

use super::{CoefficientKind, CoefficientRole};
use crate::{TransportError, ValueKind, validation};

/// Finite, non-negative reciprocal-length coefficient with a static domain role.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InteractionCoefficient<T, Role> {
    quantity: ReciprocalLength<T>,
    role: PhantomData<Role>,
}

impl<T: RealField, Role: CoefficientRole> InteractionCoefficient<T, Role> {
    /// Validate a reciprocal-length quantity for this coefficient role.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] when the canonical-SI value is
    /// negative or non-finite.
    pub fn new(quantity: ReciprocalLength<T>) -> Result<Self, TransportError<T>> {
        let value = validation::finite_non_negative(value_kind::<Role>(), quantity.into_base())?;
        Ok(Self::from_validated(ReciprocalLength::from_base(value)))
    }

    /// Return the value expressed in linear reciprocal-length unit `U`.
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<dimensions::ReciprocalLength>,
    {
        self.quantity.in_unit::<U>()
    }
}

impl<T, Role> InteractionCoefficient<T, Role> {
    /// Borrow the dimensional coefficient quantity.
    #[must_use]
    pub const fn quantity(&self) -> &ReciprocalLength<T> {
        &self.quantity
    }

    /// Move out the dimensional coefficient quantity.
    #[must_use]
    pub fn into_quantity(self) -> ReciprocalLength<T> {
        self.quantity
    }

    pub(crate) const fn from_validated(quantity: ReciprocalLength<T>) -> Self {
        Self {
            quantity,
            role: PhantomData,
        }
    }
}

const fn value_kind<Role: CoefficientRole>() -> ValueKind {
    match Role::KIND {
        CoefficientKind::Absorption => ValueKind::AbsorptionCoefficient,
        CoefficientKind::Scattering => ValueKind::ScatteringCoefficient,
        CoefficientKind::ReducedScattering => ValueKind::ReducedScatteringCoefficient,
        CoefficientKind::LinearAttenuation => ValueKind::LinearAttenuation,
        CoefficientKind::EffectiveAttenuation => ValueKind::EffectiveAttenuation,
        CoefficientKind::Transport => ValueKind::TransportCoefficient,
    }
}
