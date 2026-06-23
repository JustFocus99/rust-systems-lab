use rust_systems_lab::Transaction;
use sha2::{Digest, Sha256};

fn sample_transaction() -> Transaction {
    Transaction::new("alice", "bob", 10, 0)
}

#[test]
fn same_transaction_produces_same_canonical_bytes() {
    let left = sample_transaction();
    let right = sample_transaction();

    assert_eq!(left.canonical_bytes(), right.canonical_bytes());
}

#[test]
fn same_transaction_produces_same_id() {
    let left = sample_transaction();
    let right = sample_transaction();

    assert_eq!(left.id(), right.id());
}

#[test]
fn different_transactions_produce_different_ids() {
    let transfer_to_bob = Transaction::new("alice", "bob", 10, 0);
    let transfer_to_carol = Transaction::new("alice", "carol", 10, 0);

    assert_ne!(transfer_to_bob.id(), transfer_to_carol.id());
}

#[test]
fn transaction_id_is_sha256_of_canonical_bytes() {
    let transaction = sample_transaction();
    let canonical = transaction.canonical_bytes();
    let expected = Sha256::digest(&canonical);

    assert_eq!(transaction.id().as_bytes(), expected.as_slice());
}
