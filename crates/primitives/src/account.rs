use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(pub [u8; 32]);

impl AccountId {
    pub fn new(bytes: [u8; 32]) -> Self {
        AccountId(bytes)
    }
}
