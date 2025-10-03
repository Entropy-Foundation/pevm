# Bytecode Canonicalization Fixes

## Background
- `mainnet_blocks_from_disk` failed because the parallel executor stored newly created contracts using unanalyzed legacy bytecode (missing the jump-table padding). Sequential execution worked with analyzed code, so direct comparisons of transaction results diverged.
- Normalizing bytecode fixed the functional issue but exposed that the EOF unit test fixture in `storage::tests::eof_bytecodes` no longer matched the upstream REVM sample, causing a secondary failure.

## Changes
- Stored canonical (analyzed) bytecode in `mv_memory.new_bytecodes` by converting any fresh contract code through `canonicalize_bytecode` inside `Vm::execute`.
- Ensured `EvmCode::from(Bytecode::LegacyAnalyzed(_))` re-analyzes raw legacy code via `LegacyRawBytecode::into_analyzed()` so conversions retain padded bytecode and correct jump tables.
- Restored the EOF test vector to `ef00010100040200010001ff00000000800000fe`, matching REVM’s fixture and allowing EOF decoding tests to pass after the normalization changes.

## Validation
- `cargo test --test mainnet mainnet_blocks_from_disk -- --nocapture`
- `cargo test storage::tests::eof_bytecodes -- --nocapture`
