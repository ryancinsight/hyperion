pub(super) const KNOT_COUNT: usize = 28;
pub(super) const MINIMUM_ENERGY_MEV: f64 = 0.01;
pub(super) const MAXIMUM_ENERGY_MEV: f64 = 20.0;

pub(super) const PHOTON_ENERGY_MEV: [f64; KNOT_COUNT] = [
    0.01, 0.015, 0.02, 0.03, 0.04, 0.05, 0.06, 0.08, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.8, 1.0,
    1.25, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 15.0, 20.0,
];

// Source: https://physics.nist.gov/PhysRefData/XrayMassCoef/ComTab/air.html
pub(super) const DRY_AIR_MASS_ATTENUATION: [f64; KNOT_COUNT] = [
    5.120, 1.614, 0.7779, 0.3538, 0.2485, 0.2080, 0.1875, 0.1662, 0.1541, 0.1356, 0.1233, 0.1067,
    0.09549, 0.08712, 0.08055, 0.07074, 0.06358, 0.05687, 0.05175, 0.04447, 0.03581, 0.03079,
    0.02751, 0.02522, 0.02225, 0.02045, 0.01810, 0.01705,
];

// Source: https://physics.nist.gov/PhysRefData/XrayMassCoef/ComTab/water.html
pub(super) const LIQUID_WATER_MASS_ATTENUATION: [f64; KNOT_COUNT] = [
    5.329, 1.673, 0.8096, 0.3756, 0.2683, 0.2269, 0.2059, 0.1837, 0.1707, 0.1505, 0.1370, 0.1186,
    0.1061, 0.09687, 0.08956, 0.07865, 0.07072, 0.06323, 0.05754, 0.04942, 0.03969, 0.03403,
    0.03031, 0.02770, 0.02429, 0.02219, 0.01941, 0.01813,
];

// Source: https://physics.nist.gov/PhysRefData/XrayMassCoef/ComTab/bone.html
pub(super) const CORTICAL_BONE_MASS_ATTENUATION: [f64; KNOT_COUNT] = [
    28.51, 9.032, 4.001, 1.331, 0.6655, 0.4242, 0.3148, 0.2229, 0.1855, 0.1480, 0.1309, 0.1113,
    0.09908, 0.09022, 0.08332, 0.07308, 0.06566, 0.05871, 0.05346, 0.04607, 0.03745, 0.03257,
    0.02946, 0.02734, 0.02467, 0.02314, 0.02132, 0.02068,
];
