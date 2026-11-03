//! Representation and allocation invariants.
//!
//! Allocation windows count allocations made by the measuring thread only. A
//! process-wide counter is invalid here: libtest runs each test body on a
//! spawned thread while its main thread keeps inserting the running test into
//! its bookkeeping collections, so a process-wide window can absorb those
//! allocations and fail for reasons unrelated to the code under measurement.

use core::mem::{align_of, size_of};
use std::alloc::System;

use crate::support::{coefficient, fluence, path};
use aequitas::systems::si::quantities::{
    AreaPerMass, Dimensionless, Energy, EnergyPerArea, Length, ReciprocalLength,
};
use hyperion::{
    coefficient::{Absorption, EffectiveAttenuation, InteractionCoefficient, MassAttenuation},
    quantity::{
        Anisotropy, EnergyFluence, OpticalDepth, PathLength, PhotonEnergy, SingleScatteringAlbedo,
        Transmission, TransportAlbedo,
    },
    transport::{DiffusionCoefficients, OpticalDiffusionCoefficient, planar_fluence_at_depth},
};
use mnemosyne::counting::{AllocationDelta, CountingAllocator, measure};

#[global_allocator]
static ALLOCATOR: CountingAllocator<System> = CountingAllocator::new(System);

#[test]
fn transparent_domain_types_preserve_quantity_layout() {
    assert_eq!(size_of::<Absorption>(), 0);
    assert_eq!(
        size_of::<InteractionCoefficient<f64, Absorption>>(),
        size_of::<ReciprocalLength<f64>>()
    );
    assert_eq!(
        align_of::<InteractionCoefficient<f64, Absorption>>(),
        align_of::<ReciprocalLength<f64>>()
    );
    assert_eq!(
        size_of::<MassAttenuation<f32>>(),
        size_of::<AreaPerMass<f32>>()
    );
    assert_eq!(
        size_of::<Anisotropy<f64>>(),
        size_of::<Dimensionless<f64>>()
    );
    assert_eq!(
        size_of::<OpticalDepth<f32>>(),
        size_of::<Dimensionless<f32>>()
    );
    assert_eq!(
        size_of::<Transmission<f64>>(),
        size_of::<Dimensionless<f64>>()
    );
    assert_eq!(
        size_of::<SingleScatteringAlbedo<f64>>(),
        size_of::<Dimensionless<f64>>()
    );
    assert_eq!(
        size_of::<TransportAlbedo<f32>>(),
        size_of::<Dimensionless<f32>>()
    );
    assert_eq!(size_of::<PathLength<f64>>(), size_of::<Length<f64>>());
    assert_eq!(size_of::<PhotonEnergy<f32>>(), size_of::<Energy<f32>>());
    assert_eq!(
        size_of::<EnergyFluence<f64>>(),
        size_of::<EnergyPerArea<f64>>()
    );
    assert_eq!(
        size_of::<OpticalDiffusionCoefficient<f32>>(),
        size_of::<Length<f32>>()
    );
}

#[test]
fn representative_transport_path_allocates_nothing() {
    let absorption = coefficient::<f64, Absorption>(2.0);
    let reduced = coefficient(15.0);
    let surface = fluence(12.0);
    let depth = path(0.25);

    let (result, change) = measure(|| {
        let pair = DiffusionCoefficients::new(absorption, reduced)
            .expect("fixture transport coefficient is positive");
        let attenuation: InteractionCoefficient<f64, EffectiveAttenuation> = pair
            .effective_attenuation()
            .expect("fixture attenuation is finite");
        planar_fluence_at_depth(surface, attenuation, depth)
            .expect("fixture fluence remains finite")
    });

    assert!(result.into_quantity().into_base().is_finite());
    assert_eq!(change, AllocationDelta::default());
}

#[test]
fn subnormal_transmission_recovery_allocates_nothing() {
    let single = EnergyFluence::new(EnergyPerArea::from_base(f32::from_bits(0x7b80_0000)))
        .expect("finite incident single-precision fluence");
    let double = EnergyFluence::new(EnergyPerArea::from_base(f64::from_bits(
        0x7e70_0000_0000_0000,
    )))
    .expect("finite incident double-precision fluence");
    let unit_single = coefficient::<f32, EffectiveAttenuation>(1.0);
    let unit_double = coefficient::<f64, EffectiveAttenuation>(1.0);
    let single_depth = path(128.0);
    let double_depth = path(1024.0);

    let ((single, double), change) = measure(|| {
        (
            planar_fluence_at_depth(single, unit_single, single_depth)
                .expect("finite recovered single-precision fluence"),
            planar_fluence_at_depth(double, unit_double, double_depth)
                .expect("finite recovered double-precision fluence"),
        )
    });

    let single_bits = single.into_quantity().into_base().to_bits();
    let double_bits = double.into_quantity().into_base().to_bits();
    assert!(single_bits.abs_diff(0x1f21_75d0) <= 16);
    assert!(double_bits.abs_diff(0x2219_a3a1_32ee_86ba) <= 16);
    assert_eq!(change, AllocationDelta::default());
}
