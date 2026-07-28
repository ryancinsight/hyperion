//! Local absorbed-deposition laws.
//!
//! Absorption acting on a fluence gives the energy deposited per unit volume:
//! `Q = μ_a φ` for a rate and `q = μ_a Φ` for a time-integrated fluence. Both
//! are single products, but they are the point where an optical, radiofrequency,
//! or radiation transport result becomes a source term for a thermal or damage
//! model, so they belong to one owner rather than to each consumer.

use eunomia::{RealField, UnitScalar};

use crate::{
    TransportError, TransportLaw,
    coefficient::{Absorption, InteractionCoefficient},
    quantity::{AbsorbedEnergyDensity, AbsorbedPowerDensity, EnergyFluence, FluenceRate},
    validation,
};

/// Absorbed power density `Q = μ_a φ`, in `W/m³`.
///
/// # Errors
///
/// Returns [`TransportError::DerivedNonFinite`] when the product overflows to a
/// non-finite value.
pub fn absorbed_power_density<T: RealField + UnitScalar>(
    absorption: InteractionCoefficient<T, Absorption>,
    fluence_rate: FluenceRate<T>,
) -> Result<AbsorbedPowerDensity<T>, TransportError<T>> {
    let product = *absorption.quantity() * fluence_rate.into_quantity();
    let value = validation::derived_finite(TransportLaw::AbsorbedDeposition, product.into_base())?;
    Ok(AbsorbedPowerDensity::from_validated(
        aequitas::systems::si::quantities::VolumetricPowerDensity::from_base(value),
    ))
}

/// Absorbed energy density `q = μ_a Φ`, in `J/m³`.
///
/// # Errors
///
/// Returns [`TransportError::DerivedNonFinite`] when the product overflows to a
/// non-finite value.
pub fn absorbed_energy_density<T: RealField + UnitScalar>(
    absorption: InteractionCoefficient<T, Absorption>,
    fluence: EnergyFluence<T>,
) -> Result<AbsorbedEnergyDensity<T>, TransportError<T>> {
    let product = *absorption.quantity() * fluence.into_quantity();
    let value = validation::derived_finite(TransportLaw::AbsorbedDeposition, product.into_base())?;
    Ok(AbsorbedEnergyDensity::from_validated(
        aequitas::systems::si::quantities::EnergyPerVolume::from_base(value),
    ))
}

#[cfg(test)]
mod tests {
    use super::{absorbed_energy_density, absorbed_power_density};
    use crate::{
        coefficient::{Absorption, InteractionCoefficient},
        quantity::{EnergyFluence, FluenceRate},
    };
    use aequitas::systems::si::{
        quantities::{EnergyPerArea, Intensity, ReciprocalLength},
        units::{
            JoulePerCubicMeter, JoulePerSquareMeter, PerMeter, WattPerCubicMeter,
            WattPerSquareMeter,
        },
    };

    fn absorption(per_meter: f64) -> InteractionCoefficient<f64, Absorption> {
        InteractionCoefficient::new(ReciprocalLength::from_unit::<PerMeter>(per_meter))
            .expect("invariant: non-negative finite absorption coefficient")
    }

    #[test]
    fn power_deposition_is_absorption_times_fluence_rate() {
        let rate = FluenceRate::new(Intensity::from_unit::<WattPerSquareMeter>(2_000.0))
            .expect("invariant: non-negative finite fluence rate");

        let deposition = absorbed_power_density(absorption(0.5), rate)
            .expect("invariant: finite product of valid factors");

        assert_eq!(
            deposition.in_unit::<WattPerCubicMeter>().to_bits(),
            1_000.0_f64.to_bits()
        );
    }

    #[test]
    fn energy_deposition_is_absorption_times_energy_fluence() {
        let fluence = EnergyFluence::new(EnergyPerArea::from_unit::<JoulePerSquareMeter>(40.0))
            .expect("invariant: non-negative finite energy fluence");

        let deposition = absorbed_energy_density(absorption(0.25), fluence)
            .expect("invariant: finite product of valid factors");

        assert_eq!(
            deposition.in_unit::<JoulePerCubicMeter>().to_bits(),
            10.0_f64.to_bits()
        );
    }

    /// Beer-Lambert deposition in a purely absorbing slab integrates to the
    /// energy the beam gives up: `∫₀^∞ μ_a Φ₀ e^{-μ_a x} dx = Φ₀`. The midpoint
    /// rule over a fine grid approaches that within its own O(h²) error, which
    /// is an oracle independent of the product this module computes.
    #[test]
    fn deposition_integrates_to_the_incident_fluence() {
        let mu_a = 2.0_f64;
        let incident = 10.0_f64;
        // `u32` so the loop counter converts to `f64` losslessly.
        let steps = 100_000_u32;
        let extent = 20.0 / mu_a;
        let h = extent / f64::from(steps);

        let mut total = 0.0_f64;
        for step in 0..steps {
            let depth = (f64::from(step) + 0.5) * h;
            let local = EnergyFluence::new(EnergyPerArea::from_unit::<JoulePerSquareMeter>(
                incident * (-mu_a * depth).exp(),
            ))
            .expect("invariant: non-negative finite energy fluence");
            let deposition = absorbed_energy_density(absorption(mu_a), local)
                .expect("invariant: finite product of valid factors");
            total += deposition.in_unit::<JoulePerCubicMeter>() * h;
        }

        // Truncated tail e^{-20} plus midpoint error (μ_a h)²/24 per step.
        let tail = incident * (-20.0_f64).exp();
        let midpoint = incident * (mu_a * h).powi(2) / 24.0;
        let tolerance = tail + midpoint + incident * 8.0 * f64::EPSILON;

        assert!(
            (total - incident).abs() <= tolerance,
            "integrated deposition {total} is not within {tolerance} of {incident}"
        );
    }
}
