//! Evaluate a typed diffuse-optics transport path.

use aequitas::systems::si::{
    quantities::{EnergyPerArea, Length, ReciprocalLength},
    units::{JoulePerSquareMeter, Meter, PerMeter},
};
use hyperion::{
    coefficient::{EffectiveAttenuation, InteractionCoefficient},
    quantity::{EnergyFluence, PathLength},
    transport::planar_fluence_at_depth,
};

fn main() -> Result<(), hyperion::TransportError<f64>> {
    let attenuation =
        InteractionCoefficient::<_, EffectiveAttenuation>::new(ReciprocalLength::from_unit::<
            PerMeter,
        >(2.0_f64))?;
    let surface = EnergyFluence::new(EnergyPerArea::from_unit::<JoulePerSquareMeter>(10.0_f64))?;
    let depth = PathLength::new(Length::from_unit::<Meter>(0.5_f64))?;
    let fluence = planar_fluence_at_depth(surface, attenuation, depth)?;

    println!(
        "fluence at 0.5 m: {:.9} J/m^2",
        fluence.in_unit::<JoulePerSquareMeter>()
    );
    Ok(())
}
