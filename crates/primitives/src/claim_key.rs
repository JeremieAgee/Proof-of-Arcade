use crate::{AccountId, GameId, RunId, FloorNumber, Hash};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClaimKey(pub [u8; 32]);

impl ClaimKey {
    pub fn new(player: AccountId, game_id: GameId, run_id: RunId, floor_number: FloorNumber) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&player.0);
        hasher.update(game_id.0.to_le_bytes());
        hasher.update(run_id.0.to_le_bytes());
        hasher.update(floor_number.0.to_le_bytes());
        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        ClaimKey(bytes)
    }

    pub fn to_hash(&self) -> Hash {
        Hash::new(self.0)
    }
}
