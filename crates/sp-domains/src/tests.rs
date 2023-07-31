use crate::signer_in_tx_range;
use num_traits::ops::wrapping::{WrappingAdd, WrappingSub};
use subspace_core_primitives::U256;

#[test]
fn test_tx_range() {
    let tx_range = U256::MAX / 4;
    let bundle_vrf_hash = U256::MAX / 2;

    let signer_id_hash = bundle_vrf_hash + U256::from(10_u64);
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash - U256::from(10_u64);
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash + U256::MAX / 8;
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash - U256::MAX / 8;
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash + U256::MAX / 8 + U256::from(1_u64);
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash - U256::MAX / 8 - U256::from(1_u64);
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash + U256::MAX / 4;
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash - U256::MAX / 4;
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));
}

#[test]
fn test_tx_range_wrap_under_flow() {
    let tx_range = U256::MAX / 4;
    let bundle_vrf_hash = U256::from(100_u64);

    let signer_id_hash = bundle_vrf_hash + U256::from(1000_u64);
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash.wrapping_sub(&U256::from(1000_u64));
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash + U256::MAX / 8;
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let v = U256::MAX / 8;
    let signer_id_hash = bundle_vrf_hash.wrapping_sub(&v);
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash + U256::MAX / 8 + U256::from(1_u64);
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let v = U256::MAX / 8 + U256::from(1_u64);
    let signer_id_hash = bundle_vrf_hash.wrapping_sub(&v);
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash + U256::MAX / 4;
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let v = U256::MAX / 4;
    let signer_id_hash = bundle_vrf_hash.wrapping_sub(&v);
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));
}

#[test]
fn test_tx_range_wrap_over_flow() {
    let tx_range = U256::MAX / 4;
    let v = U256::MAX;
    let bundle_vrf_hash = v.wrapping_sub(&U256::from(100_u64));

    let signer_id_hash = bundle_vrf_hash.wrapping_add(&U256::from(1000_u64));
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash - U256::from(1000_u64);
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let v = U256::MAX / 8;
    let signer_id_hash = bundle_vrf_hash.wrapping_add(&v);
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash - U256::MAX / 8;
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let v = U256::MAX / 8 + U256::from(1_u64);
    let signer_id_hash = bundle_vrf_hash.wrapping_add(&v);
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash - U256::MAX / 8 - U256::from(1_u64);
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let v = U256::MAX / 4;
    let signer_id_hash = bundle_vrf_hash.wrapping_add(&v);
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));

    let signer_id_hash = bundle_vrf_hash - U256::MAX / 4;
    assert!(!signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));
}

#[test]
fn test_tx_range_max() {
    let tx_range = U256::MAX;
    let bundle_vrf_hash = U256::MAX / 2;

    let signer_id_hash = bundle_vrf_hash + U256::from(10_u64);
    assert!(signer_in_tx_range(
        &bundle_vrf_hash,
        &signer_id_hash,
        &tx_range
    ));
}
