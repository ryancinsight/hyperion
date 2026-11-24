//! Fixture rows that pin the bound table and the displacement edge cases.
use super::{Native, Quantity, observe};
use eunomia::{F32, F64};

/// Fixture rows at which `D` lies two steps from its reference, with the
/// computed bits and the signed step count, one above and one below the
/// reference in each format. Their references are verified by
/// `scripts/verify_diffusion_range.py`.
const DIFFUSION_WITNESSES: [(&str, u64, i128); 4] = [
    ("32 31000000 3d2aaaae 40fffffa 37800001", 0x40ff_fffc, 2),
    ("32 405b5879 3400375c 3dc72fba 40bdf578", 0x3dc7_2fb8, -2),
    (
        "64 3c50000000000000 3fa5555555555556 401ffffffffffffe 3e06a09e667f3bcd",
        0x4020_0000_0000_0000,
        2,
    ),
    (
        "64 3ffcd00826e2dae6 40109c6ab5004d58 3facaa9a6dc82591 4016af5f2fe6621d",
        0x3fac_aa9a_6dc8_258f,
        -2,
    ),
];

/// `D` attains its bound in both directions, so the bound is no looser than
/// the corpus requires.
///
/// Each witness of `T`'s width computes the recorded bits at the recorded
/// distance from its reference, and the largest distance toward each
/// direction over every `D` row equals `Quantity::max_steps`. A bound loosened
/// by one step leaves that extreme short of it, and one tightened by one step
/// rejects the witnesses.
fn diffusion_witnesses_attain_the_bound<T: Native>() {
    let rows: Vec<_> = observe::<T>()
        .into_iter()
        .filter(|row| row.quantity == Quantity::Diffusion)
        .collect();
    let mut found = 0;
    for (line, computed, excess) in DIFFUSION_WITNESSES {
        if line.split_whitespace().next() != Some(T::WIDTH) {
            continue;
        }
        let matching: Vec<_> = rows.iter().filter(|row| row.line == line).collect();
        assert_eq!(matching.len(), 1, "witness row is in the fixture: {line}");
        assert_eq!(matching[0].got.bits(), computed, "computed bits: {line}");
        assert_eq!(matching[0].excess(), excess, "distance: {line}");
        found += 1;
    }
    assert_eq!(found, 2, "one witness per direction at width {}", T::WIDTH);
    for direction in [1_i128, -1] {
        let extreme = rows
            .iter()
            .map(|row| direction * row.excess())
            .max()
            .expect("D rows");
        assert_eq!(
            extreme,
            i128::from(Quantity::Diffusion.max_steps()),
            "D rows reach the bound toward {direction}"
        );
    }
}
#[test]
fn diffusion_witnesses_attain_the_bound_for_every_scalar() {
    diffusion_witnesses_attain_the_bound::<f32>();
    diffusion_witnesses_attain_the_bound::<f64>();
    diffusion_witnesses_attain_the_bound::<F32>();
    diffusion_witnesses_attain_the_bound::<F64>();
}

/// A negative zero has no displaced encoding, since encodings move away from
/// zero on the negative side and below it reach the NaN encodings.
fn negative_zero_rows_are_not_displaced<T: Native>() {
    let rows: Vec<_> = observe::<T>()
        .into_iter()
        .filter(|row| row.got.bits() & T::sign_bit() != 0)
        .collect();
    assert!(!rows.is_empty(), "the fixture has signed-zero rows");
    for row in rows {
        assert_eq!(row.got.bits(), T::sign_bit(), "negative zero: {}", row.line);
        assert_eq!(row.want.bits(), T::sign_bit(), "reference: {}", row.line);
        for steps in [-2_i64, -1, 1, 2] {
            assert!(row.displaced(steps).is_none(), "{steps}: {}", row.line);
        }
    }
}
#[test]
fn negative_zero_is_not_displaced_for_every_scalar() {
    negative_zero_rows_are_not_displaced::<f32>();
    negative_zero_rows_are_not_displaced::<f64>();
    negative_zero_rows_are_not_displaced::<F32>();
    negative_zero_rows_are_not_displaced::<F64>();
}
