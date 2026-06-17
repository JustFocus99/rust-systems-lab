#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    AccountAlreadyExists(String),
    SenderNotFound(String),
    ReceiverNotFound(String),
    ZeroAmount,
    SelfTransfer,
    InsufficientBalance { available: u64, requested: u64 },
    IncorrectNonce { expected: u64, received: u64 },
    BalanceOverflow,
}
