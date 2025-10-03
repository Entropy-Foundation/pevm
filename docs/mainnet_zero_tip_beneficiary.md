# Mainnet Zero-Tip Beneficiary Hang

## Summary
- **Symptom:** `cargo test --test mainnet mainnet_blocks_from_disk -- --ignored --nocapture` hung with every worker repeatedly logging `tx_idx=1 blocked by tx_idx=0`.
- **Impact:** Parallel execution never progressed past the second transaction for blocks whose first transaction paid no priority fee, so the test (and any production block with the same shape) stalled indefinitely.
- **Root cause:** Ethereum pre-seeds the miner account with `MemoryEntry::Estimate` entries (`crates/pevm/src/chain/ethereum.rs`). When the first transaction does not touch the beneficiary, `MvMemory::new` failed to register that placeholder in `last_locations.write`. The cleanup path in `MvMemory::record` therefore left the stale `Estimate` behind, causing later transactions that read the beneficiary to loop on `ReadError::Blocking(0)`.
- **Fix:** Seed each transaction’s `last_locations.write` with the estimated locations as they are inserted so the cleanup logic clears untouched placeholders (`crates/pevm/src/mv_memory.rs`). With the placeholder removed after tx0 executes, tx1 falls back to storage and the scheduler no longer deadlocks.

## Verification
- Re-ran `cargo test --test mainnet mainnet_blocks_from_disk -- --ignored --nocapture` and confirmed the test completes after the fix.
