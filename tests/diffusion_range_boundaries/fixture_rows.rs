//! Fixture rows that pin the bound table and the displacement edge cases.
use super::{Native, Quantity, observe};
use eunomia::{F32, F64};

/// Fixture rows at which a quantity lies two steps from its reference: the
/// quantity, the row, the computed bits and the signed step count. Their
/// references are verified by `scripts/verify_diffusion_range.py`.
///
/// `D` has one row above and one below the reference in each format.
/// `mu_eff` has one above and two below in binary32 and two above in binary64;
/// no binary64 row two steps below is known.
const WITNESSES: [(Quantity, &str, u64, i128); 9] = [
    (
        Quantity::Diffusion,
        "32 31000000 3d2aaaae 40fffffa 37800001",
        0x40ff_fffc,
        2,
    ),
    (
        Quantity::Diffusion,
        "32 405b5879 3400375c 3dc72fba 40bdf578",
        0x3dc7_2fb8,
        -2,
    ),
    (
        Quantity::Diffusion,
        "64 3c50000000000000 3fa5555555555556 401ffffffffffffe 3e06a09e667f3bcd",
        0x4020_0000_0000_0000,
        2,
    ),
    (
        Quantity::Diffusion,
        "64 3ffcd00826e2dae6 40109c6ab5004d58 3facaa9a6dc82591 4016af5f2fe6621d",
        0x3fac_aa9a_6dc8_258f,
        -2,
    ),
    (
        Quantity::Effective,
        "32 447fffb6 5080062f 2daaa26c 4addb913",
        0x4add_b911,
        -2,
    ),
    (
        Quantity::Effective,
        "32 03000001 0f0002be 6f2aa702 095db638",
        0x095d_b63a,
        2,
    ),
    (
        Quantity::Effective,
        "32 10ffffb6 1d000397 612aa5e1 175db6d4",
        0x175d_b6d2,
        -2,
    ),
    (
        Quantity::Effective,
        "64 3fb000000000036d 3c8c00000000001f 40155555555550bf 3fbbb67ae858529c",
        0x3fbb_b67a_e858_529e,
        2,
    ),
    (
        Quantity::Effective,
        "64 38a000000000094d 357c000000000011 47255555555548ea 38abb67ae8585cc9",
        0x38ab_b67a_e858_5ccb,
        2,
    ),
];

/// Each quantity attains its bound, so the bound is no looser than the corpus
/// requires, in exactly the directions that have a witness.
///
/// Each witness of `T`'s width computes the recorded bits at the recorded
/// distance, the bound, from its reference. Toward each direction the largest
/// distance over the rows of a quantity equals `Quantity::max_steps` when a
/// witness lies that way and falls short of it otherwise, so a corpus row
/// that attains the bound where none is recorded fails here until recorded. A
/// bound loosened by one step leaves the extreme short of it, and one
/// tightened by one step rejects the witnesses.
fn witnesses_attain_the_bound<T: Native>() {
    let observations = observe::<T>();
    for quantity in Quantity::ALL {
        let rows: Vec<_> = observations
            .iter()
            .filter(|row| row.quantity == quantity)
            .collect();
        let mut recorded = Vec::new();
        for (witness, line, computed, excess) in WITNESSES {
            if witness != quantity || line.split_whitespace().next() != Some(T::WIDTH) {
                continue;
            }
            assert_eq!(
                excess.unsigned_abs(),
                u128::from(quantity.max_steps()),
                "{quantity:?} witness lies at the bound: {line}"
            );
            let matching: Vec<_> = rows.iter().filter(|row| row.line == line).collect();
            assert_eq!(matching.len(), 1, "witness row is in the fixture: {line}");
            assert_eq!(matching[0].got.bits(), computed, "computed bits: {line}");
            assert_eq!(matching[0].excess(), excess, "distance: {line}");
            recorded.push(excess);
        }
        assert!(
            !recorded.is_empty(),
            "{quantity:?} has a witness at width {}",
            T::WIDTH
        );
        for direction in [1_i128, -1] {
            let extreme = rows
                .iter()
                .map(|row| direction * row.excess())
                .max()
                .expect("rows of the quantity");
            let witnessed = recorded.iter().any(|excess| direction * excess > 0);
            assert_eq!(
                extreme == i128::from(quantity.max_steps()),
                witnessed,
                "{quantity:?} rows reach the bound toward {direction} exactly where a witness records it"
            );
        }
    }
}
#[test]
fn diffusion_witnesses_attain_the_bound_for_every_scalar() {
    witnesses_attain_the_bound::<f32>();
    witnesses_attain_the_bound::<f64>();
    witnesses_attain_the_bound::<F32>();
    witnesses_attain_the_bound::<F64>();
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
