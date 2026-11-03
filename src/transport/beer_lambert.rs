use aequitas::systems::si::quantities::{Dimensionless, EnergyPerArea};
use eunomia::{NumericElement, RealField, UnitScalar};

use crate::{
    TransportError, TransportLaw,
    coefficient::{
        AttenuatingRole, EffectiveAttenuation, InteractionCoefficient, LinearAttenuation,
    },
    quantity::{EnergyFluence, OpticalDepth, PathLength},
    validation,
};

impl<T: RealField + UnitScalar, Role: AttenuatingRole> InteractionCoefficient<T, Role> {
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
        Ok(Some(validation::finite_reciprocal_path(
            TransportLaw::HalfValueLayer,
            T::LN_2 / coefficient,
        )?))
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
        Ok(Some(validation::finite_reciprocal_path(
            TransportLaw::PenetrationDepth,
            coefficient.recip(),
        )?))
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
    T: RealField + UnitScalar,
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
/// If the transmission is subnormal or zero, evaluate the combined product
/// with four exponential factors to recover normal fluence lost by premature
/// transmission underflow. Final subnormal rounding, including the zero
/// transition, still carries floating-point error; correct rounding is not
/// guaranteed. Ordinary transmission retains its native multiplication path;
/// the standalone transmission API is unchanged.
///
/// # Errors
///
/// Returns [`TransportError::DerivedNonFinite`] when optical depth or the
/// attenuated fluence becomes non-finite.
pub fn planar_fluence_at_depth<T: RealField + UnitScalar>(
    surface: EnergyFluence<T>,
    attenuation: InteractionCoefficient<T, EffectiveAttenuation>,
    depth: PathLength<T>,
) -> Result<EnergyFluence<T>, TransportError<T>> {
    let optical_depth = attenuation.optical_depth(depth)?;
    let transmission = optical_depth.transmission();
    let incident = surface.into_quantity().into_base();
    if transmission.into_quantity().into_base() < validation::minimum_normal::<T>()
        && incident > T::ZERO
    {
        let value = small_transmission_fluence(incident, optical_depth.into_quantity().into_base());
        let valid = validation::derived_finite(TransportLaw::PlanarFluence, value)?;
        Ok(EnergyFluence::from_validated(EnergyPerArea::from_base(
            valid,
        )))
    } else {
        surface.attenuate(transmission)
    }
}

/// Split `exp(-tau)` into four normal factors whenever the final binary32/64
/// product can round nonzero: `tau <= ln(max_finite / (min_subnormal/2))`.
/// A quarter of that bound is below `-ln(min_normal)` in both formats. Division
/// by four is exact in this range. Exact intermediate magnitudes are no smaller
/// than the exact final product; native final-subnormal rounding is not assured.
/// No logarithm of the incident fluence or cross-precision arithmetic is used.
#[cold]
#[inline(never)]
fn small_transmission_fluence<T: RealField>(incident: T, optical_depth: T) -> T {
    let two = T::ONE + T::ONE;
    let factor = (-optical_depth / (two + two)).exp();
    (((incident * factor) * factor) * factor) * factor
}
