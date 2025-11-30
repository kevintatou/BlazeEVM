# BlazeEVM Architecture (Initial Draft)

BlazeEVM is a **local EVM devnode and simulator**, designed to be:

- fast
- predictable
- friendly to AI-assisted development
- a foundation for future multi-chain & cross-chain simulation

This document describes the initial structure that Codex and other tools must respect.

---

## Workspace Layout

```text
blazeevm/
  core/        -- Core logic: state, EVM, blocks, chain
  node/        -- CLI, RPC server, configuration, logging
  docs/        -- Architecture, design notes
  .github/     -- PR + Issue templates, workflows
```

---

## Crates

### `core/` (Library crate)

Pure Rust library responsible for:

- EVM execution wrapper (e.g. using REVM or similar engine)
- State:
  - accounts
  - storage
  - balances
  - nonces
- Block and chain model:
  - block headers
  - block numbers
  - simple chain management
- Snapshots and basic fork hooks (later)

**No networking, no CLI, no IO here.**

Suggested modules:

- `core/src/state.rs` — State struct, account map, balances.
- `core/src/account.rs` — Account type, nonce, balance.
- `core/src/storage.rs` — Contract storage abstraction.
- `core/src/block.rs` — Block and header structs.
- `core/src/chain.rs` — Chain struct, genesis, append block.
- `core/src/evm.rs` — EVM execution wrapper.

### `node/` (Binary crate)

Responsible for:

- CLI entrypoint (`blazeevm` command).
- Config parsing (host, port, chain id, fork URL, etc).
- JSON-RPC server:
  - eth_chainId
  - eth_blockNumber
  - eth_getBalance
  - eth_getTransactionCount
  - eth_sendRawTransaction
  - (and more over time)
- Logging and basic telemetry hooks.

Suggested modules:

- `node/src/main.rs` — CLI entrypoint.
- `node/src/config.rs` — CLI args, env config.
- `node/src/server.rs` — Server bootstrap.
- `node/src/rpc/` — Handlers for JSON-RPC methods.

---

## Design Principles

- `core` should be deterministic and easily testable in isolation.
- `node` should contain all network/IO concerns.
- Multi-chain / cross-chain simulation will later be layered on top of this separation.
- APIs should be small and explicit, not “magical.”

---

## AI / Codex Guardrails

Codex (and other AI tools) must adhere to:

- Do NOT add new top-level crates without a dedicated issue and design.
- Do NOT change the directory layout without an issue.
- Do NOT introduce networking or CLI code into `core/`.
- Do NOT change public function signatures in `core` unless:
  - a design change is agreed, and
  - there is a specific issue describing the migration.

When unsure:
- Planner & Architect should propose a design change via an issue.
- Reviewer should flag architectural drift.
- Advisor should decide whether to proceed, split, or defer.

---

## Future Extensions (Not part of MVP but good to keep in mind)

- Multi-chain support (multiple chain IDs in one coordinator).
- Cross-chain messaging simulation.
- Snapshots and forks from real RPC providers.
- Pluggable execution engines or pre/post-hooks for specialised testing.

For now, keep the system **minimal and clean**, so we have a solid base to extend.
