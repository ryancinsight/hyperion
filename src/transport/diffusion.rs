use aequitas::systems::si::quantities::{Dimensionless, Length, ReciprocalLength};
use eunomia::{NumericElement, RealField};

use crate::{
    TransportError, TransportLaw, ValueKind,
    coefficient::{
        Absorption, EffectiveAttenuation, InteractionCoefficient, ReducedScattering, Scattering,
        Transport,
    },
    quantity::{Anisotropy, PathLength, TransportAlbedo},
    validation,
};

/// Finite, positive diffuse-optics coefficient `D` with length dimension.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OpticalDiffusionCoefficient<T>(Length<T>);

impl<T: RealField> OpticalDiffusionCoefficient<T> {
    /// Validate a diffuse-optics coefficient.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] for a zero, negative, or
    /// non-finite canonical-SI value.
    pub fn new(quantity: Length<T>) -> Result<Self, TransportError<T>> {
        let value = validation::finite_positive(
            ValueKind::OpticalDiffusionCoefficient,
            quantity.into_base(),
        )?;
        Ok(Self(Length::from_base(value)))
    }
}

impl<T> OpticalDiffusionCoefficient<T> {
    /// Borrow the dimensional diffusion coefficient.
    #[must_use]
    pub const fn quantity(&self) -> &Length<T> {
        &self.0
    }

    /// Move out the dimensional diffusion coefficient.
    #[must_use]
    pub fn into_quantity(self) -> Length<T> {
        self.0
    }

    const fn from_validated(quantity: Length<T>) -> Self {
        Self(quantity)
    }
}

/// Validated coefficient pair for the optical diffusion approximation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DiffusionCoefficients<T> {
    absorption: InteractionCoefficient<T, Absorption>,
    reduced_scattering: InteractionCoefficient<T, ReducedScattering>,
}

impl<T: RealField> DiffusionCoefficients<T> {
    /// Construct a non-degenerate diffuse-optics coefficient pair.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DegenerateTransport`] when both coefficients
    /// are zero and [`TransportError::DerivedNonFinite`] when their sum
    /// overflows.
    pub fn new(
        absorption: InteractionCoefficient<T, Absorption>,
        reduced_scattering: InteractionCoefficient<T, ReducedScattering>,
    ) -> Result<Self, TransportError<T>> {
        let coefficients = Self {
            absorption,
            reduced_scattering,
        };
        coefficients.transport_value()?;
        Ok(coefficients)
    }

    /// Borrow the absorption coefficient.
    #[must_use]
    pub const fn absorption(&self) -> &InteractionCoefficient<T, Absorption> {
        &self.absorption
    }

    /// Borrow the reduced-scattering coefficient.
    #[must_use]
    pub const fn reduced_scattering(&self) -> &InteractionCoefficient<T, ReducedScattering> {
        &self.reduced_scattering
    }

    /// Return `mu_a + mu_s'`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the sum overflows.
    pub fn transport_coefficient(
        &self,
    ) -> Result<InteractionCoefficient<T, Transport>, TransportError<T>> {
        Ok(InteractionCoefficient::from_validated(
            ReciprocalLength::from_base(self.transport_value()?),
        ))
    }

    /// Return `D = 1 / (3 (mu_a + mu_s'))`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the result is not
    /// finite.
    pub fn diffusion_coefficient(
        &self,
    ) -> Result<OpticalDiffusionCoefficient<T>, TransportError<T>> {
        let denominator = T::from_f64(3.0) * self.transport_value()?;
        let value = denominator.recip();
        let valid = validation::derived_finite(TransportLaw::DiffusionCoefficient, value)?;
        Ok(OpticalDiffusionCoefficient::from_validated(
            Length::from_base(valid),
        ))
    }

    /// Return the reduced transport mean free path `1 / (mu_a + mu_s')`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the reciprocal is not
    /// finite.
    pub fn transport_mean_free_path(&self) -> Result<PathLength<T>, TransportError<T>> {
        let value = self.transport_value()?.recip();
        let valid = validation::derived_finite(TransportLaw::TransportMeanFreePath, value)?;
        Ok(PathLength::from_validated(Length::from_base(valid)))
    }

    /// Return `mu_eff = sqrt(3 mu_a (mu_a + mu_s'))`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the radicand or result
    /// is not finite.
    pub fn effective_attenuation(
        &self,
    ) -> Result<InteractionCoefficient<T, EffectiveAttenuation>, TransportError<T>> {
        let absorption = self.absorption.quantity().into_base();
        let radicand = T::from_f64(3.0) * absorption * self.transport_value()?;
        let finite_radicand =
            validation::derived_finite(TransportLaw::EffectiveAttenuation, radicand)?;
        let value = finite_radicand.sqrt();
        let valid = validation::derived_finite(TransportLaw::EffectiveAttenuation, value)?;
        Ok(InteractionCoefficient::from_validated(
            ReciprocalLength::from_base(valid),
        ))
    }

    /// Return `mu_s' / (mu_a + mu_s')`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::DerivedNonFinite`] when the division is not
    /// finite.
    pub fn transport_albedo(&self) -> Result<TransportAlbedo<T>, TransportError<T>> {
        let reduced = *self.reduced_scattering.quantity().as_base();
        let value = reduced / self.transport_value()?;
        let finite = validation::derived_finite(TransportLaw::TransportAlbedo, value)?;
        Ok(TransportAlbedo::from_validated(Dimensionless::from_base(
            finite,
        )))
    }

    fn transport_value(&self) -> Result<T, TransportError<T>> {
        let absorption = *self.absorption.quantity().as_base();
        let reduced = *self.reduced_scattering.quantity().as_base();
        let sum =
            validation::derived_finite(TransportLaw::TransportCoefficient, absorption + reduced)?;
        if sum == <T as NumericElement>::ZERO {
            Err(TransportError::DegenerateTransport)
        } else {
            Ok(sum)
        }
    }
}

/// Derive `mu_s' = mu_s (1 - g)` from scattering and anisotropy.
///
/// # Errors
///
/// Returns [`TransportError::DerivedNonFinite`] when multiplication overflows.
pub fn reduced_scattering<T: RealField>(
    scattering: InteractionCoefficient<T, Scattering>,
    anisotropy: Anisotropy<T>,
) -> Result<InteractionCoefficient<T, ReducedScattering>, TransportError<T>> {
    let factor = <T as NumericElement>::ONE - anisotropy.into_quantity().into_base();
    let value = scattering.into_quantity().into_base() * factor;
    let valid = validation::derived_finite(TransportLaw::ReducedScattering, value)?;
    Ok(InteractionCoefficient::from_validated(
        ReciprocalLength::from_base(valid),
    ))
}
