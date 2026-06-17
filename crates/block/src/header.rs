use agee_primitives::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub timestamp: u64,
    pub parent_hash: Hash,
    pub transactions_root: Hash,
    pub state_root: Hash,
    pub proposer: [u8; 32],
}
