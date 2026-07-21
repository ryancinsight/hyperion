//! Algebraic and cross-provider transport-law evidence.

use crate::support::{anisotropy, assert_relative_close, coefficient, fluence, gamma, path};
use aequitas::systems::si::{
    quantities::{AreaPerMass, Dimensionless, MassDensity as DensityQuantity},
    units::{JoulePerSquareMeter, PerMeter, SquareCentimeterPerGram},
};
use eunomia::{NumericElement, RealField};
use hyperion::{
    TransportError, TransportLaw,
    coefficient::{
        Absorption, EffectiveAttenuation, InteractionCoefficient, LinearAttenuation,
        MassAttenuation, OpticalCoefficients, ReducedScattering, Scattering,
    },
    quantity::OpticalDepth,
    transport::{
        DiffusionCoefficients, planar_fluence_at_depth, reduced_scattering, total_optical_depth,
    },
};
use proteus::MassDensity;

fn assert_unreduced_optical_laws<T: RealField>() {
    let coefficients = OpticalCoefficients::new(
        coefficient::<T, Absorption>(2.0),
        coefficient::<T, Scattering>(20.0),
    )
    .expect("fixture total attenuation is finite");
    assert_eq!(
        coefficients
            .total_attenuation()
            .expect("fixture sum remains finite")
            .in_unit::<PerMeter>(),
        T::from_f64(22.0)
    );
    let mean_free_path = coefficients
        .mean_free_path()
        .expect("fixture reciprocal remains finite")
        .expect("non-vacuum medium has a finite mean free path")
        .into_quantity()
        .into_base();
    assert_relative_close(mean_free_path, T::from_f64(1.0 / 22.0), 2.0);
    let albedo = coefficients
        .single_scattering_albedo()
        .expect("fixture ratio remains finite")
        .expect("non-vacuum medium has a defined scattering albedo")
        .into_quantity()
        .into_base();
    assert_relative_close(albedo, T::from_f64(20.0 / 22.0), 2.0);

    let vacuum = OpticalCoefficients::new(
        coefficient::<T, Absorption>(0.0),
        coefficient::<T, Scattering>(0.0),
    )
    .expect("vacuum has finite zero coefficients");
    assert_eq!(
        vacuum
            .total_attenuation()
            .expect("vacuum total is finite")
            .in_unit::<PerMeter>(),
        <T as NumericElement>::ZERO
    );
    assert_eq!(
        vacuum
            .mean_free_path()
            .expect("vacuum is a defined boundary case"),
        None
    );
    assert_eq!(
        vacuum
            .single_scattering_albedo()
            .expect("vacuum is a defined boundary case"),
        None
    );
}

fn assert_reduced_and_diffusion_laws<T: RealField>() {
    let scattering = coefficient::<T, Scattering>(20.0);
    let isotropic = reduced_scattering(scattering, anisotropy(0.0))
        .expect("finite coefficients produce finite reduced scattering");
    assert_eq!(isotropic.in_unit::<PerMeter>(), T::from_f64(20.0));

    let forward = reduced_scattering(scattering, anisotropy(1.0))
        .expect("the forward-scattering endpoint is finite");
    assert_eq!(forward.in_unit::<PerMeter>(), <T as NumericElement>::ZERO);

    let backward = reduced_scattering(scattering, anisotropy(-1.0))
        .expect("the backward-scattering endpoint is finite");
    assert_eq!(backward.in_unit::<PerMeter>(), T::from_f64(40.0));

    let reduced_low = reduced_scattering(scattering, anisotropy(0.25))
        .expect("finite coefficients produce finite reduced scattering");
    let reduced_high = reduced_scattering(scattering, anisotropy(0.75))
        .expect("finite coefficients produce finite reduced scattering");
    assert!(
        reduced_high.in_unit::<PerMeter>() < reduced_low.in_unit::<PerMeter>(),
        "reduced scattering must decrease as anisotropy increases"
    );

    let absorption = coefficient::<T, Absorption>(2.0);
    assert_eq!(
        absorption
            .optical_depth(path(0.5))
            .expect("absorption optical depth is finite")
            .into_quantity()
            .into_base(),
        <T as NumericElement>::ONE
    );
    assert_eq!(
        scattering
            .optical_depth(path(0.05))
            .expect("scattering optical depth is finite")
            .into_quantity()
            .into_base(),
        <T as NumericElement>::ONE
    );
    let pair = DiffusionCoefficients::new(absorption, reduced_low)
        .expect("positive transport coefficient is non-degenerate");
    let transport = pair
        .transport_coefficient()
        .expect("fixture sum remains finite")
        .in_unit::<PerMeter>();
    assert_eq!(transport, T::from_f64(17.0));

    let diffusion = pair
        .diffusion_coefficient()
        .expect("fixture reciprocal remains finite")
        .into_quantity()
        .into_base();
    assert_relative_close(diffusion, T::from_f64(1.0 / 51.0), 3.0);
    let transport_mean_free_path = pair
        .transport_mean_free_path()
        .expect("fixture reciprocal remains finite")
        .into_quantity()
        .into_base();
    assert_relative_close(transport_mean_free_path, T::from_f64(1.0 / 17.0), 2.0);

    let effective = pair
        .effective_attenuation()
        .expect("fixture radicand and root remain finite");
    let effective_value = effective.in_unit::<PerMeter>();
    assert_relative_close(effective_value * effective_value, T::from_f64(102.0), 6.0);
    assert_relative_close(
        effective_value * effective_value,
        T::from_f64(2.0) / diffusion,
        8.0,
    );

    let albedo = pair
        .transport_albedo()
        .expect("positive transport coefficient gives finite transport albedo")
        .into_quantity()
        .into_base();
    assert_relative_close(albedo, T::from_f64(15.0 / 17.0), 2.0);
}

