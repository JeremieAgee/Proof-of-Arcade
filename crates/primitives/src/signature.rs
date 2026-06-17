use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(pub Vec<u8>);

impl Signature {
    pub fn new(bytes: Vec<u8>) -> Self {
        Signature(bytes)
    }
}
