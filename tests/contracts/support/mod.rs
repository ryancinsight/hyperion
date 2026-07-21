use aequitas::systems::si::{
    quantities::{Dimensionless, Energy, EnergyPerArea, Length, ReciprocalLength},
    units::{JoulePerSquareMeter, MegaElectronVolt, Meter, PerMeter},
};
use eunomia::{NumericElement, RealField};
use hyperion::{
    coefficient::{CoefficientRole, InteractionCoefficient},
    quantity::{Anisotropy, EnergyFluence, PathLength, PhotonEnergy},
};

pub fn coefficient<T: RealField, Role: CoefficientRole>(
    value_per_meter: f64,
) -> InteractionCoefficient<T, Role> {
    InteractionCoefficient::new(ReciprocalLength::from_unit::<PerMeter>(T::from_f64(
        value_per_meter,
    )))
    .expect("fixture coefficient satisfies the passive-medium contract")
}

pub fn anisotropy<T: RealField>(value: f64) -> Anisotropy<T> {
    Anisotropy::new(Dimensionless::from_base(T::from_f64(value)))
        .expect("fixture anisotropy lies in the closed physical interval")
}

pub fn path<T: RealField>(meters: f64) -> PathLength<T> {
    PathLength::new(Length::from_unit::<Meter>(T::from_f64(meters)))
        .expect("fixture path is finite and non-negative")
}

pub fn photon_energy<T: RealField>(megaelectronvolts: f64) -> PhotonEnergy<T> {
    PhotonEnergy::new(Energy::from_unit::<MegaElectronVolt>(T::from_f64(
        megaelectronvolts,
    )))
    .expect("fixture photon energy is finite and positive")
}

pub fn fluence<T: RealField>(joules_per_square_meter: f64) -> EnergyFluence<T> {
    EnergyFluence::new(EnergyPerArea::from_unit::<JoulePerSquareMeter>(
        T::from_f64(joules_per_square_meter),
    ))
    .expect("fixture fluence is finite and non-negative")
}

/// Higham's standard `gamma_n` bound for `n` correctly rounded operations.
pub fn gamma<T: RealField>(operations: f64) -> T {
    let n = T::from_f64(operations);
    n * T::EPSILON / (<T as NumericElement>::ONE - n * T::EPSILON)
}

pub fn assert_relative_close<T: RealField>(actual: T, expected: T, operations: f64) {
    let magnitude = if expected.abs() > <T as NumericElement>::ONE {
        expected.abs()
    } else {
        <T as NumericElement>::ONE
    };
    let bound = gamma::<T>(operations) * magnitude;
    assert!(
        (actual - expected).abs() <= bound,
        "actual={actual:?}, expected={expected:?}, bound={bound:?}"
    );
}
