use crate::executor::execute_claim_floor_reward;
use crate::runtime::{ChainInfo, NodeRuntime};
use agee_primitives::{AccountId, ClaimKey, Hash};
use agee_tx::{ClaimFloorReward, TxError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub account_id: String,
    pub maze_coins: u64,
    pub mintable_coins: u64,
    pub locked_coins: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyResponse {
    pub circulating: u64,
    pub burned: u64,
    pub max_supply: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimCheckResponse {
    pub claim_key: String,
    pub is_claimed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub success: bool,
    pub error: Option<String>,
}

pub fn create_router(runtime: Arc<NodeRuntime>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/chain/info", get(chain_info))
        .route("/balance/:account", get(get_balance))
        .route("/supply", get(get_supply))
        .route("/claim/:claim_key", get(check_claim))
        .route("/tx/claim-floor", post(submit_claim))
        .with_state(runtime)
}

async fn health() -> &'static str {
    "ok"
}

async fn chain_info(State(runtime): State<Arc<NodeRuntime>>) -> Json<ChainInfo> {
    let state = runtime.get_state().await;
    Json(ChainInfo::from(&state))
}

async fn get_balance(
    State(runtime): State<Arc<NodeRuntime>>,
    Path(account_str): Path<String>,
) -> Result<Json<BalanceResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Parse account ID from hex string
    if account_str.len() != 64 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid account ID format".to_string(),
                code: 400,
            }),
        ));
    }

    let mut account_bytes = [0u8; 32];
    for i in 0..32 {
        account_bytes[i] = u8::from_str_radix(&account_str[i * 2..i * 2 + 2], 16)
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Invalid hex in account ID".to_string(),
                        code: 400,
                    }),
                )
            })?;
    }

    let account_id = AccountId::new(account_bytes);
    let state = runtime.get_state().await;

    match state.balances.get(account_id) {
        Some(balance) => Ok(Json(BalanceResponse {
            account_id: account_str,
            maze_coins: balance.maze_coins().0,
            mintable_coins: balance.mintable_coins().0,
            locked_coins: balance.locked_coins().0,
            total: balance.total().0,
        })),
        None => Ok(Json(BalanceResponse {
            account_id: account_str,
            maze_coins: 0,
            mintable_coins: 0,
            locked_coins: 0,
            total: 0,
        })),
    }
}

async fn get_supply(State(runtime): State<Arc<NodeRuntime>>) -> Json<SupplyResponse> {
    let state = runtime.get_state().await;
    let max_supply = agee_ledger::SupplyTracker::max_supply();

    Json(SupplyResponse {
        circulating: state.supply.circulating.0,
        burned: state.supply.burned.0,
        max_supply: max_supply.0,
    })
}

async fn check_claim(
    State(runtime): State<Arc<NodeRuntime>>,
    Path(claim_key_str): Path<String>,
) -> Result<Json<ClaimCheckResponse>, (StatusCode, Json<ErrorResponse>)> {
    if claim_key_str.len() != 64 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid claim key format".to_string(),
                code: 400,
            }),
        ));
    }

    let mut claim_bytes = [0u8; 32];
    for i in 0..32 {
        claim_bytes[i] = u8::from_str_radix(&claim_key_str[i * 2..i * 2 + 2], 16)
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Invalid hex in claim key".to_string(),
                        code: 400,
                    }),
                )
            })?;
    }

    let claim_key = ClaimKey(claim_bytes);
    let state = runtime.get_state().await;
    let is_claimed = state.claimed_floors.contains(&claim_key.0);

    Ok(Json(ClaimCheckResponse {
        claim_key: claim_key_str,
        is_claimed,
    }))
}

async fn submit_claim(
    State(runtime): State<Arc<NodeRuntime>>,
    Json(tx): Json<ClaimFloorReward>,
) -> Result<Json<TransactionResponse>, (StatusCode, Json<ErrorResponse>)> {
    let result = {
        let mut state = runtime.state.write().await;
        execute_claim_floor_reward(&mut state, &tx)
    };

    match result {
        Ok(()) => Ok(Json(TransactionResponse {
            success: true,
            error: None,
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
                code: 400,
            }),
        )),
    }
}
