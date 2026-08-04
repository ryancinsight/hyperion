//! Differential check against the tables this module replaced.
//!
//! The values below are transcribed from the Kwavers `kwavers-optics`
//! chromophore database that ADR 0032 §4 retires. Asserting equality at every
//! tabulated wavelength is what makes the move a relocation rather than a
//! reimplementation: if any sample drifted in transit, this fails.

use super::{DEOXYHEMOGLOBIN, OXYHEMOGLOBIN, hemoglobin_absorption};
use crate::TransportError;

/// (wavelength nm, `HbO2` epsilon, Hb epsilon) as tabulated by `kwavers-optics`.
const KWAVERS_TABLE: &[(f64, f64, f64)] = &[
    (450.0, 251_264.0, 413_168.0),
    (475.0, 120_454.4, 60_193.6),
    (500.0, 83_731.2, 83_448.0),
    (525.0, 117_076.8, 137_590.4),
    (532.0, 175_504.0, 162_336.0),
    (550.0, 172_064.0, 213_648.0),
    (575.0, 196_688.0, 173_360.0),
    (600.0, 12_800.0, 58_708.8),
    (625.0, 3_096.0, 23_627.2),
    (650.0, 1_472.0, 15_000.48),
    (675.0, 1_142.4, 10_510.56),
    (700.0, 1_160.0, 7_177.12),
    (725.0, 1_560.0, 4_408.8),
    (750.0, 2_072.0, 5_620.96),
    (775.0, 2_708.8, 4_852.0),
    (800.0, 3_264.0, 3_046.88),
    (825.0, 3_825.6, 2_773.28),
    (850.0, 4_232.0, 2_765.28),
    (875.0, 4_550.4, 2_856.32),
    (900.0, 4_792.0, 3_047.36),
    (925.0, 4_907.2, 3_089.44),
    (950.0, 4_816.0, 2_408.96),
    (975.0, 4_576.0, 1_557.152),
    (1000.0, 4_096.0, 827.136),
];

#[test]
// Exact equality is the property under test: a sample hit must return the
// stored value unchanged, with no arithmetic applied. A tolerance here would
// let a transcription error through, which is the only thing this guards.
#[expect(clippy::float_cmp, reason = "table lookup must be bit-exact")]
fn every_tabulated_wavelength_matches_the_retired_kwavers_tables() {
    for &(wavelength, oxy, deoxy) in KWAVERS_TABLE {
        let got_oxy = OXYHEMOGLOBIN
            .molar_extinction::<f64>(wavelength)
            .expect("tabulated wavelength is in range");
        let got_deoxy = DEOXYHEMOGLOBIN
            .molar_extinction::<f64>(wavelength)
            .expect("tabulated wavelength is in range");
        // Exact: a sample hit returns the stored value, it is not interpolated.
        assert_eq!(got_oxy, oxy, "HbO2 at {wavelength} nm");
        assert_eq!(got_deoxy, deoxy, "Hb at {wavelength} nm");
    }
}

#[test]
fn interpolation_is_linear_between_neighbouring_samples() {
    // Midpoint of the 800-825 nm interval, where both curves are smooth.
    let midpoint = 812.5_f64;
    let got = OXYHEMOGLOBIN
        .molar_extinction::<f64>(midpoint)
        .expect("inside the table");
    let expected = f64::midpoint(3_264.0, 3_825.6);
    assert!(
        (got - expected).abs() < 1e-9,
        "expected {expected}, got {got}"
    );
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
