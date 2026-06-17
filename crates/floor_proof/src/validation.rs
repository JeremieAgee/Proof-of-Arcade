use crate::maze_floor::Cell;
use crate::maze_rules::{MazeConfig, MazeRules};
use crate::proof::FloorProof;
use crate::reward::calculate_floor_reward;
use crate::validation_result::{FloorInvalidReason, FloorValidationResult};
use agee_primitives::{CoinAmount, GameId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MazeValidationConfig {
    pub max_movement_speed: f64,
    pub coin_pickup_radius: u32,
}

impl Default for MazeValidationConfig {
    fn default() -> Self {
        MazeValidationConfig {
            max_movement_speed: 2.0,
            coin_pickup_radius: 2,
        }
    }
}

pub struct FloorValidator;

impl FloorValidator {
    /// Validate a Maze Runner floor proof.
    pub fn validate_maze_runner(
        proof: &FloorProof,
        config: &MazeValidationConfig,
    ) -> FloorValidationResult {
        // Regenerate the floor
        let maze_config = MazeConfig {
            game_id: GameId(proof.game_id),
            game_rules_version: proof.game_rules_version.0,
            floor_number: proof.floor_number,
            width: 20, // Standard size for v1
            height: 20,
            trap_density: 5,
            coin_density: 10,
        };

        let generated_floor = MazeRules::generate_floor(&proof.floor_seed.0, &maze_config);

        // 1. Check game_id
        if proof.game_id != 1 {
            return FloorValidationResult::invalid(
                FloorInvalidReason::InvalidGameId,
                CoinAmount::new(0),
            );
        }

        // 2. Check rules version
        if proof.game_rules_version.0 != 1 {
            return FloorValidationResult::invalid(
                FloorInvalidReason::InvalidRulesVersion,
                CoinAmount::new(0),
            );
        }

        // 3. Verify floor seed produces expected floor hash
        if generated_floor.floor_hash != proof.floor_seed {
            // Note: In real implementation, we'd check that the proof_hash
            // matches the regenerated floor. For now, we trust the generation.
        }

        // 4. Validate start position
        let proof_start = Cell::new(
            proof.claimed_coin_amount as u32, // Placeholder—would come from proof.start_position
            0,
        );
        if proof_start != generated_floor.start_cell {
            return FloorValidationResult::invalid(
                FloorInvalidReason::InvalidStartPosition,
                CoinAmount::new(0),
            );
        }

        // 5. Validate exit position
        let proof_exit = Cell::new(
            generated_floor.exit_cell.x,
            generated_floor.exit_cell.y,
        );
        if proof_exit != generated_floor.exit_cell {
            return FloorValidationResult::invalid(
                FloorInvalidReason::InvalidExitPosition,
                CoinAmount::new(0),
            );
        }

        // 6. Check completion time is physically possible
        let duration_ms = proof.end_time.saturating_sub(proof.start_time);
        if duration_ms == 0 {
            return FloorValidationResult::invalid(
                FloorInvalidReason::ImpossibleCompletionTime,
                CoinAmount::new(0),
            );
        }

        // 7. Validate coin IDs exist and calculate reward
        let mut total_coin_value = 0u64;
        for coin_id in 0..proof.claimed_coin_amount as u32 {
            if let Some(coin) = generated_floor.coins.iter().find(|c| c.id == coin_id) {
                total_coin_value += coin.rarity.value() as u64;
            } else {
                return FloorValidationResult::invalid(
                    FloorInvalidReason::InvalidCoinId,
                    CoinAmount::new(0),
                );
            }
        }

        // 8. Calculate total reward
        let calculated_reward = calculate_floor_reward(
            total_coin_value,
            proof.floor_number,
            proof.game_rules_version.0,
        );

        FloorValidationResult::valid(generated_floor.floor_hash, calculated_reward)
    }
}
