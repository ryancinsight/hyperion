use aequitas::{
    systems::si::{
        dimensions,
        quantities::{EnergyPerVolume, Intensity, VolumetricPowerDensity},
    },
    unit::LinearUnit,
};
use eunomia::RealField;

use crate::{TransportError, ValueKind, validation};

/// Finite, non-negative fluence rate `φ` in `W/m²`.
///
/// The time derivative of energy fluence. Diffusion and radiative-transfer
/// solvers report this; time-integrated Monte Carlo reports
/// [`super::EnergyFluence`] instead.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FluenceRate<T>(Intensity<T>);

impl<T: RealField> FluenceRate<T> {
    /// Validate an intensity quantity as a fluence rate.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] when the canonical-SI value is
    /// negative or non-finite.
    pub fn new(quantity: Intensity<T>) -> Result<Self, TransportError<T>> {
        let value = validation::finite_non_negative(ValueKind::FluenceRate, quantity.into_base())?;
        Ok(Self(Intensity::from_base(value)))
    }

    /// Return the fluence rate expressed in linear intensity unit `U`.
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<dimensions::Intensity>,
    {
        self.0.in_unit::<U>()
    }
}

impl<T> FluenceRate<T> {
    /// Borrow the Aequitas quantity without conversion or copying.
    #[must_use]
    pub const fn quantity(&self) -> &Intensity<T> {
        &self.0
    }

    /// Move out the Aequitas quantity.
    #[must_use]
    pub fn into_quantity(self) -> Intensity<T> {
        self.0
    }
}

/// Finite, non-negative absorbed power density `Q` in `W/m³`.
///
/// The deposition quantity every energy-transport modality terminates in.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbsorbedPowerDensity<T>(VolumetricPowerDensity<T>);

impl<T: RealField> AbsorbedPowerDensity<T> {
    /// Validate a volumetric-power-density quantity as absorbed deposition.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] when the canonical-SI value is
    /// negative or non-finite.
    pub fn new(quantity: VolumetricPowerDensity<T>) -> Result<Self, TransportError<T>> {
        let value =
            validation::finite_non_negative(ValueKind::AbsorbedPowerDensity, quantity.into_base())?;
        Ok(Self(VolumetricPowerDensity::from_base(value)))
    }

    pub(crate) const fn from_validated(quantity: VolumetricPowerDensity<T>) -> Self {
        Self(quantity)
    }

    /// Return the deposition expressed in linear volumetric-power unit `U`.
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<dimensions::VolumetricPowerDensity>,
    {
        self.0.in_unit::<U>()
    }
}

impl<T> AbsorbedPowerDensity<T> {
    /// Borrow the Aequitas quantity without conversion or copying.
    #[must_use]
    pub const fn quantity(&self) -> &VolumetricPowerDensity<T> {
        &self.0
    }

    /// Move out the Aequitas quantity.
    #[must_use]
    pub fn into_quantity(self) -> VolumetricPowerDensity<T> {
        self.0
    }
}

/// Finite, non-negative absorbed energy density in `J/m³`.
///
/// The time-integrated counterpart of [`AbsorbedPowerDensity`].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbsorbedEnergyDensity<T>(EnergyPerVolume<T>);

impl<T: RealField> AbsorbedEnergyDensity<T> {
    /// Validate an energy-per-volume quantity as absorbed deposition.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidValue`] when the canonical-SI value is
    /// negative or non-finite.
    pub fn new(quantity: EnergyPerVolume<T>) -> Result<Self, TransportError<T>> {
        let value = validation::finite_non_negative(
            ValueKind::AbsorbedEnergyDensity,
            quantity.into_base(),
        )?;
        Ok(Self(EnergyPerVolume::from_base(value)))
    }

    pub(crate) const fn from_validated(quantity: EnergyPerVolume<T>) -> Self {
        Self(quantity)
    }

    /// Return the deposition expressed in linear energy-per-volume unit `U`.
    #[must_use]
    pub fn in_unit<U>(&self) -> T
    where
        U: LinearUnit<dimensions::EnergyPerVolume>,
    {
        self.0.in_unit::<U>()
    }
}

impl<T> AbsorbedEnergyDensity<T> {
    /// Borrow the Aequitas quantity without conversion or copying.
    #[must_use]
    pub const fn quantity(&self) -> &EnergyPerVolume<T> {
        &self.0
    }

    /// Move out the Aequitas quantity.
    #[must_use]
    pub fn into_quantity(self) -> EnergyPerVolume<T> {
        self.0
    }
}
