use agee_primitives::{AccountId, CoinAmount};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: CoinAmount,
}
