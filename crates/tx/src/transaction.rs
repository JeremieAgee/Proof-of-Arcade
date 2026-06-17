use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
    ClaimFloorReward(super::claim_floor::ClaimFloorReward),
    Transfer(super::transfer::Transfer),
    Burn(super::burn::Burn),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEnvelope {
    pub tx: Transaction,
    pub nonce: u64,
    pub signature: Option<Vec<u8>>,
}
