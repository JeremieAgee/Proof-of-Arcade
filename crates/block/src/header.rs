use agee_primitives::Hash;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub timestamp_ms: u64,
    pub previous_block_hash: Hash,
    pub tx_root: Hash,
    pub state_root: Hash,
    pub proposer: [u8; 32],
}

impl BlockHeader {
    /// Compute the hash of this block header.
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(self.height.to_le_bytes());
        hasher.update(self.timestamp_ms.to_le_bytes());
        hasher.update(&self.previous_block_hash.0);
        hasher.update(&self.tx_root.0);
        hasher.update(&self.state_root.0);
        hasher.update(&self.proposer);

        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        Hash::new(bytes)
    }
}
