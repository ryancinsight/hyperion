use aequitas::systems::si::quantities::{Dimensionless, Length, ReciprocalLength};
use eunomia::{NumericElement, RealField};

use super::{Absorption, InteractionCoefficient, LinearAttenuation, Scattering};
use crate::{
    TransportError, TransportLaw,
    quantity::{PathLength, SingleScatteringAlbedo},
    validation,
};

/// Validated absorption and unreduced scattering coefficients for one medium.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpticalCoefficients<T> {
    absorption: InteractionCoefficient<T, Absorption>,
    scattering: InteractionCoefficient<T, Scattering>,
}

impl<T: RealField> OpticalCoefficients<T> {
    /// Construct an optical coefficient pair whose sum is finite.
    ///
    /// Vacuum, where both coefficients are zero, is valid. Its ordinary mean
    /// free path and single-scattering albedo are undefined and return `None`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when `mu_a + mu_s`
    /// overflows.
    pub fn new(
        absorption: InteractionCoefficient<T, Absorption>,
        scattering: InteractionCoefficient<T, Scattering>,
    ) -> Result<Self, TransportError<T>> {
        let coefficients = Self {
            absorption,
            scattering,
        };
        coefficients.total_value()?;
        Ok(coefficients)
    }

    /// Borrow the absorption coefficient.
    #[must_use]
    pub const fn absorption(&self) -> &InteractionCoefficient<T, Absorption> {
        &self.absorption
    }

    /// Borrow the unreduced scattering coefficient.
    #[must_use]
    pub const fn scattering(&self) -> &InteractionCoefficient<T, Scattering> {
        &self.scattering
    }

    /// Return total attenuation `mu_t = mu_a + mu_s`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the sum is not finite.
    pub fn total_attenuation(
        &self,
    ) -> Result<InteractionCoefficient<T, LinearAttenuation>, TransportError<T>> {
        Ok(InteractionCoefficient::from_validated(
            ReciprocalLength::from_base(self.total_value()?),
        ))
    }

    /// Return the ordinary mean free path `1 / (mu_a + mu_s)`.
    ///
    /// Vacuum returns `None` because it has no finite interaction length.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the reciprocal is not
    /// finite.
    pub fn mean_free_path(&self) -> Result<Option<PathLength<T>>, TransportError<T>> {
        let total = self.total_value()?;
        if total == <T as NumericElement>::ZERO {
            return Ok(None);
        }
        let value = validation::derived_finite(TransportLaw::MeanFreePath, total.recip())?;
        Ok(Some(PathLength::from_validated(Length::from_base(value))))
    }

    /// Return ordinary single-scattering albedo `mu_s / (mu_a + mu_s)`.
    ///
    /// Vacuum returns `None` because the ratio is `0 / 0`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the ratio is not
    /// finite.
    pub fn single_scattering_albedo(
        &self,
    ) -> Result<Option<SingleScatteringAlbedo<T>>, TransportError<T>> {
        let total = self.total_value()?;
        if total == <T as NumericElement>::ZERO {
            return Ok(None);
        }
        let scattering = *self.scattering.quantity().as_base();
        let value =
            validation::derived_finite(TransportLaw::SingleScatteringAlbedo, scattering / total)?;
        Ok(Some(SingleScatteringAlbedo::from_validated(
            Dimensionless::from_base(value),
        )))
    }

    fn total_value(&self) -> Result<T, TransportError<T>> {
        let absorption = *self.absorption.quantity().as_base();
        let scattering = *self.scattering.quantity().as_base();
        validation::derived_finite(TransportLaw::TotalAttenuation, absorption + scattering)
    }
}
