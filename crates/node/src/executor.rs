use agee_primitives::{ClaimKey, GameId, RunId, FloorNumber};
use agee_tx::{ClaimFloorReward, TxError};
use crate::state::ChainState;

/// Apply a ClaimFloorReward transaction atomically.
///
/// Follows design rules:
/// 1. Check claim_key not already claimed
/// 2. Verify validator signatures (stub)
/// 3. Verify calculated reward matches claimed
/// 4. Check emission rules / max supply
/// 5. Atomically: insert claim_key + mint balance + increase supply
pub fn execute_claim_floor_reward(
    state: &mut ChainState,
    tx: &ClaimFloorReward,
) -> Result<(), TxError> {
    // 1. Compute claim key and check for duplicates
    let claim_key = ClaimKey::new(
        tx.player,
        GameId(tx.game_id),
        RunId(tx.run_id),
        FloorNumber(tx.floor_number),
    );

    if state.is_floor_claimed(&claim_key) {
        return Err(TxError::DuplicateFloorClaim);
    }

    // 2. Verify validator signatures (TODO: implement full verification in Stage 6)
    // For now, we accept that signatures were passed in.
    if tx.validator_signatures.is_empty() {
        return Err(TxError::InvalidValidatorSignature);
    }

    // 3. Verify claimed amount matches validated reward
    // TODO: Decode signatures to extract ValidatorAttestation and verify reward
    // For now, we assume validator_signatures encode the approved reward.

    // 4. Check emission rules and max supply
    if !state.supply.can_mint(tx.claimed_amount) {
        return Err(TxError::MaxSupplyExceeded);
    }

    // 5. Atomic mutations (all succeed or all fail)
    state.mark_floor_claimed(claim_key);

    // Mint to player balance
    let player_balance = state.balances.get_or_create(tx.player);
    player_balance.grant_mintable(tx.claimed_amount);

    // Increase circulating supply
    state.supply.mint(tx.claimed_amount);

    // TODO: Add ledger entry with grant source = FloorComplete, status = MintClaimed

    Ok(())
}
