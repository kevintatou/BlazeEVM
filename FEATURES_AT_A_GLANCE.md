# BlazeEVM Features at a Glance

**Overall Goal:** Build a local-first EVM devnode and simulator that's fast, deterministic, and designed for clean layering to support future multi-chain/cross-chain testing.

---

## ✅ Currently Implemented (as of latest commit)

| Feature | One-Liner | File(s) |
|---------|-----------|---------|
| **Workspace Structure** | Minimal Rust workspace with `core` (execution/state) and `node` (RPC/CLI) separation | `Cargo.toml` |
| **Account Model** | Ethereum account with balance, nonce, and storage mapping | `core/src/account.rs` |
| **State Management** | Global state managing all accounts with get/set operations | `core/src/state.rs` |
| **Block Structure** | Block and header types with number, parent hash, state root, timestamp | `core/src/block.rs` |
| **Chain Management** | Blockchain with genesis and ability to append blocks | `core/src/chain.rs` |
| **EVM Execution** | Minimal REVM wrapper for executing transactions and managing state | `core/src/evm.rs` |
| **JSON-RPC Server** | HTTP server with Axum for handling RPC requests | `node/src/server.rs` |
| **Health Check** | Basic health endpoint returning `{"status":"ok"}` | `node/src/rpc/health.rs` |
| **eth_chainId** | Returns the chain ID in hex format (default: 0x539 for 1337) | `node/src/rpc/eth.rs` |

**Summary:** Basic infrastructure is in place with core EVM concepts and minimal RPC functionality.

---

## 🚧 Phase 1: Essential JSON-RPC Methods (4 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **eth_blockNumber** | Return current block number from chain state | 🔴 Critical |
| **eth_getBalance** | Query account balance by address | 🔴 Critical |
| **eth_getTransactionCount** | Get account nonce (transaction count) | 🔴 Critical |
| **Shared State Management** | Thread-safe state container for RPC handlers | 🔴 Critical |

**Goal:** Enable basic queries about blockchain state (blocks, balances, nonces).

---

## 🚧 Phase 2: Transaction Handling (4 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **Transaction Structure** | Define Transaction type with all required fields | 🔴 Critical |
| **RLP Encoding/Decoding** | Serialize/deserialize transactions with signature recovery | 🔴 Critical |
| **eth_sendRawTransaction** | Accept, validate, execute, and return transaction hash | 🔴 Critical |
| **Transaction Pool (Mempool)** | Store pending transactions before mining | 🔴 Critical |

**Goal:** Enable users to submit and track transactions.

---

## 🚧 Phase 3: Block Production and Mining (3 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **Manual Block Mining** | Create blocks from pending transactions with proper execution | 🟠 High |
| **evm_mine RPC** | Trigger manual mining via RPC call | 🟠 High |
| **Auto-Mining Mode** | Automatically mine block after each transaction | 🟠 High |

**Goal:** Enable block production and transaction confirmation.

---

## 🚧 Phase 4: Query Methods for Transactions and Blocks (5 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **eth_getBlockByNumber** | Retrieve block details by number (supports "latest", "earliest") | 🟠 High |
| **eth_getBlockByHash** | Retrieve block details by block hash | 🟠 High |
| **Transaction Storage** | Store transactions in blocks with indexing by hash | 🟠 High |
| **eth_getTransactionByHash** | Retrieve transaction with block context | 🟠 High |
| **eth_getTransactionReceipt** | Get transaction receipt with gas used, logs, status | 🟠 High |

**Goal:** Enable querying historical blockchain data for explorers and dapps.

---

## 🚧 Phase 5: Contract Storage and State Queries (4 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **eth_getCode** | Return contract bytecode at address | 🟠 High |
| **eth_getStorageAt** | Read specific contract storage slot | 🟠 High |
| **eth_call** | Execute read-only message call without persisting state | 🟠 High |
| **eth_estimateGas** | Estimate gas needed for transaction execution | 🟠 High |

**Goal:** Enable smart contract interaction and state inspection.

---

