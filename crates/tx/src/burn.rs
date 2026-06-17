use agee_primitives::{AccountId, CoinAmount};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Burn {
    pub account: AccountId,
    pub amount: CoinAmount,
    pub burn_type: String,
}
