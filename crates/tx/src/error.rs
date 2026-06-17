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
    InsufficientValidatorSignatures,
    UnknownValidator,
    DuplicateValidatorSignature,
    InsufficientMatchingSignatures,
    InvalidGameId,
    InvalidFloorNumber,
    InvalidRewardAmount,
    ClaimKeyMismatch,
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
            TxError::InsufficientValidatorSignatures => write!(f, "Insufficient validator signatures"),
            TxError::UnknownValidator => write!(f, "Unknown or inactive validator"),
            TxError::DuplicateValidatorSignature => write!(f, "Duplicate validator signature"),
            TxError::InsufficientMatchingSignatures => write!(f, "Insufficient matching validator signatures"),
            TxError::InvalidGameId => write!(f, "Invalid game ID"),
            TxError::InvalidFloorNumber => write!(f, "Invalid floor number"),
            TxError::InvalidRewardAmount => write!(f, "Invalid reward amount"),
            TxError::ClaimKeyMismatch => write!(f, "Claim key mismatch"),
        }
    }
}

impl std::error::Error for TxError {}
