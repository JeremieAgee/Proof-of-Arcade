use agee_primitives::CoinAmount;
use serde::{Deserialize, Serialize};

/// Emission schedule for AGEE supply-based halving.
/// Epochs transition based on circulating supply milestones, not time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmissionEpoch {
    Epoch1, // 0 - 10M (multiplier: 1.0000)
    Epoch2, // 10M - 25M (multiplier: 0.5000)
    Epoch3, // 25M - 50M (multiplier: 0.2500)
    Epoch4, // 50M - 75M (multiplier: 0.1250)
    Epoch5, // 75M - 100M (multiplier: 0.0625)
}

impl EmissionEpoch {
    /// Get the reward multiplier for this epoch.
    pub fn multiplier(&self) -> f64 {
        match self {
            EmissionEpoch::Epoch1 => 1.0,
            EmissionEpoch::Epoch2 => 0.5,
            EmissionEpoch::Epoch3 => 0.25,
            EmissionEpoch::Epoch4 => 0.125,
            EmissionEpoch::Epoch5 => 0.0625,
        }
    }

    /// Determine the active epoch based on circulating supply.
    /// The entire claim uses the epoch active at execution start.
    pub fn from_circulating_supply(circulating: u64) -> EmissionEpoch {
        match circulating {
            0..=10_000_000 => EmissionEpoch::Epoch1,
            10_000_001..=25_000_000 => EmissionEpoch::Epoch2,
            25_000_001..=50_000_000 => EmissionEpoch::Epoch3,
            50_000_001..=75_000_000 => EmissionEpoch::Epoch4,
            75_000_001..=100_000_000 => EmissionEpoch::Epoch5,
            _ => EmissionEpoch::Epoch5, // Cap at Epoch5
        }
    }

    /// Get the supply range for this epoch.
    pub fn supply_range(&self) -> (u64, u64) {
        match self {
            EmissionEpoch::Epoch1 => (0, 10_000_000),
            EmissionEpoch::Epoch2 => (10_000_001, 25_000_000),
            EmissionEpoch::Epoch3 => (25_000_001, 50_000_000),
            EmissionEpoch::Epoch4 => (50_000_001, 75_000_000),
            EmissionEpoch::Epoch5 => (75_000_001, 100_000_000),
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
/// raw_reward = coins_collected + floor_depth_bonus
/// coins_collected = sum of coin rarity values (common=1, uncommon=3, rare=5)
/// floor_depth_bonus = floor_number (floor 1 = +1, floor 5 = +5)
///
/// The validator calculates raw reward. The chain applies halving separately.
pub fn calculate_floor_reward(
    total_coin_value: u64,
    floor_number: u32,
    _rules_version: u32,
) -> CoinAmount {
    let coin_reward = total_coin_value;
    let floor_bonus = floor_number as u64;
    let total = coin_reward.saturating_add(floor_bonus);
    CoinAmount::new(total)
}

/// Calculate minted reward after applying emission epoch multiplier.
///
/// The chain determines the active epoch based on current circulating supply.
/// The entire claim uses the epoch active at execution start (not split across epochs).
/// If the minted amount would exceed max supply, we reject (no partial minting).
pub fn apply_emission_multiplier(
    raw_reward: CoinAmount,
    epoch: EmissionEpoch,
) -> CoinAmount {
    let multiplied = raw_reward.0 as f64 * epoch.multiplier();
    CoinAmount::new(multiplied as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_reward_calculation() {
        // 3 common coins (1 each) + floor 1 bonus = 3 + 1 = 4
        let reward = calculate_floor_reward(3, 1, 1);
        assert_eq!(reward.0, 4);
    }

    #[test]
    fn reward_with_mixed_coins() {
        // 1 rare (5) + 2 uncommon (3 each) + floor 5 bonus = 5 + 6 + 5 = 16
        let coin_value = 5 + 3 + 3;
        let reward = calculate_floor_reward(coin_value, 5, 1);
        assert_eq!(reward.0, 16);
    }

    #[test]
    fn epoch_1_full_reward() {
        let raw = CoinAmount::new(100);
        let minted = apply_emission_multiplier(raw, EmissionEpoch::Epoch1);
        assert_eq!(minted.0, 100); // 1.0000x
    }

    #[test]
    fn epoch_2_half_reward() {
        let raw = CoinAmount::new(100);
        let minted = apply_emission_multiplier(raw, EmissionEpoch::Epoch2);
        assert_eq!(minted.0, 50); // 0.5000x
    }

    #[test]
    fn epoch_3_quarter_reward() {
        let raw = CoinAmount::new(100);
        let minted = apply_emission_multiplier(raw, EmissionEpoch::Epoch3);
        assert_eq!(minted.0, 25); // 0.2500x
    }

    #[test]
    fn epoch_4_eighth_reward() {
        let raw = CoinAmount::new(100);
        let minted = apply_emission_multiplier(raw, EmissionEpoch::Epoch4);
        assert_eq!(minted.0, 12); // 0.1250x (rounded)
    }

    #[test]
    fn epoch_5_tiny_reward() {
        let raw = CoinAmount::new(100);
        let minted = apply_emission_multiplier(raw, EmissionEpoch::Epoch5);
        assert_eq!(minted.0, 6); // 0.0625x (rounded)
    }

    #[test]
    fn epoch_from_supply_milestones() {
        assert_eq!(EmissionEpoch::from_circulating_supply(0), EmissionEpoch::Epoch1);
        assert_eq!(
            EmissionEpoch::from_circulating_supply(10_000_000),
            EmissionEpoch::Epoch1
        );
        assert_eq!(
            EmissionEpoch::from_circulating_supply(10_000_001),
            EmissionEpoch::Epoch2
        );
        assert_eq!(
            EmissionEpoch::from_circulating_supply(25_000_000),
            EmissionEpoch::Epoch2
        );
        assert_eq!(
            EmissionEpoch::from_circulating_supply(25_000_001),
            EmissionEpoch::Epoch3
        );
        assert_eq!(
            EmissionEpoch::from_circulating_supply(50_000_001),
            EmissionEpoch::Epoch4
        );
        assert_eq!(
            EmissionEpoch::from_circulating_supply(75_000_001),
            EmissionEpoch::Epoch5
        );
        assert_eq!(
            EmissionEpoch::from_circulating_supply(100_000_000),
            EmissionEpoch::Epoch5
        );
    }

    #[test]
    fn epoch_changes_after_supply_threshold() {
        // At epoch 1 boundary (9.9M)
        let epoch_1_high = EmissionEpoch::from_circulating_supply(9_999_999);
        assert_eq!(epoch_1_high, EmissionEpoch::Epoch1);

        // At epoch 2 boundary (10.1M)
        let epoch_2_low = EmissionEpoch::from_circulating_supply(10_000_001);
        assert_eq!(epoch_2_low, EmissionEpoch::Epoch2);
    }

    #[test]
    fn rounding_consistent() {
        // Test that rounding is consistent
        for raw in [10, 50, 99, 100] {
            let minted = apply_emission_multiplier(CoinAmount::new(raw), EmissionEpoch::Epoch5);
            let expected = (raw as f64 * 0.0625) as u64;
            assert_eq!(minted.0, expected);
        }
    }
}
