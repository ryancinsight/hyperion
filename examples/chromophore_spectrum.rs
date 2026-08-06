//! Evaluate a typed hemoglobin spectrum at a sub-nanometre wavelength.

use aequitas::systems::si::units::PerMeter;
use hyperion::coefficient::{hemoglobin_absorption, OXYHEMOGLOBIN};

fn main() -> Result<(), hyperion::TransportError<f64>> {
    let wavelength_nm = 812.5_f64;
    let extinction = OXYHEMOGLOBIN.molar_extinction(wavelength_nm)?;
    let absorption = hemoglobin_absorption(wavelength_nm, 5.0e-4, 5.0e-4)?;

    println!("HbO₂ extinction at {wavelength_nm:.1} nm: {extinction:.3} M⁻¹·cm⁻¹");
    println!(
        "mixed hemoglobin absorption: {:.3} m⁻¹",
        absorption.in_unit::<PerMeter>()
    );
    Ok(())
}
