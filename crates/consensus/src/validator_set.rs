use agee_primitives::AccountId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    validators: HashSet<AccountId>,
}

impl ValidatorSet {
    pub fn new() -> Self {
        ValidatorSet {
            validators: HashSet::new(),
        }
    }

    pub fn add_validator(&mut self, account: AccountId) {
        self.validators.insert(account);
    }

    pub fn is_validator(&self, account: AccountId) -> bool {
        self.validators.contains(&account)
    }

    pub fn size(&self) -> usize {
        self.validators.len()
    }
}
