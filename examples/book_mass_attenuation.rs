//! Convert a mass attenuation coefficient to linear attenuation.

use aequitas::systems::si::{
    quantities::{AreaPerMass, MassDensity as DensityQuantity},
    units::{KilogramPerCubicMeter, Meter, PerMeter, SquareCentimeterPerGram},
};
use hyperion::coefficient::{InteractionCoefficient, LinearAttenuation, MassAttenuation};
use proteus::MassDensity;

fn main() -> Result<(), hyperion::TransportError<f64>> {
    // Illustrative tissue-style values: mu/rho = 0.02 cm^2/g and rho = 1000
    // kg/m^3 (= 1 g/cm^3), so the derived linear attenuation is
    // mu = (mu/rho) * rho = 2 m^-1.
    let mass_attenuation =
        MassAttenuation::new(AreaPerMass::from_unit::<SquareCentimeterPerGram>(0.02_f64))?;
    let density = MassDensity::new(DensityQuantity::from_unit::<KilogramPerCubicMeter>(
        1_000.0_f64,
    ))
    .expect("finite non-negative density");

    println!(
        "mu/rho = 0.02 cm^2/g, rho = {} kg/m^3",
        density.into_quantity().in_unit::<KilogramPerCubicMeter>()
    );

    let linear: InteractionCoefficient<f64, LinearAttenuation> =
        mass_attenuation.to_linear(density)?;
    let mu_per_meter = linear.in_unit::<PerMeter>();
    assert!(
        (mu_per_meter - 2.0).abs() < 1e-9,
        "expected mu = 2 m^-1, got {mu_per_meter}"
    );

    let half_value_layer = linear
        .half_value_layer()?
        .expect("non-zero coefficient has a finite half-value layer");
    let hvl_meters = half_value_layer.in_unit::<Meter>();
    assert!(
        (hvl_meters - core::f64::consts::LN_2 / 2.0).abs() < 1e-9,
        "expected HVL = ln(2)/2 m, got {hvl_meters}"
    );

    println!("linear attenuation: {mu_per_meter:.6} m^-1");
    println!("half-value layer: {hvl_meters:.6} m (ln 2 / mu)");
    Ok(())
}
