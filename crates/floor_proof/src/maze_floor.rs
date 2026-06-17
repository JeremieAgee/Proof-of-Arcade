use agee_primitives::{GameId, Hash};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Cell { x, y }
    }

    pub fn distance_to(&self, other: Cell) -> u32 {
        let dx = (self.x as i32 - other.x as i32).abs() as u32;
        let dy = (self.y as i32 - other.y as i32).abs() as u32;
        dx.max(dy)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wall {
    pub from: Cell,
    pub to: Cell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trap {
    pub cell: Cell,
    pub damage: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoinRarity {
    Common,
    Uncommon,
    Rare,
}

impl CoinRarity {
    pub fn value(&self) -> u32 {
        match self {
            CoinRarity::Common => 1,
            CoinRarity::Uncommon => 3,
            CoinRarity::Rare => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoinSpawn {
    pub id: u32,
    pub cell: Cell,
    pub rarity: CoinRarity,
}

/// Deterministically generated Maze Runner floor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedMazeFloor {
    pub game_id: GameId,
    pub game_rules_version: u32,
    pub floor_number: u32,

    pub width: u32,
    pub height: u32,

    pub start_cell: Cell,
    pub exit_cell: Cell,

    pub walls: Vec<Wall>,
    pub traps: Vec<Trap>,
    pub coins: Vec<CoinSpawn>,

    pub floor_hash: Hash,
}

impl GeneratedMazeFloor {
    /// Compute the hash of this floor for validation.
    pub fn compute_hash(&self) -> Hash {
        let mut hasher = Sha256::new();

        // Hash all deterministic properties
        hasher.update(self.game_id.0.to_le_bytes());
        hasher.update(self.game_rules_version.to_le_bytes());
        hasher.update(self.floor_number.to_le_bytes());
        hasher.update(self.width.to_le_bytes());
        hasher.update(self.height.to_le_bytes());

        hasher.update(self.start_cell.x.to_le_bytes());
        hasher.update(self.start_cell.y.to_le_bytes());
        hasher.update(self.exit_cell.x.to_le_bytes());
        hasher.update(self.exit_cell.y.to_le_bytes());

        for wall in &self.walls {
            hasher.update(wall.from.x.to_le_bytes());
            hasher.update(wall.from.y.to_le_bytes());
            hasher.update(wall.to.x.to_le_bytes());
            hasher.update(wall.to.y.to_le_bytes());
        }

        for trap in &self.traps {
            hasher.update(trap.cell.x.to_le_bytes());
            hasher.update(trap.cell.y.to_le_bytes());
            hasher.update(trap.damage.to_le_bytes());
        }

        for coin in &self.coins {
            hasher.update(coin.id.to_le_bytes());
            hasher.update(coin.cell.x.to_le_bytes());
            hasher.update(coin.cell.y.to_le_bytes());
            hasher.update((coin.rarity as u32).to_le_bytes());
        }

        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        Hash::new(bytes)
    }
}
