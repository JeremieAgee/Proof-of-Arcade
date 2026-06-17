use agee_primitives::{AccountId, CoinAmount};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Account balance.
///
/// IMPORTANT: Fields are private to enforce the core invariant:
/// No AGEE enters circulation except through execute_claim_floor_reward().
///
/// All state mutations must go through grant_mintable() or burn(),
/// which are only called from the claim executor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    account_id: AccountId,
    maze_coins: CoinAmount,
    mintable_coins: CoinAmount,
    locked_coins: CoinAmount,
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

    pub fn account_id(&self) -> AccountId {
        self.account_id
    }

    pub fn maze_coins(&self) -> CoinAmount {
        self.maze_coins
    }

    pub fn mintable_coins(&self) -> CoinAmount {
        self.mintable_coins
    }

    pub fn locked_coins(&self) -> CoinAmount {
        self.locked_coins
    }

    pub fn total(&self) -> CoinAmount {
        self.maze_coins
            .saturating_add(self.mintable_coins)
            .saturating_add(self.locked_coins)
    }

    /// Mint coins to this balance. Only called from execute_claim_floor_reward.
    pub fn grant_mintable(&mut self, amount: CoinAmount) {
        self.mintable_coins = self.mintable_coins.saturating_add(amount);
    }

    /// Burn coins from this balance.
    pub fn burn(&mut self, amount: CoinAmount) {
        self.locked_coins = self.locked_coins.saturating_sub(amount);
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
