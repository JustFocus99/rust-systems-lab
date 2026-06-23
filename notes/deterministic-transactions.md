# Transaction Identity

A transaction ID should not be manually assigned. It should be **derived from transaction content**.

This document describes the three-layer design in `src/transaction.rs` and verified in `tests/transaction_tests.rs`.

## Three layers

Protocol software separates three concerns:

| Layer | Type | Role |
|-------|------|------|
| Human-readable data | `Transaction` | The Rust struct you construct and reason about in code |
| Canonical bytes | `Vec<u8>` from `canonical_bytes()` | A deterministic encoded representation of the content |
| Identity | `TransactionId` | SHA-256 hash of the canonical bytes |

```
Transaction  --canonical_bytes()-->  Vec<u8>  --SHA-256-->  TransactionId
```

Each layer has one job. The struct is for humans and application logic. The bytes are for wire format and hashing. The ID is for referencing a specific piece of content without storing or comparing the full payload.

## Transaction — human-readable Rust data

A `Transaction` is a plain struct with four fields:

| Field | Type | Meaning |
|-------|------|---------|
| `sender` | `String` | Account sending tokens |
| `receiver` | `String` | Account receiving tokens |
| `amount` | `u64` | Tokens to move |
| `nonce` | `u64` | Sender's sequence number at time of signing |

Example:

```rust
let tx = Transaction::new("alice", "bob", 10, 0);
```

This is the layer you work with in Rust. Field names, types, and constructors are chosen for clarity — not for hashing.

## Canonical bytes — deterministic encoding

`canonical_bytes()` produces a **fixed, deterministic** byte sequence for a given transaction. Two transactions with identical field values always produce identical bytes.

Implementation:

- `Transaction` derives `Serialize` (via serde).
- `canonical_bytes()` encodes with **bincode** using `config::standard()`.

```rust
let bytes = tx.canonical_bytes();
```

Properties:

- **Deterministic.** Same content → same bytes, every time, on every machine.
- **Content-derived.** The encoding is computed from the transaction fields, not assigned externally.
- **Protocol-defined.** The exact byte layout is part of the protocol specification. Any change to field order, types, or encoding config changes the bytes — and therefore changes the ID.

Canonical encoding is what makes transaction identity portable. Nodes, wallets, and indexers can independently hash the same content and agree on the ID without sharing a central ID allocator.

## TransactionId — SHA-256 of canonical bytes

`id()` derives a `TransactionId` from the transaction:

```rust
let id = tx.id(); // SHA-256(canonical_bytes())
```

`TransactionId` wraps a 32-byte array (`[u8; 32]`). It can be displayed as hex via `to_hex()` or `Display`.

The ID is never set by hand. It is always:

```
TransactionId = SHA-256(transaction.canonical_bytes())
```

If any field changes — sender, receiver, amount, or nonce — the canonical bytes change, and the ID changes.

## Guarantees verified by tests

| Test | Property |
|------|----------|
| `same_transaction_produces_same_canonical_bytes` | Identical content → identical encoding |
| `same_transaction_produces_same_id` | Identical content → identical ID |
| `different_transactions_produce_different_ids` | Different content → different ID |
| `transaction_id_is_sha256_of_canonical_bytes` | ID is exactly the SHA-256 digest of canonical bytes |

## Why this matters in protocol software

This pattern appears throughout blockchain and distributed systems:

- **Content addressing.** A transaction is identified by what it *is*, not by who minted an ID for it. Two honest nodes hashing the same signed payload arrive at the same transaction hash.
- **Tamper evidence.** Changing a single field changes the ID. You cannot silently alter a transaction and keep the same identity.
- **Deduplication.** Systems can detect duplicate submissions by comparing IDs without deep equality checks on every field.
- **Stable references.** Blocks, mempools, and explorers refer to transactions by ID. That reference remains valid as long as the underlying content is unchanged.
- **Separation of concerns.** Application code works with `Transaction`. Networking and storage work with bytes. Indexing and lookup work with `TransactionId`. Each layer stays simple.

Manually assigning transaction IDs — auto-increment counters, UUIDs, or database primary keys — breaks these properties. Different nodes would disagree on identity, replays would be harder to detect, and content could drift from its reference.

Deriving the ID from canonical bytes keeps identity **deterministic, content-bound, and protocol-native**.
