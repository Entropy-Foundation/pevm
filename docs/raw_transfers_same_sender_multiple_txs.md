# raw_transfers_same_sender_multiple_txs Regression (revm v20)

## Summary
- **Symptom:** `cargo test -p pevm --test raw_transfers raw_transfers_same_sender_multiple_txs -- --ignored` hung indefinitely when the `#[ignore]` was removed.
- **Impact:** Scheduler workers stalled, so the regression blocked parallel execution of sequential-sender blocks.
- **Regression window:** Present after upgrading to `revm` v20; test passed quickly on v19.

## Investigation Notes
- Added temporary logging controlled by `PEVM_PROGRESS` to trace execution, validation, and multi-version (MV) memory writes.
- Logs showed every worker stuck on messages like `blocking total=2 tx=2 incarnation=1 waiting_on=1`.
- MV memory traces exposed the beneficiary account location (`0x710cc12ee46db147`, derived from `Address::ZERO`) perpetually marked as `Estimate` after the first transaction.
- Because we pre-seed beneficiary locations with `Estimate` entries (`chain::ethereum::build_mv_memory`), later transactions interpreted the placeholder as “pending write” and never re-ran.
- Under revm v19 we implicitly rewrote the beneficiary entry during execution; revm v20 stopped touching the coinbase address in this path, leaving the placeholder intact.

## Fix
- When a transaction is lazily updated, only the sender and recipient should use lazy writes. All other accounts (including the beneficiary) now fall back to writing a concrete `MemoryValue::Basic` entry.
- This ensures the beneficiary slot replaces its `Estimate` marker during the first incarnation, letting dependent transactions proceed without scheduler deadlock.

## Follow-up
- Removed the temporary `PEVM_PROGRESS` instrumentation after validating the fix.
- Recommendation: if we add similar diagnostics in the future, guard them behind feature flags or dedicated tracing hooks to avoid touching hot paths.
