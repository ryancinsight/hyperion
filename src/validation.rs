use eunomia::{NumericElement, RealField};

use crate::{TransportError, TransportLaw, ValueConstraint, ValueKind};

#[inline]
pub(crate) fn finite_non_negative<T: RealField>(
    field: ValueKind,
    value: T,
) -> Result<T, TransportError<T>> {
    if value.is_finite() && value >= <T as NumericElement>::ZERO {
        Ok(value)
    } else {
        Err(TransportError::InvalidValue {
            field,
            value,
            constraint: ValueConstraint::FiniteNonNegative,
        })
    }
}

#[inline]
pub(crate) fn finite_positive<T: RealField>(
    field: ValueKind,
    value: T,
) -> Result<T, TransportError<T>> {
    if value.is_finite() && value > <T as NumericElement>::ZERO {
        Ok(value)
    } else {
        Err(TransportError::InvalidValue {
            field,
            value,
            constraint: ValueConstraint::FinitePositive,
        })
    }
}

#[inline]
pub(crate) fn closed_minus_one_to_one<T: RealField>(
    field: ValueKind,
    value: T,
) -> Result<T, TransportError<T>> {
    if value.is_finite()
        && value >= -<T as NumericElement>::ONE
        && value <= <T as NumericElement>::ONE
    {
        Ok(value)
    } else {
        Err(TransportError::InvalidValue {
            field,
            value,
            constraint: ValueConstraint::ClosedMinusOneToOne,
        })
    }
}

#[inline]
pub(crate) fn closed_unit_interval<T: RealField>(
    field: ValueKind,
    value: T,
) -> Result<T, TransportError<T>> {
    if value.is_finite()
        && value >= <T as NumericElement>::ZERO
        && value <= <T as NumericElement>::ONE
    {
        Ok(value)
    } else {
        Err(TransportError::InvalidValue {
            field,
            value,
            constraint: ValueConstraint::ClosedUnitInterval,
        })
    }
}

#[inline]
pub(crate) fn derived_finite<T: RealField>(
    law: TransportLaw,
    value: T,
) -> Result<T, TransportError<T>> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(TransportError::DerivedNonFinite { law, value })
    }
}
