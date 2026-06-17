use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxError {
    DuplicateFloorClaim,
    InvalidValidatorSignature,
    RewardMismatch,
    MaxSupplyExceeded,
    EmissionEpochMismatch,
    InsufficientBalance,
    InvalidTransaction,
    InvalidGameRulesVersion,
}

impl std::fmt::Display for TxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxError::DuplicateFloorClaim => write!(f, "Floor already claimed"),
            TxError::InvalidValidatorSignature => write!(f, "Invalid validator signature"),
            TxError::RewardMismatch => write!(f, "Claimed reward does not match validated reward"),
            TxError::MaxSupplyExceeded => write!(f, "Max supply exceeded"),
            TxError::EmissionEpochMismatch => write!(f, "Emission epoch mismatch"),
            TxError::InsufficientBalance => write!(f, "Insufficient balance"),
            TxError::InvalidTransaction => write!(f, "Invalid transaction"),
            TxError::InvalidGameRulesVersion => write!(f, "Invalid game rules version"),
        }
    }
}

impl std::error::Error for TxError {}
