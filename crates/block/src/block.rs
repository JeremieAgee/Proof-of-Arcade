use agee_tx::Transaction;
use serde::{Deserialize, Serialize};

use super::header::BlockHeader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
