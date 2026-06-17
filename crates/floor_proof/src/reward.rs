use agee_primitives::CoinAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EmissionEpoch {
    Epoch1,
    Epoch2,
    Epoch3,
    Epoch4,
    Epoch5,
}

impl EmissionEpoch {
    pub fn multiplier(&self) -> f64 {
        match self {
            EmissionEpoch::Epoch1 => 1.0,
            EmissionEpoch::Epoch2 => 0.5,
            EmissionEpoch::Epoch3 => 0.25,
            EmissionEpoch::Epoch4 => 0.125,
            EmissionEpoch::Epoch5 => 0.0625,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FloorReward {
    pub base_amount: CoinAmount,
    pub epoch: EmissionEpoch,
}

impl FloorReward {
    pub fn calculated(&self) -> CoinAmount {
        let multiplied = self.base_amount.0 as f64 * self.epoch.multiplier();
        CoinAmount::new(multiplied as u64)
    }
}

/// Calculate raw floor reward (before emission halving).
///
/// Formula for v1:
/// base_reward = coins_collected + floor_depth_bonus
/// coins_collected = sum of coin rarity values (common=1, uncommon=3, rare=5)
/// floor_depth_bonus = floor_number (floor 1 = +1, floor 5 = +5)
///
/// The emission layer applies halving separately.
pub fn calculate_floor_reward(
    total_coin_value: u64,
    floor_number: u32,
    _rules_version: u32,
) -> CoinAmount {
    // Coin value (already summed from rarity)
    let coin_reward = total_coin_value;

    // Floor completion bonus = floor number
    let floor_bonus = floor_number as u64;

    let total = coin_reward.saturating_add(floor_bonus);
    CoinAmount::new(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reward_floor_1_common_coins() {
        // 3 common coins (1 each) + floor 1 bonus = 3 + 1 = 4
        let reward = calculate_floor_reward(3, 1, 1);
        assert_eq!(reward.0, 4);
    }

    #[test]
    fn reward_floor_5_mixed_coins() {
        // 1 rare (5) + 2 uncommon (3 each) + floor 5 bonus = 5 + 6 + 5 = 16
        let coin_value = 5 + 3 + 3;
        let reward = calculate_floor_reward(coin_value, 5, 1);
        assert_eq!(reward.0, 16);
    }
}
