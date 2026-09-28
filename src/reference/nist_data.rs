pub(super) const KNOT_COUNT: usize = 28;
pub(super) const MINIMUM_ENERGY_MEV: f64 = 0.01;
pub(super) const MAXIMUM_ENERGY_MEV: f64 = 20.0;

pub(super) const PHOTON_ENERGY_MEV: [f64; KNOT_COUNT] = [
    0.01, 0.015, 0.02, 0.03, 0.04, 0.05, 0.06, 0.08, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.8, 1.0,
    1.25, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 15.0, 20.0,
];

/// Knot energies in canonical SI base units (joules). Every table shares the
/// same 0.01–20 `MeV` knot grid.
pub(super) const KNOTS_BASE_JOULE: [f64; KNOT_COUNT] = [
    1.602_176_634e-15,
    2.403_264_951e-15,
    3.204_353_268e-15,
    4.806_529_902e-15,
    6.408_706_536e-15,
    8.010_883_17e-15,
    9.613_059_804e-15,
    1.281_741_307_2e-14,
    1.602_176_634e-14,
    2.403_264_951e-14,
    3.204_353_268e-14,
    4.806_529_902e-14,
    6.408_706_536e-14,
    8.010_883_17e-14,
    9.613_059_804e-14,
    1.281_741_307_2e-13,
    1.602_176_634e-13,
    2.002_720_792_500_000_2e-13,
    2.403_264_951e-13,
    3.204_353_268e-13,
    4.806_529_902e-13,
    6.408_706_536e-13,
    8.010_883_170_000_001e-13,
    9.613_059_804e-13,
    1.281_741_307_2e-12,
    1.602_176_634_000_000_1e-12,
    2.403_264_951e-12,
    3.204_353_268_000_000_3e-12,
];

/// Natural logarithms of [`KNOTS_BASE_JOULE`], precomputed because `ln` is not
/// const-stable and the interpolation spans need them on every query.
pub(super) const LN_KNOTS: [f64; KNOT_COUNT] = [
    -34.067_413_293_915_564,
    -33.661_948_185_807_404,
    -33.374_266_113_355_62,
    -32.968_801_005_247_46,
    -32.681_118_932_795_68,
    -32.457_975_381_481_47,
    -32.275_653_824_687_51,
    -31.987_971_752_235_73,
    -31.764_828_200_921_52,
    -31.359_363_092_813_354,
    -31.071_681_020_361_574,
    -30.666_215_912_253_41,
    -30.378_533_839_801_63,
    -30.155_390_288_487_418,
    -29.973_068_731_693_463,
    -29.685_386_659_241_683,
    -29.462_243_107_927_474,
    -29.239_099_556_613_265,
    -29.056_777_999_819_31,
    -28.769_095_927_367_527,
    -28.363_630_819_259_363,
    -28.075_948_746_807_583,
    -27.852_805_195_493_374,
    -27.670_483_638_699_42,
    -27.382_801_566_247_636,
    -27.159_658_014_933_427,
    -26.754_192_906_825_264,
    -26.466_510_834_373_484,
];

/// Per-table NIST reference data with its pre-solved spline curvature.
///
/// Holding the knots, their logarithms, and the natural cubic-spline second
/// derivatives beside the coefficients moves the tridiagonal solve and the
/// knot-energy conversions out of the per-query path and into constant data.
pub(super) struct NistTable<const N: usize> {
    /// Knot energies in canonical SI base units (joules).
    pub(super) knots: [f64; N],
    /// Natural logarithms of `knots`, matching [`LN_KNOTS`].
    pub(super) ln_knots: [f64; N],
    /// `mu/rho` values at the knots, in `cm^2/g`.
    pub(super) coefficients: [f64; N],
    /// Natural cubic-spline second derivatives in log-energy/log-coefficient
    /// space.
    pub(super) second_derivatives: [f64; N],
}

// Source: NIST X-Ray Mass Attenuation Coefficients, dry-air HTML table;
// https://physics.nist.gov/PhysRefData/XrayMassCoef/ComTab/air.html;
// retrieved 2026-08-14.
pub(super) const DRY_AIR_MASS_ATTENUATION: [f64; KNOT_COUNT] = [
    5.120, 1.614, 0.7779, 0.3538, 0.2485, 0.2080, 0.1875, 0.1662, 0.1541, 0.1356, 0.1233, 0.1067,
    0.09549, 0.08712, 0.08055, 0.07074, 0.06358, 0.05687, 0.05175, 0.04447, 0.03581, 0.03079,
    0.02751, 0.02522, 0.02225, 0.02045, 0.01810, 0.01705,
];

const DRY_AIR_SECOND_DERIVATIVES: [f64; KNOT_COUNT] = [
    0.0,
    0.987_901_342_919_169_4,
    1.707_330_382_288_008_6,
    2.251_050_109_971_453_5,
    1.659_824_220_072_059_2,
    1.081_707_014_133_710_4,
    0.666_003_587_000_211_4,
    0.265_326_700_718_281,
    0.088_862_317_971_594_13,
    -0.076_323_368_476_501_84,
    -0.072_667_014_742_536_27,
    -0.083_483_557_298_678_87,
    -0.104_620_132_812_496_84,
    -0.092_630_693_460_280_51,
    -0.083_644_537_751_999_19,
    -0.113_679_501_506_897_32,
    -0.092_223_844_585_892_35,
    -0.098_080_420_702_939_46,
    -0.031_383_097_835_901_3,
    -0.034_417_182_758_991_59,
    0.033_888_060_609_909_26,
    0.076_176_831_340_286_22,
    0.151_372_066_319_107_24,
    0.157_948_210_995_144_4,
    0.246_489_623_402_574_13,
    0.213_524_219_349_429_28,
    0.341_482_500_011_609_46,
    0.0,
];

