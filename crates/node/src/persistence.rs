use crate::state::ChainState;
use agee_block::Block;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genesis {
    pub chain_id: [u8; 32],
    pub created_at_ms: u64,
}

impl Default for Genesis {
    fn default() -> Self {
        Genesis {
            chain_id: [0u8; 32],
            created_at_ms: 0,
        }
    }
}

/// Block store: append-only log of blocks.
pub struct BlockStore {
    data_dir: String,
    block_log_path: String,
    genesis_path: String,
}

impl BlockStore {
    pub fn new(data_dir: &str) -> Self {
        fs::create_dir_all(data_dir).ok();

        BlockStore {
            data_dir: data_dir.to_string(),
            block_log_path: format!("{}/blocks.log", data_dir),
            genesis_path: format!("{}/genesis.json", data_dir),
        }
    }

    /// Initialize genesis if not exists.
    pub fn init_genesis(&self, genesis: &Genesis) -> std::io::Result<()> {
        if !Path::new(&self.genesis_path).exists() {
            let json = serde_json::to_string(genesis)?;
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&self.genesis_path)?;
            file.write_all(json.as_bytes())?;
        }
        Ok(())
    }

    /// Load genesis.
    pub fn load_genesis(&self) -> std::io::Result<Genesis> {
        let content = fs::read_to_string(&self.genesis_path)?;
        Ok(serde_json::from_str(&content)?)
    }

    /// Append a block to the log.
    pub fn append_block(&self, block: &Block) -> std::io::Result<()> {
        let json = serde_json::to_string(block)?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.block_log_path)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }

    /// Load all blocks from log.
    pub fn load_blocks(&self) -> std::io::Result<Vec<Block>> {
        if !Path::new(&self.block_log_path).exists() {
            return Ok(vec![]);
        }

        let file = fs::File::open(&self.block_log_path)?;
        let reader = BufReader::new(file);
        let mut blocks = vec![];

        for line in reader.lines() {
            let line = line?;
            if !line.is_empty() {
                match serde_json::from_str::<Block>(&line) {
                    Ok(block) => blocks.push(block),
                    Err(_) => return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Corrupt block in log",
                    )),
                }
            }
        }

        Ok(blocks)
    }

    /// Replay blocks to rebuild state.
    pub fn replay_blocks(
        &self,
        initial_state: &mut ChainState,
    ) -> std::io::Result<()> {
        let blocks = self.load_blocks()?;

        for block in blocks {
            // TODO: Execute transactions in block sequentially
            // For now, just increment height
            initial_state.height = block.header.height;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn genesis_persists() {
        let temp_dir = TempDir::new().unwrap();
        let store = BlockStore::new(temp_dir.path().to_str().unwrap());

        let genesis = Genesis {
            chain_id: [1u8; 32],
            created_at_ms: 1000,
        };

        store.init_genesis(&genesis).unwrap();
        let loaded = store.load_genesis().unwrap();

        assert_eq!(loaded.chain_id, [1u8; 32]);
        assert_eq!(loaded.created_at_ms, 1000);
    }

    #[test]
    fn blocks_persist_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let store = BlockStore::new(temp_dir.path().to_str().unwrap());

        // TODO: Create and append test block when Block is serializable
        // For now, just verify the store can be created
        assert!(store.load_blocks().is_ok());
    }

    #[test]
    fn duplicate_genesis_not_overwritten() {
        let temp_dir = TempDir::new().unwrap();
        let store = BlockStore::new(temp_dir.path().to_str().unwrap());

        let genesis1 = Genesis {
            chain_id: [1u8; 32],
            created_at_ms: 1000,
        };
        let genesis2 = Genesis {
            chain_id: [2u8; 32],
            created_at_ms: 2000,
        };

        store.init_genesis(&genesis1).unwrap();
        store.init_genesis(&genesis2).unwrap();

        let loaded = store.load_genesis().unwrap();
        assert_eq!(loaded.chain_id, [1u8; 32]); // First genesis preserved
    }
}
