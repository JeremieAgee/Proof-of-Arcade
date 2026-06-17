# Agee Arcade Coin / Proof-of-Arcade

## 🔒 THE CORE INVARIANT

**No AGEE can enter circulation except through `execute_claim_floor_reward()`.**

Do not:
```rust
balance += amount;    // ❌ NO
supply += amount;     // ❌ NO
mint(account, amount);  // ❌ NO
```

Do only:
```rust
execute_claim_floor_reward(state, tx)  // ✅ THE GATE
```

This invariant **is** the coin. If there's a backdoor to minting, the economy breaks immediately. No presales, no inflation, no shortcuts.

---

## Project Vision

Agee Coin is mined through verified arcade gameplay, not presales or paid entry. Players complete floors, collect coins, survive hazards—validator nodes verify proofs, and the Rust chain mints coins only if validators approve.

**Zero presale, zero paid entry, zero casino mechanics.** Score ≠ coin. Gameplay = mining.

---

## Architecture

### Crates

- **primitives** — Core types: `AccountId`, `CoinAmount`, `Hash`, `Signature`, game/run/floor IDs
- **ledger** — Wallet state: `Balance`, `CoinGrant` (with source/status history), `SupplyTracker` (capped at 100M, supply-based halving)
- **floor_proof** — Floor validation types: `FloorProof`, `FloorValidator`, Maze Runner rules, emission epochs
- **tx** — Transaction types: `ClaimFloorReward`, `Transfer`, `Burn`
- **block** — Block structures: `BlockHeader`, `Block`, merkle roots
- **consensus** — Validator consensus: `ValidatorSet`, voting (M-of-N), finality rules
- **node** — Chain state: `ChainState` (balances, supply, claimed floors), mempool, JSON RPC API stubs
- **cli** — Binary: CLI tool to query/submit transactions

### Data Flow

```
Player plays floor → client submits FloorProof → validator nodes verify → 
  enough signatures → ClaimFloorReward tx → chain mints AGEE or rejects
```

---

## Wallet Model

**PlayerProfile** (persistent):
- `maze_coins` — arcade coins (may not be mintable)
- `mintable_coins` — eligible to claim on-chain
- `locked_coins` — burned or reserved

**GameState** (temporary):
- `run_coins` — coins collected in current run
- `floor_coins` — coins on current floor

**Ledger entry** for every grant:
```rust
CoinGrant {
  id, account_id, amount,
  source: MazePickup | FloorComplete | FloorMilestone | BossClear | SeasonReward | ...,
  game_id, run_id, floor_number,
  mint_eligible: bool,
  status: LocalUnverified | FloorVerified | MintEligible | MintClaimed | Burned | Rejected,
  created_at
}
```

---

## Key Invariants

1. **No double-claim**: Each (player, game, run, floor) can be claimed once. Tracked via `claimed_floors: HashSet<String>` in `ChainState`.
2. **Client untrusted**: Validators recalculate reward and verify collisions/traps/coins.
3. **Every coin has a ledger**: No direct balance mutations. Coin must come from a grant with source.
4. **Emission capped**: Supply halves at milestones (10M, 25M, 50M, 75M). Validators must check epoch multiplier before minting.
5. **Floor proof immutable**: Once submitted, floor cannot be re-validated with different claims.

---

## Development Path

**Stage 1: Local Arcade Wallet** ← Next
- Persistent `PlayerProfile` with wallet balances
- Non-mintable maze pickups by default
- No ads, no paid entry

**Stage 2: Ledger**
- `CoinGrant` history for every balance change
- Ledger queries

**Stage 3: Floor Receipts**
- `FloorResult` object from each floor

**Stage 4: Local Floor Validator**
- Maze Runner validator: regenerates maze, checks collisions, verifies coins/traps/time

**Stage 5: Rust Chain v0**
- `ChainState`: balances, supply, claimed floors
- `ClaimFloorReward` tx mints coins if valid
- Duplicate prevention

**Stage 6: Multi-Node Validation**
- M-of-N validator signatures required

**Stage 7: Burn + NFT**
- Coins burn for cosmetics, trophies, achievement NFTs

**Stage 8: Public Testnet**
- Test AGEE mining

**Stage 9: Mainnet**
- Real coin, supply starts at zero

---

## Commit Conventions

Branch naming:
- `stage/<number>` for development (e.g., `stage/1`, `stage/2`)
- `feature/<name>` for specific features
- `fix/<name>` for bug fixes

Commit messages:
```
[Stage N] Brief description of change

Longer explanation if needed. Reference which crates changed.
```

Examples:
```
[Stage 1] Add PlayerProfile wallet structure
[Stage 2] Implement CoinGrant ledger with source tracking
[Stage 3] Define FloorResult struct for Maze Runner
[Stage 4] Add Maze Runner floor validator (seeded PRNG regeneration)
```

---

## How to Add a New Game Validator

1. Create a new crate `crates/game_<name>_validator/`
2. Implement `GameValidator` trait in `floor_proof`:
   ```rust
   pub trait GameValidator {
       fn validate_floor(proof: &FloorProof) -> ValidationResult;
       fn regenerate_state(seed: &[u8; 32]) -> GameState;
   }
   ```
3. Add to `FloorValidator::dispatch()` in `floor_proof/src/validation.rs`
4. Validators will call the appropriate validator based on `floor_proof.game_id`

---

## Testing

Run tests with:
```bash
cargo test
```

Unit tests should exist for:
- Ledger balance updates
- Supply cap enforcement
- Duplicate claim detection
- Emission epoch calculations
- Floor proof validation (mock validator)

---

## API Endpoints (Future)

```
POST /claim-floor       → ClaimFloorReward
GET  /balance/:account  → Balance
GET  /supply            → SupplyTracker
GET  /block/:height     → Block
```

---

## Key Files to Know

- `crates/primitives/src/account.rs` — Account types
- `crates/ledger/src/grant.rs` — CoinGrant structure (the audit trail)
- `crates/ledger/src/supply.rs` — Supply cap and halving logic
- `crates/tx/src/claim_floor.rs` — The core transaction that mints coins
- `crates/node/src/state.rs` — ChainState with balances and claimed floors
- `crates/floor_proof/src/validation.rs` — Entry point for floor verification

---

## Running the CLI

```bash
cargo build --release
./target/release/agee balance <account>
./target/release/agee supply
```

---

## Notes

- Edition 2024 (latest Rust edition)
- Workspace resolver v2
- All deps are serde, serde_json, sha2 (minimal, intentional)
- No external blockchain libraries yet—building from primitives
