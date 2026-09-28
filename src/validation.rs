use aequitas::systems::si::quantities::Length;
use eunomia::{NumericElement, RealField};

use crate::{TransportError, TransportLaw, ValueConstraint, ValueKind, quantity::PathLength};

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

/// Validate the sum of two canonical-SI scalars for one derived law.
#[inline]
pub(crate) fn finite_pair_sum<T: RealField>(
    law: TransportLaw,
    first: T,
    second: T,
) -> Result<T, TransportError<T>> {
    derived_finite(law, first + second)
}

/// Validate the ratio of two canonical-SI scalars for one derived law.
#[inline]
pub(crate) fn finite_ratio<T: RealField>(
    law: TransportLaw,
    numerator: T,
    denominator: T,
) -> Result<T, TransportError<T>> {
    derived_finite(law, numerator / denominator)
}

/// Validate a reciprocal length and wrap it as a finite path.
///
/// The caller supplies the already-formed reciprocal (`1 / x` or `LN_2 / x`)
/// so this helper owns only the finiteness check and the quantity wrap.
#[inline]
pub(crate) fn finite_reciprocal_path<T: RealField>(
    law: TransportLaw,
    reciprocal: T,
) -> Result<PathLength<T>, TransportError<T>> {
    let value = derived_finite(law, reciprocal)?;
    Ok(PathLength::from_validated(Length::from_base(value)))
}
