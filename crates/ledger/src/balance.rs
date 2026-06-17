use agee_primitives::{AccountId, CoinAmount};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub account_id: AccountId,
    pub maze_coins: CoinAmount,
    pub mintable_coins: CoinAmount,
    pub locked_coins: CoinAmount,
}

impl Balance {
    pub fn new(account_id: AccountId) -> Self {
        Balance {
            account_id,
            maze_coins: CoinAmount::new(0),
            mintable_coins: CoinAmount::new(0),
            locked_coins: CoinAmount::new(0),
        }
    }

    pub fn total(&self) -> CoinAmount {
        self.maze_coins
            .saturating_add(self.mintable_coins)
            .saturating_add(self.locked_coins)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balances {
    accounts: HashMap<AccountId, Balance>,
}

impl Balances {
    pub fn new() -> Self {
        Balances {
            accounts: HashMap::new(),
        }
    }

    pub fn get_or_create(&mut self, account_id: AccountId) -> &mut Balance {
        self.accounts
            .entry(account_id)
            .or_insert_with(|| Balance::new(account_id))
    }

    pub fn get(&self, account_id: AccountId) -> Option<&Balance> {
        self.accounts.get(&account_id)
    }
}
