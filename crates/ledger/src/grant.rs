use agee_primitives::{AccountId, CoinAmount, GameRulesVersion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantSource {
    MazePickup,
    FloorComplete,
    FloorMilestone,
    BossClear,
    SeasonReward,
    BurnRefund,
    AdminAdjustment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantStatus {
    LocalUnverified,
    FloorVerified,
    MintEligible,
    MintClaimed,
    Burned,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinGrant {
    pub id: u64,
    pub account_id: AccountId,
    pub amount: CoinAmount,
    pub source: GrantSource,
    pub game_id: Option<u32>,
    pub game_rules_version: Option<GameRulesVersion>,
    pub run_id: Option<u64>,
    pub floor_number: Option<u32>,
    pub mint_eligible: bool,
    pub status: GrantStatus,
    pub created_at: u64,
}

impl CoinGrant {
    pub fn new(
        id: u64,
        account_id: AccountId,
        amount: CoinAmount,
        source: GrantSource,
    ) -> Self {
        CoinGrant {
            id,
            account_id,
            amount,
            source,
            game_id: None,
            game_rules_version: None,
            run_id: None,
            floor_number: None,
            mint_eligible: false,
            status: GrantStatus::LocalUnverified,
            created_at: 0,
        }
    }
}
