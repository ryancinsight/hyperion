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