// Source: NIST X-Ray Mass Attenuation Coefficients, liquid-water HTML table;
// https://physics.nist.gov/PhysRefData/XrayMassCoef/ComTab/water.html;
// retrieved 2026-08-14.
pub(super) const LIQUID_WATER_MASS_ATTENUATION: [f64; KNOT_COUNT] = [
    5.329, 1.673, 0.8096, 0.3756, 0.2683, 0.2269, 0.2059, 0.1837, 0.1707, 0.1505, 0.1370, 0.1186,
    0.1061, 0.09687, 0.08956, 0.07865, 0.07072, 0.06323, 0.05754, 0.04942, 0.03969, 0.03403,
    0.03031, 0.02770, 0.02429, 0.02219, 0.01941, 0.01813,
];

const LIQUID_WATER_SECOND_DERIVATIVES: [f64; KNOT_COUNT] = [
    0.0,
    1.065_615_734_950_483_8,
    1.836_910_839_928_375_3,
    2.269_557_529_990_773_8,
    1.589_963_071_866_41,
    1.043_083_833_106_582,
    0.601_623_865_812_629_5,
    0.211_804_519_824_576_96,
    0.073_696_721_079_889_25,
    -0.074_237_730_829_456_9,
    -0.081_293_673_549_001_16,
    -0.098_721_524_046_640_43,
    -0.065_484_981_184_413_2,
    -0.130_025_500_908_199_03,
    -0.081_341_163_396_997_9,
    -0.093_958_559_912_026_27,
    -0.130_106_759_301_192_28,
    -0.068_882_735_842_949_27,
    -0.045_031_012_187_976_16,
    -0.050_850_971_414_811_48,
    0.028_894_249_908_957_843,
    0.056_244_306_671_472_205,
    0.136_115_584_273_115_38,
    0.145_475_075_140_574_92,
    0.215_079_998_299_830_02,
    0.210_280_933_428_163_54,
    0.340_943_253_671_293_47,
    0.0,
];

// Source: NIST X-Ray Mass Attenuation Coefficients, cortical-bone HTML table;
// https://physics.nist.gov/PhysRefData/XrayMassCoef/ComTab/bone.html;
// retrieved 2026-08-14.
pub(super) const CORTICAL_BONE_MASS_ATTENUATION: [f64; KNOT_COUNT] = [
    28.51, 9.032, 4.001, 1.331, 0.6655, 0.4242, 0.3148, 0.2229, 0.1855, 0.1480, 0.1309, 0.1113,
    0.09908, 0.09022, 0.08332, 0.07308, 0.06566, 0.05871, 0.05346, 0.04607, 0.03745, 0.03257,
    0.02946, 0.02734, 0.02467, 0.02314, 0.02132, 0.02068,
];

const CORTICAL_BONE_SECOND_DERIVATIVES: [f64; KNOT_COUNT] = [
    0.0,
    -0.029_665_609_475_200_937,
    0.240_123_274_541_419_13,
    0.914_574_781_542_999_9,
    1.616_121_224_057_045_5,
    1.942_703_650_687_551_7,
    1.958_775_482_978_477,
    1.460_921_735_748_929_7,
    0.919_966_614_803_954_3,
    0.281_155_612_151_807_23,
    0.064_256_034_026_975_33,
    -0.023_476_087_845_265_177,
    -0.065_578_508_434_062_61,
    -0.087_123_080_392_765_2,
    -0.077_928_879_680_406_78,
    -0.095_671_371_895_876_6,
    -0.106_110_979_911_169_6,
    -0.060_088_945_015_980_985,
    -0.011_494_013_039_414_34,
    0.005_939_181_311_596_743,
    0.080_063_538_606_403_7,
    0.139_684_091_641_551_27,
    0.213_482_033_652_479_14,
    0.200_055_041_326_524_3,
    0.304_104_691_684_797_8,
    0.239_645_751_648_746_35,
    0.345_781_824_955_831_26,
    0.0,
];

/// Dry-air table with pre-solved spline curvature.
pub(super) const DRY_AIR_TABLE: NistTable<KNOT_COUNT> = NistTable {
    knots: KNOTS_BASE_JOULE,
    ln_knots: LN_KNOTS,
    coefficients: DRY_AIR_MASS_ATTENUATION,
    second_derivatives: DRY_AIR_SECOND_DERIVATIVES,
};

/// Liquid-water table with pre-solved spline curvature.
pub(super) const LIQUID_WATER_TABLE: NistTable<KNOT_COUNT> = NistTable {
    knots: KNOTS_BASE_JOULE,
    ln_knots: LN_KNOTS,
    coefficients: LIQUID_WATER_MASS_ATTENUATION,
    second_derivatives: LIQUID_WATER_SECOND_DERIVATIVES,
};

/// Cortical-bone table with pre-solved spline curvature.
pub(super) const CORTICAL_BONE_TABLE: NistTable<KNOT_COUNT> = NistTable {
    knots: KNOTS_BASE_JOULE,
    ln_knots: LN_KNOTS,
    coefficients: CORTICAL_BONE_MASS_ATTENUATION,
    second_derivatives: CORTICAL_BONE_SECOND_DERIVATIVES,
};
