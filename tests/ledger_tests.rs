use rust_systems_lab::Ledger;
use rust_systems_lab::LedgerError;
use rust_systems_lab::Transfer;

// helper function to avoid duplicated codes
fn ledger_setup() -> (Ledger, String, String) {
    let mut ledger = Ledger::new();
    let id0 = String::from("alice");
    assert_eq!(ledger.create_account(&id0, 100), Ok(()));

    let id1 = String::from("bob");
    assert_eq!(ledger.create_account(&id1, 100), Ok(()));
    (ledger, id0, id1)
}

#[test]
fn creates_account() {
    let (ledger, id0, _) = ledger_setup();
    assert_ne!(ledger.account(&id0), None);
    let account = ledger.account(&id0).unwrap();
    assert_eq!(account.balance, 100);
    assert_eq!(account.nonce, 0);
}

#[test]
fn rejects_duplicate_account() {
    let (mut ledger, id0, _) = ledger_setup();
    assert_eq!(
        ledger.create_account(&id0, 100),
        Err(LedgerError::AccountAlreadyExists)
    );
}

// helper to avoid duplication
fn preserves_balance_after_error(ledger: Ledger, id0: String, id1: String) {
    let alice = ledger.account(&id0).unwrap();
    let bob = ledger.account(&id1).unwrap();

    assert_eq!(alice.balance, 100);
    assert_eq!(bob.balance, 100);
    assert_eq!(alice.nonce, 0);
}

#[test]
fn applies_valid_transfer() {
    let (mut ledger, id0, id1) = ledger_setup();
    let transfer = Transfer {
        from: id0.clone(),
        to: id1.clone(),
        amount: 10,
        nonce: 0,
    };
    assert_eq!(ledger.apply_transfer(&transfer), Ok(()));

    let alice = ledger.account(&id0).unwrap();
    let bob = ledger.account(&id1).unwrap();

    assert_eq!(alice.balance, 90);
    assert_eq!(bob.balance, 110);
    assert_eq!(alice.nonce, 1);
}

#[test]
fn rejects_unknown_sender() {
    let (mut ledger, id0, id1) = ledger_setup();
    let transfer = Transfer {
        from: String::from("Nobody"),
        to: id1.clone(),
        amount: 10,
        nonce: 0,
    };
    assert_eq!(
        ledger.apply_transfer(&transfer),
        Err(LedgerError::SenderNotFound)
    );

    preserves_balance_after_error(ledger, id0, id1);
}

#[test]
fn rejects_unknown_receiver() {
    let (mut ledger, id0, id1) = ledger_setup();
    let transfer = Transfer {
        from: id0.clone(),
        to: String::from("Nobody"),
        amount: 10,
        nonce: 0,
    };
    assert_eq!(
        ledger.apply_transfer(&transfer),
        Err(LedgerError::ReceiverNotFound)
    );

    preserves_balance_after_error(ledger, id0, id1);
}

#[test]
fn rejects_insufficient_balance() {
    let (mut ledger, id0, id1) = ledger_setup();
    let transfer = Transfer {
        from: id0.clone(),
        to: id1.clone(),
        amount: 500,
        nonce: 0,
    };
    assert_eq!(
        ledger.apply_transfer(&transfer),
        Err(LedgerError::InsufficientBalance)
    );

    preserves_balance_after_error(ledger, id0, id1);
}

#[test]
fn rejects_wrong_nonce() {
    let (mut ledger, id0, id1) = ledger_setup();
    let transfer = Transfer {
        from: id0.clone(),
        to: id1.clone(),
        amount: 10,
        nonce: 110,
    };
    assert_eq!(
        ledger.apply_transfer(&transfer),
        Err(LedgerError::IncorrectNonce)
    );

    preserves_balance_after_error(ledger, id0, id1);
}

#[test]
fn preserves_total_supply() {
    let (mut ledger, id0, id1) = ledger_setup();
    let total_original = ledger.total_supply();
    assert_eq!(total_original, 200);

    let transfer = Transfer {
        from: id0.clone(),
        to: id1.clone(),
        amount: 10,
        nonce: 0,
    };
    assert_eq!(ledger.apply_transfer(&transfer), Ok(()));

    let total_later = ledger.total_supply();
    assert_eq!(total_later, 200);
}
