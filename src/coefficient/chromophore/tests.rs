//! Differential check against the independent Prahl/OMLC source table.
//!
//! The source table is available at
//! <https://omlc.org/spectra/hemoglobin/summary.html> (retrieved 2026-08-20).
//! Its preformatted `lambda`/`Hb02`/`Hb` columns report molar extinction per
//! hemoglobin molecule using 64,500 g/mol. The fixture below is independently
//! transcribed from those source rows rather than derived from the production
//! slices; the provider knots therefore must not carry a second factor of four.

use super::{DEOXYHEMOGLOBIN, OXYHEMOGLOBIN, hemoglobin_absorption};
use crate::TransportError;

/// (wavelength nm, `HbO2` epsilon, Hb epsilon) from independently reread OMLC rows.
const PRAHL_OMLC_SOURCE_SAMPLES: &[(u16, f64, f64)] = &[
    (450, 62_816.0, 103_292.0),
    (474, 30_113.6, 15_048.4),
    (500, 20_932.8, 20_862.0),
    (524, 29_269.2, 34_397.6),
    (532, 43_876.0, 40_584.0),
    (550, 43_016.0, 53_412.0),
    (574, 53_308.0, 41_716.0),
    (600, 3_200.0, 14_677.2),
    (624, 774.0, 5_906.8),
    (650, 368.0, 3_750.12),
    (674, 285.6, 2_627.64),
    (700, 290.0, 1_794.28),
    (724, 364.0, 1_244.44),
    (750, 518.0, 1_405.24),
    (774, 677.2, 1_213.0),
    (800, 816.0, 761.72),
    (824, 944.8, 693.48),
    (850, 1_058.0, 691.32),
    (874, 1_137.6, 714.08),
    (900, 1_198.0, 761.84),
    (924, 1_227.2, 776.64),
    (950, 1_204.0, 602.24),
    (974, 1_150.8, 402.28),
    (1000, 1_024.0, 206.784),
];

#[test]
// Exact equality is the property under test: a sample hit must return the
// stored value unchanged, with no arithmetic applied. A tolerance here would
// let a transcription error through, which is the only thing this guards.
#[expect(clippy::float_cmp, reason = "source knots must be bit-exact")]
fn every_tabulated_wavelength_matches_the_prahl_omlc_source() {
    for &(wavelength, oxy, deoxy) in PRAHL_OMLC_SOURCE_SAMPLES {
        let got_oxy = OXYHEMOGLOBIN
            .molar_extinction::<f64>(f64::from(wavelength))
            .expect("tabulated wavelength is in range");
        let got_deoxy = DEOXYHEMOGLOBIN
            .molar_extinction::<f64>(f64::from(wavelength))
            .expect("tabulated wavelength is in range");
        // Exact: a sample hit returns the stored value, it is not interpolated.
        assert_eq!(got_oxy, oxy, "HbO2 at {wavelength} nm");
        assert_eq!(got_deoxy, deoxy, "Hb at {wavelength} nm");
    }
}

#[test]
fn interpolation_is_linear_between_neighbouring_samples() {
    // Midpoint of the 800-824 nm interval, where both curves are smooth.
    let midpoint = 812.0_f64;
    let got = OXYHEMOGLOBIN
        .molar_extinction::<f64>(midpoint)
        .expect("inside the table");
    let expected = f64::midpoint(816.0, 944.8);
    assert!(
        (got - expected).abs() < 1e-9,
        "expected {expected}, got {got}"
    );
}

#[test]
fn interpolation_is_monomorphized_for_f32_and_f64() {
    let f32_value = OXYHEMOGLOBIN
        .molar_extinction::<f32>(812.0)
        .expect("f32 query is inside the table");
    let f64_value = OXYHEMOGLOBIN
        .molar_extinction::<f64>(812.0)
        .expect("f64 query is inside the table");

    assert!(f32_value.is_finite());
    assert!(f64_value.is_finite());
    assert!((f64::from(f32_value) - f64_value).abs() < 0.01);
}

#[test]
fn negative_concentration_is_rejected_at_the_coefficient_boundary() {
    // Validate each concentration before combining it; a positive counterpart
    // must not mask an invalid negative input.
    let result = hemoglobin_absorption::<f64>(800.0, -1.0e-3, 1.0e-3);
    assert!(matches!(
        result,
        Err(TransportError::InvalidValue {
            field: crate::ValueKind::ChromophoreConcentration,
            constraint: crate::ValueConstraint::FiniteNonNegative,
            ..
        })
    ));
}

#[test]
fn sub_nanometre_queries_are_not_quantised() {
    // The retired implementation rounded the query to whole nanometres before
    // interpolating, so these two would have returned the same value.
    let low = DEOXYHEMOGLOBIN
        .molar_extinction::<f64>(600.2)
        .expect("inside the table");
    let high = DEOXYHEMOGLOBIN
        .molar_extinction::<f64>(600.8)
        .expect("inside the table");
    assert!(
        low > high,
        "Hb falls steeply past 600 nm; {low} should exceed {high}"
    );
}

#[test]
fn wavelengths_outside_the_measured_range_are_rejected() {
    let below = OXYHEMOGLOBIN.molar_extinction::<f64>(400.0);
    assert!(matches!(
        below,
        Err(TransportError::WavelengthOutOfRange { .. })
    ));
    let above = OXYHEMOGLOBIN.molar_extinction::<f64>(1100.0);
    assert!(matches!(
        above,
        Err(TransportError::WavelengthOutOfRange { .. })
    ));
}

#[test]
fn absorption_follows_beer_lambert_at_an_isosbestic_point() {
    // Near 800 nm the two curves nearly cross, so an even oxy/deoxy split
    // should land between the pure-oxy and pure-deoxy coefficients.
    let concentration = 1.0e-3_f64;
    let mixed = hemoglobin_absorption::<f64>(800.0, concentration / 2.0, concentration / 2.0)
        .expect("valid inputs");
    let oxy_only = hemoglobin_absorption::<f64>(800.0, concentration, 0.0).expect("valid inputs");
    let deoxy_only = hemoglobin_absorption::<f64>(800.0, 0.0, concentration).expect("valid inputs");

    let mixed_value = mixed.in_unit::<aequitas::systems::si::units::PerMeter>();
    let oxy_value = oxy_only.in_unit::<aequitas::systems::si::units::PerMeter>();
    let deoxy_value = deoxy_only.in_unit::<aequitas::systems::si::units::PerMeter>();

    let (lower, upper) = if oxy_value <= deoxy_value {
        (oxy_value, deoxy_value)
    } else {
        (deoxy_value, oxy_value)
    };
    assert!(
        mixed_value >= lower && mixed_value <= upper,
        "mixed {mixed_value} outside [{lower}, {upper}]"
    );
}
