use crate::maze_floor::{Cell, CoinRarity, CoinSpawn, GeneratedMazeFloor, Trap, Wall};
use crate::prng::SplitMix64;
use agee_primitives::{GameId, Hash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MazeConfig {
    pub game_id: GameId,
    pub game_rules_version: u32,
    pub floor_number: u32,
    pub width: u32,
    pub height: u32,
    pub trap_density: u32,
    pub coin_density: u32,
}

pub struct MazeRules;

impl MazeRules {
    /// Generate a deterministic Maze Runner floor from seed and config.
    pub fn generate_floor(seed: &[u8; 32], config: &MazeConfig) -> GeneratedMazeFloor {
        let mut rng = SplitMix64::from_bytes(seed);

        // Start cell: always (1, 1)
        let start_cell = Cell::new(1, 1);

        // Exit cell: somewhere near the opposite corner
        let exit_x = (config.width - 2).max(1);
        let exit_y = (config.height - 2).max(1);
        let exit_cell = Cell::new(exit_x, exit_y);

        // Generate walls (simple: create a perimeter + some internal walls)
        let mut walls = vec![];

        // Perimeter walls
        for x in 0..config.width {
            walls.push(Wall {
                from: Cell::new(x, 0),
                to: Cell::new(x, 1),
            });
            walls.push(Wall {
                from: Cell::new(x, config.height - 1),
                to: Cell::new(x, config.height - 2),
            });
        }
        for y in 0..config.height {
            walls.push(Wall {
                from: Cell::new(0, y),
                to: Cell::new(1, y),
            });
            walls.push(Wall {
                from: Cell::new(config.width - 1, y),
                to: Cell::new(config.width - 2, y),
            });
        }

        // Internal walls: place deterministically using PRNG
        let max_internal_walls = (config.width * config.height) / 10;
        for _ in 0..max_internal_walls {
            let x = rng.next_range(config.width);
            let y = rng.next_range(config.height);
            // Skip start and exit
            if (x == start_cell.x && y == start_cell.y) || (x == exit_cell.x && y == exit_cell.y) {
                continue;
            }
            // Random wall direction
            if rng.next_u32() % 2 == 0 && x + 1 < config.width {
                walls.push(Wall {
                    from: Cell::new(x, y),
                    to: Cell::new(x + 1, y),
                });
            } else if y + 1 < config.height {
                walls.push(Wall {
                    from: Cell::new(x, y),
                    to: Cell::new(x, y + 1),
                });
            }
        }

        // Generate traps
        let mut traps = vec![];
        let trap_count = (config.width * config.height * config.trap_density) / 100;
        for _ in 0..trap_count {
            let x = rng.next_range(config.width);
            let y = rng.next_range(config.height);
            if (x == start_cell.x && y == start_cell.y) || (x == exit_cell.x && y == exit_cell.y) {
                continue;
            }
            let damage = (rng.next_u32() % 30) + 10; // 10-40 damage
            traps.push(Trap {
                cell: Cell::new(x, y),
                damage,
            });
        }

        // Generate coins
        let mut coins = vec![];
        let coin_count = (config.width * config.height * config.coin_density) / 100;
        for i in 0..coin_count {
            let x = rng.next_range(config.width);
            let y = rng.next_range(config.height);
            if (x == start_cell.x && y == start_cell.y) || (x == exit_cell.x && y == exit_cell.y) {
                continue;
            }
            let rarity_roll = rng.next_u32() % 100;
            let rarity = if rarity_roll < 70 {
                CoinRarity::Common
            } else if rarity_roll < 95 {
                CoinRarity::Uncommon
            } else {
                CoinRarity::Rare
            };
            coins.push(CoinSpawn {
                id: i,
                cell: Cell::new(x, y),
                rarity,
            });
        }

        let mut floor = GeneratedMazeFloor {
            game_id: config.game_id,
            game_rules_version: config.game_rules_version,
            floor_number: config.floor_number,
            width: config.width,
            height: config.height,
            start_cell,
            exit_cell,
            walls,
            traps,
            coins,
            floor_hash: Hash::new([0u8; 32]),
        };

        floor.floor_hash = floor.compute_hash();
        floor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maze_generation_is_deterministic() {
        let seed = [0u8; 32];
        let config = MazeConfig {
            game_id: GameId(1),
            game_rules_version: 1,
            floor_number: 1,
            width: 20,
            height: 20,
            trap_density: 5,
            coin_density: 10,
        };

        let floor_a = MazeRules::generate_floor(&seed, &config);
        let floor_b = MazeRules::generate_floor(&seed, &config);
        let floor_c = MazeRules::generate_floor(&seed, &config);

        assert_eq!(floor_a.floor_hash, floor_b.floor_hash);
        assert_eq!(floor_b.floor_hash, floor_c.floor_hash);
    }

    #[test]
    fn different_seeds_produce_different_floors() {
        let config = MazeConfig {
            game_id: GameId(1),
            game_rules_version: 1,
            floor_number: 1,
            width: 20,
            height: 20,
            trap_density: 5,
            coin_density: 10,
        };

        let floor_a = MazeRules::generate_floor(&[0u8; 32], &config);
        let mut seed_b = [0u8; 32];
        seed_b[0] = 1;
        let floor_b = MazeRules::generate_floor(&seed_b, &config);

        assert_ne!(floor_a.floor_hash, floor_b.floor_hash);
    }

    #[test]
    fn different_rules_version_produces_different_floors() {
        let seed = [0u8; 32];

        let mut config_v1 = MazeConfig {
            game_id: GameId(1),
            game_rules_version: 1,
            floor_number: 1,
            width: 20,
            height: 20,
            trap_density: 5,
            coin_density: 10,
        };

        let floor_v1 = MazeRules::generate_floor(&seed, &config_v1);

        config_v1.game_rules_version = 2;
        let floor_v2 = MazeRules::generate_floor(&seed, &config_v1);

        assert_ne!(floor_v1.floor_hash, floor_v2.floor_hash);
    }
}
