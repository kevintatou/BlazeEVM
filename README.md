# BlazeEVM

Local-first EVM devnode and simulator. Fast, deterministic, and designed for clean layering so we can grow into multi-chain/cross-chain testing.

## What we're building
- Minimal Rust workspace with `core` (execution/state) and `node` (RPC/CLI)
- REVM-backed execution wrapper
- JSON-RPC basics: health, `eth_chainId`, `eth_blockNumber`
- Extensible foundation for future cross-chain simulation

## How we work
- Small, atomic issues using the `.github/ISSUE_TEMPLATE/feature.md`
- Keep scope tight; no extra deps or refactors without an issue
- Prioritize determinism and simplicity in early milestones
