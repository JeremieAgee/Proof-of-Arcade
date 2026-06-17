use crate::state::ChainState;
use agee_consensus::{ValidatorSet, ValidatorInfo};
use agee_primitives::ValidatorId;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Validator entry in genesis config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisValidator {
    pub validator_id: String,
    pub public_key: String,
}

/// Validator set config section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSetConfig {
    pub threshold: u32,
    pub validators: Vec<GenesisValidator>,
}

/// Runtime configuration for the Agee node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub chain_id: [u8; 32],
    pub listen_addr: String,
    pub listen_port: u16,
    pub data_dir: String,
    pub validators: Option<ValidatorSetConfig>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            chain_id: [0u8; 32],
            listen_addr: "127.0.0.1".to_string(),
            listen_port: 8080,
            data_dir: "./data".to_string(),
            validators: None,
        }
    }
}

impl NodeConfig {
    /// Load config from TOML file.
    pub fn from_toml_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    /// Build ValidatorSet from config.
    pub fn build_validator_set(&self) -> ValidatorSet {
        let validators = match &self.validators {
            Some(vc) => {
                let mut vs = ValidatorSet::new(vc.threshold);
                for v in &vc.validators {
                    // Hash validator_id string to [u8; 32]
                    let mut id_bytes = [0u8; 32];
                    let id_hash = blake3::hash(v.validator_id.as_bytes());
                    id_bytes.copy_from_slice(id_hash.as_bytes());

                    // Hash public_key string to [u8; 32]
                    let mut pk_bytes = [0u8; 32];
                    let pk_hash = blake3::hash(v.public_key.as_bytes());
                    pk_bytes.copy_from_slice(pk_hash.as_bytes());

                    vs.add_validator(ValidatorInfo {
                        validator_id: ValidatorId(id_bytes),
                        public_key: pk_bytes,
                        active: true,
                    });
                }
                vs
            }
            None => ValidatorSet::new(1),
        };
        validators
    }
}

/// Shared chain state for the node runtime.
pub struct NodeRuntime {
    pub config: NodeConfig,
    pub state: Arc<RwLock<ChainState>>,
}

impl NodeRuntime {
    pub fn new(config: NodeConfig) -> Self {
        let mut state = ChainState::new();
        // Load validators from config
        state.validators = config.build_validator_set();

        NodeRuntime {
            config,
            state: Arc::new(RwLock::new(state)),
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
