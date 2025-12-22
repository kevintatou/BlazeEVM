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

## How we work
- Small, atomic issues using the `.github/ISSUE_TEMPLATE/feature.md`
- Keep scope tight; no extra deps or refactors without an issue
- Prioritize determinism and simplicity in early milestones
