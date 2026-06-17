use crate::attestation_verifier::verify_claim_attestations;
use crate::state::ChainState;
use agee_primitives::{ClaimKey, FloorNumber};
use agee_tx::{ClaimFloorReward, TxError};

/// Apply a ClaimFloorReward transaction atomically.
///
/// Follows design rules:
/// 1. Check claim_key not already claimed
/// 2. Verify validator attestations meet threshold
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
        tx.game_id,
        tx.run_id,
        FloorNumber(tx.floor_number),
    );

    if state.is_floor_claimed(&claim_key) {
        return Err(TxError::DuplicateFloorClaim);
    }

    // 2. Verify validator attestations
    let verified = verify_claim_attestations(state, tx)?;

    // 3. Verify claim key matches
    if verified.claim_key != claim_key {
        return Err(TxError::ClaimKeyMismatch);
    }

    // 4. Check max supply
    if !state.supply.can_mint(verified.reward) {
        return Err(TxError::MaxSupplyExceeded);
    }

    // 5. Atomic mutations (all succeed or all fail)
    state.mark_floor_claimed(claim_key);

    // Mint to player balance
    let player_balance = state.balances.get_or_create(tx.player);
    player_balance.grant_mintable(verified.reward);

    // Increase circulating supply
    state.supply.mint(verified.reward);

    // TODO: Add ledger entry with grant source = FloorComplete, status = MintClaimed

    Ok(())
}
