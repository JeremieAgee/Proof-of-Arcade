use agee_primitives::{CoinAmount, Hash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloorInvalidReason {
    InvalidGameId,
    InvalidRulesVersion,
    InvalidSeed,
    InvalidStartPosition,
    InvalidExitPosition,
    CheckpointsOutOfOrder,
    ImpossibleSpeed,
    WallClip,
    InvalidCoinId,
    CoinTooFarFromPath,
    TrapMismatch,
    ImpossibleCompletionTime,
    RewardMismatch,
    UnexpectedError,
}

impl std::fmt::Display for FloorInvalidReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloorInvalidReason::InvalidGameId => write!(f, "Invalid game ID"),
            FloorInvalidReason::InvalidRulesVersion => write!(f, "Invalid rules version"),
            FloorInvalidReason::InvalidSeed => write!(f, "Invalid seed"),
            FloorInvalidReason::InvalidStartPosition => write!(f, "Invalid start position"),
            FloorInvalidReason::InvalidExitPosition => write!(f, "Invalid exit position"),
            FloorInvalidReason::CheckpointsOutOfOrder => write!(f, "Checkpoints out of order"),
            FloorInvalidReason::ImpossibleSpeed => write!(f, "Movement speed is impossible"),
            FloorInvalidReason::WallClip => write!(f, "Path clips through walls"),
            FloorInvalidReason::InvalidCoinId => write!(f, "Invalid coin ID"),
            FloorInvalidReason::CoinTooFarFromPath => write!(f, "Coin too far from path"),
            FloorInvalidReason::TrapMismatch => write!(f, "Trap collision mismatch"),
            FloorInvalidReason::ImpossibleCompletionTime => {
                write!(f, "Completion time is impossible")
            }
            FloorInvalidReason::RewardMismatch => write!(f, "Reward mismatch"),
            FloorInvalidReason::UnexpectedError => write!(f, "Unexpected validation error"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorValidationResult {
    pub valid: bool,
    pub floor_proof_hash: Hash,
    pub calculated_reward: CoinAmount,
    pub reason: Option<FloorInvalidReason>,
}

impl FloorValidationResult {
    pub fn valid(floor_proof_hash: Hash, calculated_reward: CoinAmount) -> Self {
        FloorValidationResult {
            valid: true,
            floor_proof_hash,
            calculated_reward,
            reason: None,
        }
    }

    pub fn invalid(reason: FloorInvalidReason, calculated_reward: CoinAmount) -> Self {
        FloorValidationResult {
            valid: false,
            floor_proof_hash: Hash::new([0u8; 32]),
            calculated_reward,
            reason: Some(reason),
        }
    }
}