fn assert_beer_lambert_and_fluence_laws<T: RealField>() {
    let empty = total_optical_depth::<T, LinearAttenuation, _>(core::iter::empty())
        .expect("the empty reduction is the additive identity");
    assert_eq!(
        empty.into_quantity().into_base(),
        <T as NumericElement>::ZERO
    );

    let attenuation = coefficient::<T, LinearAttenuation>(2.0);
    let first = attenuation
        .optical_depth(path(0.25))
        .expect("fixture product is finite");
    let second = attenuation
        .optical_depth(path(0.75))
        .expect("fixture product is finite");
    let combined = total_optical_depth([(attenuation, path(0.25)), (attenuation, path(0.75))])
        .expect("fixture reduction is finite");
    assert_eq!(combined.into_quantity().into_base(), T::from_f64(2.0));

    let composed_transmission = first.transmission().into_quantity().into_base()
        * second.transmission().into_quantity().into_base();
    let combined_transmission = combined.transmission().into_quantity().into_base();
    // Eunomia routes both scalars through libm. The locked f32 exp polynomial
    // has approximation error below 2^-27.74 and the f64 implementation below
    // one ulp; gamma_32 covers their arithmetic plus this final product.
    assert_relative_close(composed_transmission, combined_transmission, 32.0);

    let zero_transmission =
        OpticalDepth::new(Dimensionless::from_base(<T as NumericElement>::ZERO))
            .expect("zero is a valid optical depth")
            .transmission()
            .into_quantity()
            .into_base();
    assert_eq!(zero_transmission, <T as NumericElement>::ONE);

    let pair = DiffusionCoefficients::new(
        coefficient::<T, Absorption>(2.0),
        coefficient::<T, ReducedScattering>(15.0),
    )
    .expect("positive transport coefficient is non-degenerate");
    let effective = pair
        .effective_attenuation()
        .expect("fixture radicand and root remain finite");
    let penetration = effective
        .penetration_depth()
        .expect("fixture reciprocal is finite")
        .expect("positive attenuation has finite penetration depth");
    let attenuated = planar_fluence_at_depth(fluence(12.0), effective, penetration)
        .expect("fixture fluence remains finite")
        .in_unit::<JoulePerSquareMeter>();
    assert_relative_close(attenuated, T::from_f64(12.0) / T::E, 32.0);

    let half_value = coefficient::<T, LinearAttenuation>(2.0)
        .half_value_layer()
        .expect("fixture reciprocal is finite")
        .expect("positive attenuation has a finite half-value layer");
    let half_transmission = attenuation
        .optical_depth(half_value)
        .expect("fixture product remains finite")
        .transmission()
        .into_quantity()
        .into_base();
    assert_relative_close(half_transmission, T::from_f64(0.5), 32.0);
}

#[test]
fn transport_laws_hold_for_every_supported_real_scalar() {
    assert_unreduced_optical_laws::<f32>();
    assert_unreduced_optical_laws::<f64>();
    assert_reduced_and_diffusion_laws::<f32>();
    assert_reduced_and_diffusion_laws::<f64>();
    assert_beer_lambert_and_fluence_laws::<f32>();
    assert_beer_lambert_and_fluence_laws::<f64>();
}

#[test]
fn mass_attenuation_composes_with_proteus_density_and_aequitas_units() {
    let mass_attenuation = MassAttenuation::new(AreaPerMass::from_unit::<SquareCentimeterPerGram>(
        0.07072_f64,
    ))
    .expect("NIST water fixture is finite and non-negative");
    let density = MassDensity::new(DensityQuantity::from_base(1_000.0_f64))
        .expect("water density is finite and non-negative");
    let linear = mass_attenuation
        .to_linear(density)
        .expect("fixture product remains finite");

    // 1 cm^2/g = 0.1 m^2/kg, hence 0.07072 cm^2/g * 1000 kg/m^3
    // equals 7.072 m^-1. The conversion and product are exact in binary only
    // up to three rounded operations, so gamma_3 bounds the result.
    assert_relative_close(linear.in_unit::<PerMeter>(), 7.072_f64, 3.0);
}