## 🚧 Phase 6: Development Tools and Testing Features (5 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **eth_accounts** | List pre-funded test accounts available on node | 🟡 Medium |
| **evm_snapshot / evm_revert** | Save and restore chain state for testing | 🟡 Medium |
| **evm_increaseTime** | Fast-forward block timestamp by specified seconds | 🟡 Medium |
| **evm_setNextBlockTimestamp** | Set exact timestamp for next block | 🟡 Medium |
| **hardhat_reset** | Reset entire node state to genesis | 🟡 Medium |

**Goal:** Provide convenient testing and debugging tools for developers.

---

## 🚧 Phase 7: Configuration and CLI (4 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **Configuration File** | Load settings from JSON/TOML config file | 🟡 Medium |
| **CLI Arguments** | Support command-line flags (port, host, chain-id, etc.) | 🟡 Medium |
| **Structured Logging** | Configurable log levels with tracing/env_logger | 🟡 Medium |
| **Version Command** | Display node version and build info | 🟡 Medium |

**Goal:** Improve usability and operational flexibility.

---

## 🚧 Phase 8: State Persistence and Forking (4 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **State Serialization** | Serialize/deserialize entire state to disk | 🟡 Medium |
| **Automatic Persistence** | Save state on shutdown, restore on startup | 🟡 Medium |
| **RPC Client for Forking** | Connect to external Ethereum nodes for data | 🟡 Medium |
| **Network Forking** | Fork from mainnet/testnet at specific block with fallback | 🟡 Medium |

**Goal:** Enable persistent state and forking from live networks.

---

## 🚧 Phase 9: Performance and Optimization (3 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **State Caching** | LRU cache for frequently accessed accounts and storage | 🟢 Low |
| **Database Backend** | Replace in-memory blocks with RocksDB/SQLite for scalability | 🟢 Low |
| **Parallel Execution** | Execute independent transactions concurrently when mining | 🟢 Low |

**Goal:** Improve performance and scalability for production use.

---

## 🔵 Phase 10: Multi-Chain and Cross-Chain (Future) (4 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **Multi-Chain Design** | Architecture document for supporting multiple chains | 🔵 Future |
| **Chain Registry** | Manage multiple chain instances in one node | 🔵 Future |
| **Chain-Specific Routing** | Route RPC requests to appropriate chain by ID or path | 🔵 Future |
| **Cross-Chain Messaging** | Design protocol for simulating cross-chain messages | 🔵 Future |

**Goal:** Enable multi-chain and cross-chain simulation for advanced testing.

---

## 🚧 Phase 11: Documentation and Examples (4 issues)

| Feature | One-Liner | Priority |
|---------|-----------|----------|
| **Getting Started Guide** | Installation and basic usage walkthrough | 🟡 Medium |
| **API Reference** | Complete documentation of all RPC methods | 🟡 Medium |
| **Example Scripts** | JavaScript/TypeScript tutorials for common use cases | 🟡 Medium |
| **Architecture Decision Records** | Document key technical decisions and rationale | 🟡 Medium |

**Goal:** Help users understand and effectively use BlazeEVM.

---

## 📊 Quick Stats

- **Completed Features:** 9
- **Planned Features:** 44
- **Total Features:** 53
- **Critical Priority:** 8 features
- **High Priority:** 12 features
- **Medium Priority:** 18 features
- **Low Priority:** 3 features
- **Future Research:** 4 features

---

## 🎯 Recommended Implementation Order

**For MVP (Minimum Viable Devnode):**
```
Phase 1 → Phase 2 → Phase 3 → Phase 5 (basic) → Phase 6 (snapshots)
~12 issues, 2-4 weeks
```

**For Feature-Complete Devnode:**
```
Phases 1-7 complete
~32 issues, 6-8 weeks
```

**For Production-Ready:**
```
All phases except Phase 10
~44 issues, 10-14 weeks
```

---

## 📚 Related Documentation

- **[ROADMAP.md](./ROADMAP.md)** - Complete details on each feature with acceptance criteria, tests, and implementation notes
- **[ROADMAP_SUMMARY.md](./ROADMAP_SUMMARY.md)** - Visual overview with complexity estimates and learning paths
- **[docs/creating_issues_from_roadmap.md](./docs/creating_issues_from_roadmap.md)** - Guide for converting features to GitHub issues
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - System design principles and guardrails
- **[README.md](./README.md)** - Getting started and usage instructions
