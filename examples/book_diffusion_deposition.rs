//! Derive diffuse-optics coefficients and evaluate local deposition laws.
//!
//! [`reduced_scattering`] turns a scattering coefficient and an anisotropy
//! into the reduced scattering `mu_s' = mu_s (1 - g)`.  [`DiffusionCoefficients`]
//! then holds the absorption/reduced-scattering pair and derives the transport
//! coefficient, the optical diffusion coefficient `D = 1 / (3 (mu_a + mu_s'))`,
//! the reduced transport mean free path, the effective attenuation, and the
//! reduced transport albedo.  Finally, the deposition laws
//! [`absorbed_power_density`] and [`absorbed_energy_density`] turn an
//! absorption coefficient acting on a fluence (rate) into the absorbed
//! deposition `Q = mu_a phi` / `q = mu_a Phi` that thermal models consume.

#![expect(clippy::print_stdout, reason = "example stdout is the deliverable")]

use aequitas::systems::si::{
    quantities::{Dimensionless, EnergyPerArea, Intensity, ReciprocalLength},
    units::{
        JoulePerCubicMeter, JoulePerSquareMeter, Meter, PerMeter, WattPerCubicMeter,
        WattPerSquareMeter,
    },
};
use hyperion::{
    TransportError,
    coefficient::{Absorption, EffectiveAttenuation, InteractionCoefficient, Scattering},
    quantity::{Anisotropy, EnergyFluence, FluenceRate},
    transport::{
        DiffusionCoefficients, absorbed_energy_density, absorbed_power_density, reduced_scattering,
    },
};

/// Assert `actual` is within a small relative tolerance of `expected`.
fn assert_close(actual: f64, expected: f64, what: &str) {
    let tolerance = 1e-9 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "{what}: expected {expected}, got {actual}"
    );
}

fn main() -> Result<(), TransportError<f64>> {
    // Tissue-style fixture: mu_a = 2 m^-1, mu_s = 20 m^-1, g = 0.25.
    let absorption_coefficient = InteractionCoefficient::<f64, Absorption>::new(
        ReciprocalLength::from_unit::<PerMeter>(2.0_f64),
    )?;
    let scattering = InteractionCoefficient::<f64, Scattering>::new(
        ReciprocalLength::from_unit::<PerMeter>(20.0_f64),
    )?;

    let anisotropy = Anisotropy::new(Dimensionless::from_base(0.25_f64))?;

    // mu_s' = mu_s (1 - g) = 20 * 0.75 = 15 m^-1.
    let reduced = reduced_scattering(scattering, anisotropy)?;
    let reduced_per_meter = reduced.in_unit::<PerMeter>();
    assert_close(reduced_per_meter, 15.0, "reduced scattering mu_s'");
    println!("mu_s' = {reduced_per_meter:.6} m^-1 (from mu_s = 20 m^-1, g = 0.25)");

    // mu_t = mu_a + mu_s' = 17 m^-1; the pair rejects the degenerate zero sum.
    let pair = DiffusionCoefficients::new(absorption_coefficient, reduced)?;
    let transport = pair.transport_coefficient()?.in_unit::<PerMeter>();
    assert_close(transport, 17.0, "transport coefficient");
    println!("mu_t = mu_a + mu_s' = {transport:.6} m^-1");

    // D = 1 / (3 * 17) = 1/51 m.
    let diffusion = pair
        .diffusion_coefficient()?
        .into_quantity()
        .in_unit::<Meter>();
    assert_close(diffusion, 1.0 / 51.0, "optical diffusion coefficient D");
    println!("D = 1 / (3 mu_t) = {diffusion:.9} m");

    // 1 / mu_t = 1/17 m.
    let transport_mean_free_path = pair.transport_mean_free_path()?.in_unit::<Meter>();
    assert_close(
        transport_mean_free_path,
        1.0 / 17.0,
        "transport mean free path",
    );
    println!("transport mean free path = {transport_mean_free_path:.9} m");

    // mu_eff = sqrt(3 mu_a mu_t) = sqrt(102) m^-1.
    let effective: InteractionCoefficient<f64, EffectiveAttenuation> =
        pair.effective_attenuation()?;
    let effective_per_meter = effective.in_unit::<PerMeter>();
    assert_close(effective_per_meter * effective_per_meter, 102.0, "mu_eff^2");
    println!("mu_eff = {effective_per_meter:.9} m^-1 (sqrt(3 mu_a mu_t))");

    // mu_s' / mu_t = 15/17.
    let transport_albedo = pair.transport_albedo()?.into_quantity().into_base();
    assert_close(transport_albedo, 15.0 / 17.0, "transport albedo");
    println!("transport albedo = mu_s' / mu_t = {transport_albedo:.9}");

    // Deposition: Q = mu_a phi with phi = 2000 W/m^2 and mu_a = 0.5 m^-1.
    let rate = FluenceRate::new(Intensity::from_unit::<WattPerSquareMeter>(2_000.0_f64))?;
    let power = absorbed_power_density(absorption(0.5_f64), rate)?;
    let watts_per_cubic_meter = power.in_unit::<WattPerCubicMeter>();
    assert_close(watts_per_cubic_meter, 1_000.0, "absorbed power density Q");
    println!("Q = mu_a phi = {watts_per_cubic_meter:.3} W/m^3");

    // Deposition: q = mu_a Phi with Phi = 40 J/m^2 and mu_a = 0.25 m^-1.
    let fluence = EnergyFluence::new(EnergyPerArea::from_unit::<JoulePerSquareMeter>(40.0_f64))?;
    let energy = absorbed_energy_density(absorption(0.25_f64), fluence)?;
    let joules_per_cubic_meter = energy.in_unit::<JoulePerCubicMeter>();
    assert_close(joules_per_cubic_meter, 10.0, "absorbed energy density q");
    println!("q = mu_a Phi = {joules_per_cubic_meter:.3} J/m^3");

    println!("all diffusion and deposition assertions passed");
    Ok(())
}

/// Helper: a fresh absorption coefficient in m^-1 (deposition laws take the
/// coefficient by value, so the pair above cannot be reused).
fn absorption(per_meter: f64) -> InteractionCoefficient<f64, Absorption> {
    InteractionCoefficient::new(ReciprocalLength::from_unit::<PerMeter>(per_meter))
        .expect("invariant: non-negative finite absorption coefficient")
}
