# BlazeEVM

Local-first EVM devnode and simulator. Fast, deterministic, and designed for clean layering so we can grow into multi-chain/cross-chain testing.

## What we're building
- Minimal Rust workspace with `core` (execution/state) and `node` (RPC/CLI)
- REVM-backed execution wrapper
- JSON-RPC basics: health, `eth_chainId`, `eth_blockNumber`
- Extensible foundation for future cross-chain simulation

## Usage

### Running the Node

Start the BlazeEVM node server:

```bash
cargo run --bin blazeevm-node
```

The server will start on `http://127.0.0.1:8545` by default.

### Available Endpoints

- **Health Check**: `GET /health`
  ```bash
  curl http://127.0.0.1:8545/health
  # Response: {"status":"ok"}
  ```

### Running Tests

Run all tests:

```bash
cargo test --workspace
```

Run only node tests:

```bash
cargo test --package blazeevm-node
```

## Development Roadmap

See [ROADMAP.md](./ROADMAP.md) for the complete development roadmap with 11 phases and 44+ micro-issues covering:
- Essential JSON-RPC methods
- Transaction handling and mining
- Development tools (snapshots, time manipulation)
- State persistence and network forking
- Multi-chain and cross-chain simulation (future)

For a quick overview, see [ROADMAP_SUMMARY.md](./ROADMAP_SUMMARY.md).

For creating GitHub issues from the roadmap, see [docs/creating_issues_from_roadmap.md](./docs/creating_issues_from_roadmap.md).

## How we work
- Small, atomic issues using the `.github/ISSUE_TEMPLATE/feature.md`
- Keep scope tight; no extra deps or refactors without an issue
- Prioritize determinism and simplicity in early milestones

## Documentation

- [ROADMAP_SUMMARY.md](./ROADMAP_SUMMARY.md) - Quick overview of the roadmap
- [ROADMAP.md](./ROADMAP.md) - Complete development roadmap with 44 micro-issues
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System architecture and design principles
- [docs/creating_issues_from_roadmap.md](./docs/creating_issues_from_roadmap.md) - Guide for creating issues
