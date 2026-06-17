use crate::state::ChainState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Runtime configuration for the Agee node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub chain_id: [u8; 32],
    pub listen_addr: String,
    pub listen_port: u16,
    pub data_dir: String,
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            chain_id: [0u8; 32],
            listen_addr: "127.0.0.1".to_string(),
            listen_port: 8080,
            data_dir: "./data".to_string(),
        }
    }
}

/// Shared chain state for the node runtime.
pub struct NodeRuntime {
    pub config: NodeConfig,
    pub state: Arc<RwLock<ChainState>>,
}

impl NodeRuntime {
    pub fn new(config: NodeConfig) -> Self {
        NodeRuntime {
            config,
            state: Arc::new(RwLock::new(ChainState::new())),
        }
    }

    pub async fn get_state(&self) -> ChainState {
        self.state.read().await.clone()
    }

    pub async fn mutate_state<F>(&self, f: F)
    where
        F: FnOnce(&mut ChainState),
    {
        let mut state = self.state.write().await;
        f(&mut state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainInfo {
    pub height: u64,
    pub supply: u64,
    pub claimed_floors: usize,
}

impl From<&ChainState> for ChainInfo {
    fn from(state: &ChainState) -> Self {
        ChainInfo {
            height: state.height,
            supply: state.supply.circulating.0,
            claimed_floors: state.claimed_floors.len(),
        }
    }
}
