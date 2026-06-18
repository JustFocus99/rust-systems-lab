# Ledger Invariants

An **invariant** is a property that must always hold true for the system. If an operation would break an invariant, it must be rejected rather than applied partially or incorrectly.

This document describes the invariants enforced by the ledger in `src/ledger.rs` and verified in `tests/ledger_tests.rs`.

## Account invariants

Each account is identified by a unique string ID and holds two fields:

| Field     | Meaning |
|-----------|---------|
| `balance` | Tokens held by the account |
| `nonce`   | Number of transfers already sent from this account |

Rules:

- **Unique IDs.** No two accounts may share the same ID. `create_account` rejects a duplicate with `AccountAlreadyExists`.
- **Non-negative balance.** Balances are `u64`, so they cannot go negative. Insufficient funds are rejected before any mutation.
- **Initial nonce is zero.** A newly created account starts with `nonce = 0`.
- **Nonce only advances on send.** When an account sends a transfer, its nonce increases by exactly 1. Receiving a transfer does not change the receiver's nonce.

## Transfer invariants

A `Transfer` moves `amount` tokens from `from` to `to`, and must include the sender's current `nonce`.

A transfer is valid only when all of the following hold:

| Check | Error if violated |
|-------|-------------------|
| `amount > 0` | `ZeroAmount` |
| `from != to` | `SelfTransfer` |
| Sender account exists | `SenderNotFound` |
| Receiver account exists | `ReceiverNotFound` |
| `transfer.nonce == sender.nonce` | `IncorrectNonce` |
| `sender.balance >= amount` | `InsufficientBalance` |
| `receiver.balance + amount` does not overflow `u64` | `BalanceOverflow` |

On success:

- Sender balance decreases by `amount`.
- Sender nonce increases by 1.
- Receiver balance increases by `amount`.
- Receiver nonce is unchanged.

## Ledger invariants

At the ledger level, these properties must hold across all operations:

- **Conservation of supply.** The sum of all account balances (`total_supply`) is unchanged by a successful transfer. Tokens move between accounts; they are not created or destroyed.
- **Atomicity.** A transfer either fully succeeds (both balances and the sender nonce update together) or fully fails (no field changes).
- **No partial updates.** Validation completes before any mutation. The implementation validates inside an immutable-borrow block, then mutates one account at a time only after all checks pass.

## Why failed operations must not mutate state

If a rejected transfer still changed balances or nonces, the ledger would enter an inconsistent state:

- A sender could lose tokens without the receiver gaining them.
- A nonce could advance even though the transfer was invalid, permanently blocking future transfers from that account.
- `total_supply` could drift, breaking the conservation invariant.

The ledger enforces **validate-then-mutate**: every check in `apply_transfer` runs before any `balance` or `nonce` field is written. The error-path tests (`rejects_unknown_sender`, `rejects_unknown_receiver`, `rejects_insufficient_balance`, `rejects_wrong_nonce`) assert that Alice and Bob still have balance `100` and nonce `0` after each failure.

The same principle applies to `create_account`: if the ID already exists, the existing account is left untouched.

## Why this matters in blockchain/protocol software

Distributed ledgers and protocol state machines share the same constraints:

- **Determinism.** Every node must reach the same state given the same sequence of operations. Partial updates make replay and consensus impossible.
- **Replay protection.** Nonces (or sequence numbers) ensure each transfer is applied at most once in the intended order. Advancing a nonce on failure would desynchronize clients from the chain.
- **Economic safety.** Conservation of supply is fundamental. Silent minting, burning, or loss of funds destroys trust in the system.
- **Composable failures.** Higher layers (wallets, APIs, smart contracts) rely on `Result<(), LedgerError>` meaning "nothing changed." Callers can safely retry or surface errors without repairing corrupted local state.

Designing operations as pure state transitions with explicit error returns — and tests that verify the no-mutation property on every failure path — is standard practice in financial and protocol engineering.
