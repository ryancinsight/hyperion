use aequitas::systems::si::quantities::{Dimensionless, Length};
use eunomia::{NumericElement, RealField};

use crate::{
    TransportError, TransportLaw,
    coefficient::{
        AttenuatingRole, EffectiveAttenuation, InteractionCoefficient, LinearAttenuation,
    },
    quantity::{EnergyFluence, OpticalDepth, PathLength},
    validation,
};

impl<T: RealField, Role: AttenuatingRole> InteractionCoefficient<T, Role> {
    /// Evaluate the dimensionless optical depth `tau = mu L` for one segment.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the product overflows.
    pub fn optical_depth(self, path: PathLength<T>) -> Result<OpticalDepth<T>, TransportError<T>> {
        let quantity: Dimensionless<T> = self.into_quantity() * path.into_quantity();
        let value = validation::derived_finite(TransportLaw::OpticalDepth, quantity.into_base())?;
        Ok(OpticalDepth::from_validated(Dimensionless::from_base(
            value,
        )))
    }
}

impl<T: RealField> InteractionCoefficient<T, LinearAttenuation> {
    /// Return the half-value layer `ln(2) / mu`.
    ///
    /// Zero attenuation returns `None` because no finite path halves the beam.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] if the reciprocal overflows.
    pub fn half_value_layer(self) -> Result<Option<PathLength<T>>, TransportError<T>> {
        let coefficient = self.into_quantity().into_base();
        if coefficient == <T as NumericElement>::ZERO {
            return Ok(None);
        }
        let length = T::LN_2 / coefficient;
        let valid = validation::derived_finite(TransportLaw::HalfValueLayer, length)?;
        Ok(Some(PathLength::from_validated(Length::from_base(valid))))
    }
}

impl<T: RealField> InteractionCoefficient<T, EffectiveAttenuation> {
    /// Return the penetration depth `1 / mu_eff`.
    ///
    /// Zero effective attenuation returns `None` because the decay length is
    /// not finite.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] if the reciprocal overflows.
    pub fn penetration_depth(self) -> Result<Option<PathLength<T>>, TransportError<T>> {
        let coefficient = self.into_quantity().into_base();
        if coefficient == <T as NumericElement>::ZERO {
            return Ok(None);
        }
        let length = coefficient.recip();
        let valid = validation::derived_finite(TransportLaw::PenetrationDepth, length)?;
        Ok(Some(PathLength::from_validated(Length::from_base(valid))))
    }
}

/// Sum optical depth across heterogeneous path segments.
///
/// The empty iterator returns the additive identity `tau = 0`.
///
/// # Errors
///
/// Returns [`TransportError::DerivedNonFinite`] when a product or partial sum
/// overflows.
pub fn total_optical_depth<T, Role, Segments>(
    segments: Segments,
) -> Result<OpticalDepth<T>, TransportError<T>>
where
    T: RealField,
    Role: AttenuatingRole,
    Segments: IntoIterator<Item = (InteractionCoefficient<T, Role>, PathLength<T>)>,
{
    segments
        .into_iter()
        .try_fold(OpticalDepth::zero(), |total, (coefficient, path)| {
            total.checked_add(coefficient.optical_depth(path)?)
        })
}

/// Attenuate planar energy fluence at depth using `F(z) = F_0 exp(-mu_eff z)`.
///
/// # Errors
///
/// Returns [`TransportError::DerivedNonFinite`] when optical depth or the
/// attenuated fluence becomes non-finite.
pub fn planar_fluence_at_depth<T: RealField>(
    surface: EnergyFluence<T>,
    attenuation: InteractionCoefficient<T, EffectiveAttenuation>,
    depth: PathLength<T>,
) -> Result<EnergyFluence<T>, TransportError<T>> {
    surface.attenuate(attenuation.optical_depth(depth)?.transmission())
}