#[test]
fn degenerate_and_overflowing_laws_report_the_exact_failure() {
    let zero_absorption = coefficient::<f64, Absorption>(0.0);
    let zero_reduced =
        hyperion::coefficient::InteractionCoefficient::<
            f64,
            hyperion::coefficient::ReducedScattering,
        >::new(aequitas::systems::si::quantities::ReciprocalLength::from_base(0.0))
        .expect("zero reduced scattering is physically valid");
    assert_eq!(
        DiffusionCoefficients::new(zero_absorption, zero_reduced),
        Err(TransportError::DegenerateTransport)
    );

    let largest = InteractionCoefficient::<f64, Scattering>::new(
        aequitas::systems::si::quantities::ReciprocalLength::from_base(f64::MAX),
    )
    .expect("the largest finite coefficient passes the input boundary");
    let largest_absorption = InteractionCoefficient::<f64, Absorption>::new(
        aequitas::systems::si::quantities::ReciprocalLength::from_base(f64::MAX),
    )
    .expect("the largest finite coefficient passes the input boundary");
    let total_error = OpticalCoefficients::new(largest_absorption, largest)
        .expect_err("summing two largest finite coefficients must overflow");
    match total_error {
        TransportError::DerivedNonFinite { law, value } => {
            assert_eq!(law, TransportLaw::TotalAttenuation);
            assert!(value.is_infinite() && value.is_sign_positive());
        }
        other => panic!("expected total-attenuation overflow, got {other:?}"),
    }

    let largest_absorption = InteractionCoefficient::<f64, Absorption>::new(
        aequitas::systems::si::quantities::ReciprocalLength::from_base(f64::MAX),
    )
    .expect("the largest finite coefficient passes the input boundary");
    let largest_reduced = InteractionCoefficient::<f64, ReducedScattering>::new(
        aequitas::systems::si::quantities::ReciprocalLength::from_base(f64::MAX),
    )
    .expect("the largest finite coefficient passes the input boundary");
    let transport_error = DiffusionCoefficients::new(largest_absorption, largest_reduced)
        .expect_err("summing two largest finite coefficients must overflow");
    match transport_error {
        TransportError::DerivedNonFinite { law, value } => {
            assert_eq!(law, TransportLaw::TransportCoefficient);
            assert!(value.is_infinite() && value.is_sign_positive());
        }
        other => panic!("expected transport-coefficient overflow, got {other:?}"),
    }

    let error = reduced_scattering(largest, anisotropy::<f64>(-1.0))
        .expect_err("doubling the largest finite coefficient must overflow");
    match error {
        TransportError::DerivedNonFinite { law, value } => {
            assert_eq!(law, TransportLaw::ReducedScattering);
            assert!(value.is_infinite() && value.is_sign_positive());
        }
        other => panic!("expected reduced-scattering overflow, got {other:?}"),
    }

    let no_half_value = coefficient::<f64, LinearAttenuation>(0.0)
        .half_value_layer()
        .expect("zero attenuation is a defined boundary case");
    assert_eq!(no_half_value, None);
    let no_penetration = coefficient::<f64, EffectiveAttenuation>(0.0)
        .penetration_depth()
        .expect("zero attenuation is a defined boundary case");
    assert_eq!(no_penetration, None);
}

proptest::proptest! {
    #[test]
    fn reduced_scattering_matches_its_algebraic_law(
        scattering in 0.0_f64..1.0e6,
        anisotropy_value in -1.0_f64..=1.0,
    ) {
        let actual = reduced_scattering(
            coefficient::<f64, Scattering>(scattering),
            anisotropy(anisotropy_value),
        )
        .expect("bounded property inputs cannot overflow")
        .in_unit::<PerMeter>();
        let expected = scattering * (1.0 - anisotropy_value);
        // One subtraction and one multiplication in the law, plus one unit
        // conversion roundoff, give the gamma_3 relative bound.
        let magnitude = expected.abs().max(1.0);
        let bound = gamma::<f64>(3.0) * magnitude;
        proptest::prop_assert!(
            (actual - expected).abs() <= bound,
            "actual={actual}, expected={expected}, bound={bound}"
        );
    }

    #[test]
    fn optical_depth_is_additive_for_bounded_segments(
        attenuation_value in 0.0_f64..1.0e6,
        first_length in 0.0_f64..1.0e3,
        second_length in 0.0_f64..1.0e3,
    ) {
        let attenuation = coefficient::<f64, LinearAttenuation>(attenuation_value);
        let actual = total_optical_depth([
            (attenuation, path(first_length)),
            (attenuation, path(second_length)),
        ])
        .expect("bounded property inputs cannot overflow")
        .into_quantity()
        .into_base();
        let expected = attenuation_value * (first_length + second_length);
        // Two products, one sum, and the equivalent reference sum/product
        // give a gamma_5 comparison bound.
        let magnitude = expected.abs().max(1.0);
        let bound = gamma::<f64>(5.0) * magnitude;
        proptest::prop_assert!(
            (actual - expected).abs() <= bound,
            "actual={actual}, expected={expected}, bound={bound}"
        );
    }
}
