use rust_systems_lab::{Block, BlockHeader};

fn sample_block() -> Block {
    Block {
        header: BlockHeader {
            height: 1,
            previous_hash: [0u8; 32],
            transaction_commitment: [1u8; 32],
            state_commitment: [2u8; 32],
        },
        transactions: Vec::new(),
    }
}

#[test]
fn same_block_hashes_identically() {
    let left = sample_block();
    let right = sample_block();

    assert_eq!(left.hash().unwrap(), right.hash().unwrap());
}

#[test]
fn different_block_height_changes_hash() {
    let base = sample_block();
    let mut changed = sample_block();
    changed.header.height = 2;

    assert_ne!(base.hash().unwrap(), changed.hash().unwrap());
}

#[test]
fn different_previous_hash_changes_hash() {
    let base = sample_block();
    let mut changed = sample_block();
    changed.header.previous_hash = [9u8; 32];

    assert_ne!(base.hash().unwrap(), changed.hash().unwrap());
}

#[test]
fn different_transaction_commitment_changes_hash() {
    let base = sample_block();
    let mut changed = sample_block();
    changed.header.transaction_commitment = [9u8; 32];

    assert_ne!(base.hash().unwrap(), changed.hash().unwrap());
}

#[test]
fn different_state_commitment_changes_hash() {
    let base = sample_block();
    let mut changed = sample_block();
    changed.header.state_commitment = [9u8; 32];

    assert_ne!(base.hash().unwrap(), changed.hash().unwrap());
}
