# BlazeEVM Roadmap

This document outlines the development roadmap for BlazeEVM, organized into phases with micro-issues that can be tracked and implemented independently.

---

## Current Status

✅ **Completed:**
- Workspace structure (`core/` and `node/` crates)
- Core data structures (Account, State, Block, Chain)
- Basic EVM execution wrapper with REVM
- JSON-RPC server with Axum
- Health check endpoint (`/health`)
- `eth_chainId` RPC method
- Comprehensive test coverage for existing features

---

## Phase 1: Essential JSON-RPC Methods

These are the foundational RPC methods needed for basic EVM devnode functionality.

### Issue 1.1: Implement `eth_blockNumber`
**Summary:** Add JSON-RPC method to return the current block number.

**Description:** 
Implement the `eth_blockNumber` method which returns the number of the most recent block. This requires integrating the `Chain` from core into the node's RPC handlers.

**Acceptance Criteria:**
- [ ] Add handler for `eth_blockNumber` in `node/src/rpc/eth.rs`
- [ ] Return current block number from chain state
- [ ] Return hex-encoded block number (e.g., "0x1" for block 1)
- [ ] Handle edge case of genesis block (should return "0x0")

**Required Tests:**
- [ ] Unit test for `eth_blockNumber` handler
- [ ] Integration test for `eth_blockNumber` endpoint
- [ ] Test with genesis block
- [ ] Test with multiple blocks in chain

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler logic
- `node/tests/integration_test.rs` - Add integration test

---

### Issue 1.2: Implement `eth_getBalance`
**Summary:** Add JSON-RPC method to query account balance by address.

**Description:**
Implement the `eth_getBalance` method which returns the balance of an account at a given address. This requires connecting the RPC layer to the State management in core.

**Acceptance Criteria:**
- [ ] Add handler for `eth_getBalance` in `node/src/rpc/eth.rs`
- [ ] Parse address parameter from JSON-RPC request
- [ ] Query balance from state
- [ ] Return hex-encoded balance (e.g., "0x3e8" for 1000 wei)
- [ ] Return "0x0" for non-existent accounts

**Required Tests:**
- [ ] Unit test for balance query of existing account
- [ ] Unit test for balance query of non-existent account
- [ ] Integration test with sample addresses and balances
- [ ] Test with zero balance
- [ ] Test with large balance values

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler logic
- `node/tests/integration_test.rs` - Add integration tests

---

### Issue 1.3: Implement `eth_getTransactionCount`
**Summary:** Add JSON-RPC method to get the nonce of an account.

**Description:**
Implement the `eth_getTransactionCount` method which returns the number of transactions sent from an address (the nonce). This is essential for transaction validation.

**Acceptance Criteria:**
- [ ] Add handler for `eth_getTransactionCount` in `node/src/rpc/eth.rs`
- [ ] Parse address parameter from JSON-RPC request
- [ ] Query nonce from state
- [ ] Return hex-encoded nonce
- [ ] Return "0x0" for non-existent accounts

**Required Tests:**
- [ ] Unit test for nonce query of existing account
- [ ] Unit test for nonce query of non-existent account
- [ ] Integration test with incremented nonces
- [ ] Test with zero nonce (new account)

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler logic
- `node/tests/integration_test.rs` - Add integration tests

---

### Issue 1.4: Add Shared State Management to Node
**Summary:** Create shared state container for RPC handlers to access chain and state.

**Description:**
Before implementing transaction submission and more complex methods, we need a shared state structure that RPC handlers can access. This will hold the Chain and State, wrapped in Arc<RwLock<>> for thread-safe concurrent access.

**Acceptance Criteria:**
- [ ] Create `NodeState` struct in `node/src/state.rs`
- [ ] Wrap Chain and State from core
- [ ] Use Arc<RwLock<>> for thread-safe access
- [ ] Pass NodeState to RPC handlers via Axum's State extractor
- [ ] Update existing handlers to use NodeState

**Required Tests:**
- [ ] Unit tests for NodeState creation
- [ ] Unit tests for concurrent access patterns
- [ ] Integration tests showing handlers using shared state

**Files to create:**
- `node/src/state.rs` - New module for node state management

**Files to modify:**
- `node/src/lib.rs` - Export state module
- `node/src/rpc/eth.rs` - Update handlers to use NodeState
- `node/src/server.rs` - Initialize and pass NodeState

---

## Phase 2: Transaction Handling

These issues enable the node to accept and process transactions.

### Issue 2.1: Define Transaction Structure in Core
**Summary:** Add Transaction struct to represent raw Ethereum transactions.

**Description:**
Create a Transaction type in the core library that can represent Ethereum transactions with all necessary fields (from, to, value, data, nonce, gas, etc.).

**Acceptance Criteria:**
- [ ] Create `core/src/transaction.rs` module
- [ ] Define Transaction struct with required fields
- [ ] Implement serialization/deserialization
- [ ] Add validation logic (nonce, gas limits, etc.)
- [ ] Export from core lib.rs

**Required Tests:**
- [ ] Unit tests for transaction creation
- [ ] Unit tests for transaction validation
- [ ] Tests for edge cases (invalid nonce, zero gas, etc.)

**Files to create:**
- `core/src/transaction.rs`

**Files to modify:**
- `core/src/lib.rs` - Export transaction module

---

### Issue 2.2: Implement RLP Encoding/Decoding for Transactions
**Summary:** Add RLP encoding and decoding support for raw transactions.

**Description:**
Implement RLP (Recursive Length Prefix) encoding and decoding for transactions to support `eth_sendRawTransaction`. Use existing RLP libraries like `rlp` crate.

**Acceptance Criteria:**
- [ ] Add RLP dependency to core/Cargo.toml
- [ ] Implement RLP encoding for Transaction
- [ ] Implement RLP decoding for Transaction
- [ ] Handle signature extraction from raw transaction bytes
- [ ] Recover sender address from signature

**Required Tests:**
- [ ] Round-trip encoding/decoding tests
- [ ] Tests with known transaction samples
- [ ] Tests for signature verification
- [ ] Tests for sender recovery

**Files to modify:**
- `core/Cargo.toml` - Add RLP dependency
- `core/src/transaction.rs` - Add RLP implementation

---

### Issue 2.3: Implement `eth_sendRawTransaction`
**Summary:** Add JSON-RPC method to submit signed raw transactions.

**Description:**
Implement the `eth_sendRawTransaction` method which accepts a signed raw transaction, validates it, executes it, and returns the transaction hash.

**Acceptance Criteria:**
- [ ] Add handler for `eth_sendRawTransaction` in `node/src/rpc/eth.rs`
- [ ] Parse raw transaction bytes from hex string
- [ ] Decode transaction using RLP
- [ ] Validate transaction (signature, nonce, balance)
- [ ] Execute transaction using core EVM
- [ ] Update state and chain
- [ ] Return transaction hash

**Required Tests:**
- [ ] Integration test with valid signed transaction
- [ ] Test with invalid signature
- [ ] Test with insufficient balance
- [ ] Test with incorrect nonce
- [ ] Test contract deployment transaction
- [ ] Test contract call transaction

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler logic
- `node/tests/integration_test.rs` - Add integration tests

---

### Issue 2.4: Add Transaction Pool (Mempool)
**Summary:** Create a transaction pool to hold pending transactions.

**Description:**
Implement a simple transaction pool (mempool) that stores pending transactions before they are mined into blocks. This is needed for more realistic devnode behavior.

**Acceptance Criteria:**
- [ ] Create `node/src/txpool.rs` module
- [ ] Define TxPool struct with pending transactions
- [ ] Implement add/remove/get operations
- [ ] Use transaction hash as key
- [ ] Thread-safe access with Arc<RwLock<>>
- [ ] Integrate with NodeState

**Required Tests:**
- [ ] Unit tests for pool operations
- [ ] Tests for duplicate transaction handling
- [ ] Tests for pool size limits (optional)
- [ ] Concurrent access tests

**Files to create:**
- `node/src/txpool.rs`

**Files to modify:**
- `node/src/lib.rs` - Export txpool module
- `node/src/state.rs` - Add TxPool to NodeState

---

## Phase 3: Block Production and Mining

These issues enable the node to produce blocks from transactions.

### Issue 3.1: Implement Manual Block Mining
**Summary:** Add ability to manually trigger block creation from pending transactions.

**Description:**
Create functionality to mine a new block containing transactions from the mempool. This will be triggered manually initially (not automatic/interval-based).

**Acceptance Criteria:**
- [ ] Add `mine_block` method to Chain in core
- [ ] Take transactions from txpool
- [ ] Execute transactions and update state
- [ ] Create new block with proper header fields
- [ ] Update chain with new block
- [ ] Clear mined transactions from pool

**Required Tests:**
- [ ] Unit tests for block mining with transactions
- [ ] Tests for empty block mining
- [ ] Tests for state updates after mining
- [ ] Tests for proper block number increment

**Files to modify:**
- `core/src/chain.rs` - Add mining logic
- Integration with core/src/evm.rs for execution

---

### Issue 3.2: Add Mining RPC Method (`evm_mine`)
**Summary:** Add custom RPC method to manually trigger mining.

**Description:**
Implement a custom RPC method `evm_mine` (common in dev nodes like Hardhat/Ganache) that triggers manual mining of pending transactions into a new block.

**Acceptance Criteria:**
- [ ] Add handler for `evm_mine` in `node/src/rpc/eth.rs`
- [ ] Trigger block mining on NodeState
- [ ] Return new block hash or number
- [ ] Handle case with no pending transactions

**Required Tests:**
- [ ] Integration test for mining with transactions
- [ ] Test mining empty block
- [ ] Test that transactions are cleared from pool after mining
- [ ] Test block number increases

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler
- `node/tests/integration_test.rs` - Add tests

---

### Issue 3.3: Implement Auto-Mining Mode
**Summary:** Add configuration option for automatic mining on transaction submission.

**Description:**
Add an auto-mine mode where each transaction automatically triggers mining of a new block. This is useful for testing and provides instant confirmation.

**Acceptance Criteria:**
- [ ] Add auto_mine flag to NodeConfig
- [ ] Check flag in `eth_sendRawTransaction`
- [ ] Automatically mine block after transaction if enabled
- [ ] Default to auto-mine: true for dev convenience

**Required Tests:**
- [ ] Integration test with auto-mine enabled
- [ ] Integration test with auto-mine disabled
- [ ] Test that block numbers increment correctly

**Files to modify:**
- `node/src/config.rs` (create if needed) - Add configuration
- `node/src/rpc/eth.rs` - Check auto-mine flag

---

## Phase 4: Query Methods for Transactions and Blocks

These methods allow querying historical data.

### Issue 4.1: Implement `eth_getBlockByNumber`
**Summary:** Add method to retrieve block details by block number.

**Description:**
Implement `eth_getBlockByNumber` which returns full block information for a given block number.

**Acceptance Criteria:**
- [ ] Add handler for `eth_getBlockByNumber`
- [ ] Parse block number parameter (supports "latest", "earliest", "pending", or hex)
- [ ] Query block from chain
- [ ] Return block details in JSON-RPC format
- [ ] Handle non-existent block numbers

**Required Tests:**
- [ ] Test with genesis block
- [ ] Test with "latest" tag
- [ ] Test with specific block number
- [ ] Test with non-existent block

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler
- `node/tests/integration_test.rs` - Add tests

---

### Issue 4.2: Implement `eth_getBlockByHash`
**Summary:** Add method to retrieve block details by block hash.

**Description:**
Implement `eth_getBlockByHash` which returns full block information for a given block hash. This requires adding block hash computation to the Block struct.

**Acceptance Criteria:**
- [ ] Add block hash computation to Block in core
- [ ] Add hash index to Chain for O(1) lookup
- [ ] Add handler for `eth_getBlockByHash`
- [ ] Return block details in JSON-RPC format
- [ ] Handle non-existent hashes

**Required Tests:**
- [ ] Test with valid block hash
- [ ] Test with invalid block hash
- [ ] Test hash computation consistency

**Files to modify:**
- `core/src/block.rs` - Add hash computation
- `core/src/chain.rs` - Add hash index
- `node/src/rpc/eth.rs` - Add handler

---

### Issue 4.3: Add Transaction Storage and Indexing
**Summary:** Store transactions in blocks and add indexing for queries.

**Description:**
Enhance the Block structure to store transactions and add indexing so transactions can be queried by hash.

**Acceptance Criteria:**
- [ ] Add transactions Vec to Block struct
- [ ] Update block creation to include transactions
- [ ] Create transaction index in Chain (tx hash -> block + index)
- [ ] Add helper methods for transaction lookup

**Required Tests:**
- [ ] Test block with transactions
- [ ] Test transaction lookup by hash
- [ ] Test with multiple transactions in a block

**Files to modify:**
- `core/src/block.rs` - Add transactions field
- `core/src/chain.rs` - Add transaction index

---

### Issue 4.4: Implement `eth_getTransactionByHash`
**Summary:** Add method to retrieve transaction details by hash.

**Description:**
Implement `eth_getTransactionByHash` which returns transaction details including block number, block hash, and transaction index.

**Acceptance Criteria:**
- [ ] Add handler for `eth_getTransactionByHash`
- [ ] Use transaction index to find transaction
- [ ] Return transaction with block context
- [ ] Return null for non-existent transactions

**Required Tests:**
- [ ] Test with valid transaction hash
- [ ] Test with non-existent hash
- [ ] Test includes correct block information

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler
- `node/tests/integration_test.rs` - Add tests

---

### Issue 4.5: Implement `eth_getTransactionReceipt`
**Summary:** Add method to get transaction receipt with execution results.

**Description:**
Implement `eth_getTransactionReceipt` which returns the receipt of a mined transaction, including gas used, logs, and status.

**Acceptance Criteria:**
- [ ] Define Receipt struct in core
- [ ] Store receipts alongside transactions in blocks
- [ ] Add handler for `eth_getTransactionReceipt`
- [ ] Return receipt with gas used, logs, status
- [ ] Return null for pending or non-existent transactions

**Required Tests:**
- [ ] Test receipt for successful transaction
- [ ] Test receipt for failed transaction
- [ ] Test with non-existent transaction
- [ ] Test includes correct gas usage

**Files to modify:**
- `core/src/transaction.rs` - Add Receipt struct
- `core/src/block.rs` - Store receipts
- `node/src/rpc/eth.rs` - Add handler

---

## Phase 5: Contract Storage and State Queries

These features enable querying contract storage.

### Issue 5.1: Implement `eth_getCode`
**Summary:** Add method to get contract code at an address.

**Description:**
Implement `eth_getCode` which returns the contract code (bytecode) stored at a given address.

**Acceptance Criteria:**
- [ ] Add code storage to Account in core (if not present)
- [ ] Add handler for `eth_getCode`
- [ ] Return hex-encoded contract code
- [ ] Return "0x" for EOAs (non-contract accounts)

**Required Tests:**
- [ ] Test with deployed contract
- [ ] Test with EOA (returns "0x")
- [ ] Test with non-existent address

**Files to modify:**
- `core/src/account.rs` - Ensure code field exists
- `node/src/rpc/eth.rs` - Add handler

---

### Issue 5.2: Implement `eth_getStorageAt`
**Summary:** Add method to read contract storage at a specific slot.

**Description:**
Implement `eth_getStorageAt` which reads a storage slot value from a contract at a given address.

**Acceptance Criteria:**
- [ ] Add handler for `eth_getStorageAt`
- [ ] Parse address and storage key parameters
- [ ] Query storage from state
- [ ] Return hex-encoded storage value
- [ ] Return "0x0" for empty slots

**Required Tests:**
- [ ] Test reading existing storage slot
- [ ] Test reading empty storage slot
- [ ] Test with non-existent contract
- [ ] Test with various slot keys

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler
- `node/tests/integration_test.rs` - Add tests

---

### Issue 5.3: Implement `eth_call`
**Summary:** Add method to execute a read-only call without creating a transaction.

**Description:**
Implement `eth_call` which executes a message call immediately without creating a transaction on the blockchain. This is used for reading contract state.

**Acceptance Criteria:**
- [ ] Add handler for `eth_call`
- [ ] Parse transaction call object parameters
- [ ] Execute call on temporary state (don't persist)
- [ ] Return call result data
- [ ] Support block number parameter

**Required Tests:**
- [ ] Test call to contract function
- [ ] Test call with parameters
- [ ] Test call that would revert
- [ ] Test with different block numbers

**Files to modify:**
- `core/src/evm.rs` - Add read-only execution method
- `node/src/rpc/eth.rs` - Add handler

---

### Issue 5.4: Implement `eth_estimateGas`
**Summary:** Add method to estimate gas required for a transaction.

**Description:**
Implement `eth_estimateGas` which estimates the gas needed to execute a transaction without actually submitting it.

**Acceptance Criteria:**
- [ ] Add handler for `eth_estimateGas`
- [ ] Execute transaction on temporary state
- [ ] Return estimated gas used
- [ ] Handle execution failures gracefully

**Required Tests:**
- [ ] Test gas estimation for simple transfer
- [ ] Test gas estimation for contract call
- [ ] Test gas estimation for contract deployment
- [ ] Test with transactions that would fail

**Files to modify:**
- `core/src/evm.rs` - Add gas estimation support
- `node/src/rpc/eth.rs` - Add handler

---

## Phase 6: Development Tools and Testing Features

These are convenience features for development and testing.

### Issue 6.1: Implement `eth_accounts`
**Summary:** Add method to list available accounts.

**Description:**
Implement `eth_accounts` which returns a list of addresses owned by the node. For a local devnode, this typically returns pre-funded test accounts.

**Acceptance Criteria:**
- [ ] Define list of pre-funded accounts in NodeConfig
- [ ] Initialize accounts with balances on startup
- [ ] Add handler for `eth_accounts`
- [ ] Return array of account addresses

**Required Tests:**
- [ ] Test returns configured accounts
- [ ] Test accounts have correct initial balances
- [ ] Test empty accounts list scenario

**Files to modify:**
- `node/src/config.rs` - Add test accounts configuration
- `node/src/rpc/eth.rs` - Add handler
- `node/src/main.rs` - Initialize test accounts

---

### Issue 6.2: Implement `evm_snapshot` and `evm_revert`
**Summary:** Add methods to save and restore chain state.

**Description:**
Implement snapshot and revert functionality (common in dev nodes like Hardhat) that allows saving the current state and reverting to it later. This is useful for testing.

**Acceptance Criteria:**
- [ ] Add snapshot storage to NodeState
- [ ] Implement `evm_snapshot` handler (returns snapshot ID)
- [ ] Implement `evm_revert` handler (reverts to snapshot)
- [ ] Support multiple snapshots (stack-based)
- [ ] Handle invalid snapshot IDs

**Required Tests:**
- [ ] Test snapshot and revert restores state
- [ ] Test multiple snapshots
- [ ] Test revert with invalid snapshot ID
- [ ] Test transactions after revert

**Files to modify:**
- `node/src/state.rs` - Add snapshot storage
- `node/src/rpc/eth.rs` - Add handlers

---

### Issue 6.3: Implement `evm_increaseTime`
**Summary:** Add method to increase the block timestamp.

**Description:**
Implement `evm_increaseTime` which increases the timestamp of the next block by a specified amount. This is useful for testing time-dependent contracts.

**Acceptance Criteria:**
- [ ] Add time offset to NodeState
- [ ] Implement `evm_increaseTime` handler
- [ ] Apply offset when mining next block
- [ ] Return new timestamp

**Required Tests:**
- [ ] Test timestamp increases in next block
- [ ] Test multiple time increases accumulate
- [ ] Test with negative time (should error)

**Files to modify:**
- `node/src/state.rs` - Add time offset
- `core/src/chain.rs` - Use offset in block creation
- `node/src/rpc/eth.rs` - Add handler

---

### Issue 6.4: Implement `evm_setNextBlockTimestamp`
**Summary:** Add method to set the exact timestamp of the next block.

**Description:**
Implement `evm_setNextBlockTimestamp` which sets the exact timestamp for the next block. This gives more precise control than `evm_increaseTime`.

**Acceptance Criteria:**
- [ ] Add next block timestamp override to NodeState
- [ ] Implement `evm_setNextBlockTimestamp` handler
- [ ] Use override when mining next block (if set)
- [ ] Clear override after use

**Required Tests:**
- [ ] Test next block has specified timestamp
- [ ] Test override clears after mining
- [ ] Test with timestamp in the past (should error or allow)

**Files to modify:**
- `node/src/state.rs` - Add timestamp override
- `core/src/chain.rs` - Use override in block creation
- `node/src/rpc/eth.rs` - Add handler

---

### Issue 6.5: Add `hardhat_reset` Method
**Summary:** Add method to reset the node to a clean state.

**Description:**
Implement `hardhat_reset` (compatible with Hardhat network) which resets the entire node state to genesis. This is useful for test cleanup.

**Acceptance Criteria:**
- [ ] Implement `hardhat_reset` handler
- [ ] Reset chain to genesis block
- [ ] Clear all state
- [ ] Clear transaction pool
- [ ] Clear snapshots
- [ ] Re-initialize test accounts if configured

**Required Tests:**
- [ ] Test reset clears transactions
- [ ] Test reset clears blocks
- [ ] Test reset restores initial balances
- [ ] Test reset clears snapshots

**Files to modify:**
- `node/src/rpc/eth.rs` - Add handler
- `node/src/state.rs` - Add reset method

---

## Phase 7: Configuration and CLI

These issues improve node configuration and command-line interface.

### Issue 7.1: Add Configuration File Support
**Summary:** Support loading configuration from a file (JSON or TOML).

**Description:**
Add ability to configure the node via a configuration file instead of just command-line arguments.

**Acceptance Criteria:**
- [ ] Create `node/src/config.rs` module (if not exists)
- [ ] Define NodeConfig struct with all options
- [ ] Support JSON and/or TOML config files
- [ ] Add --config CLI flag to specify config file
- [ ] Provide default config if not specified

**Required Tests:**
- [ ] Test loading config from file
- [ ] Test with invalid config file
- [ ] Test with missing config file
- [ ] Test config validation

**Files to create/modify:**
- `node/src/config.rs` - Configuration loading
- `node/src/main.rs` - Parse config file argument

---

### Issue 7.2: Add CLI Arguments for Common Options
**Summary:** Support command-line arguments for common configuration options.

**Description:**
Add command-line argument parsing for common options like port, host, chain ID, auto-mining, etc. using a crate like `clap`.

**Acceptance Criteria:**
- [ ] Add clap dependency
- [ ] Define CLI arguments for: port, host, chain-id, auto-mine
- [ ] Parse arguments in main.rs
- [ ] Override config file values with CLI arguments
- [ ] Add --help documentation

**Required Tests:**
- [ ] Test with various CLI arguments
- [ ] Test CLI overrides config file
- [ ] Test default values

**Files to modify:**
- `node/Cargo.toml` - Add clap dependency
- `node/src/main.rs` - Add CLI parsing
- `node/src/config.rs` - Merge CLI and file config

---

### Issue 7.3: Add Logging with Configurable Levels
**Summary:** Implement structured logging with configurable log levels.

**Description:**
Add proper logging using `tracing` or `env_logger` crate with configurable log levels (debug, info, warn, error).

**Acceptance Criteria:**
- [ ] Add logging dependency (tracing or env_logger)
- [ ] Add log statements throughout code
- [ ] Support log level configuration
- [ ] Log incoming RPC requests
- [ ] Log transaction execution
- [ ] Log block mining

**Required Tests:**
- [ ] Test logs are generated
- [ ] Test log level filtering
- [ ] Test log format

**Files to modify:**
- `node/Cargo.toml` - Add logging dependency
- `node/src/main.rs` - Initialize logging
- Multiple files - Add log statements

---

### Issue 7.4: Add Version Command
**Summary:** Add --version flag to display node version.

**Description:**
Implement --version CLI flag that displays the BlazeEVM version and build information.

**Acceptance Criteria:**
- [ ] Add --version flag to CLI
- [ ] Display version from Cargo.toml
- [ ] Include build date/commit hash if available
- [ ] Exit after displaying version

**Required Tests:**
- [ ] Test --version output format
- [ ] Test version matches Cargo.toml

**Files to modify:**
- `node/src/main.rs` - Add version flag

---

## Phase 8: State Persistence and Forking

These features add state persistence and forking from other networks.

### Issue 8.1: Implement State Serialization
**Summary:** Add ability to serialize state to disk.

**Description:**
Implement serialization of the entire state (accounts, storage, chain) so it can be saved to disk and restored later.

**Acceptance Criteria:**
- [ ] Implement Serialize/Deserialize for State
- [ ] Implement Serialize/Deserialize for Chain
- [ ] Add save_state method to NodeState
- [ ] Choose format (JSON, bincode, etc.)
- [ ] Handle large states efficiently

**Required Tests:**
- [ ] Test round-trip serialization
- [ ] Test with empty state
- [ ] Test with complex state (many accounts, storage)

**Files to modify:**
- `core/src/state.rs` - Add serialization
- `core/src/chain.rs` - Add serialization
- `node/src/state.rs` - Add save method

---

### Issue 8.2: Implement State Persistence to Disk
**Summary:** Automatically save and restore state on node restart.

**Description:**
Add functionality to automatically persist state to disk periodically and on shutdown, and restore it on startup.

**Acceptance Criteria:**
- [ ] Add data directory configuration
- [ ] Save state on node shutdown
- [ ] Restore state on node startup
- [ ] Add periodic saving (optional)
- [ ] Handle corruption/missing files gracefully

**Required Tests:**
- [ ] Test state persists across restarts
- [ ] Test with missing data directory
- [ ] Test with corrupted state file

**Files to modify:**
- `node/src/main.rs` - Load/save state on startup/shutdown
- `node/src/state.rs` - Add persistence logic
- `node/src/config.rs` - Add data directory config

---

### Issue 8.3: Implement Network Forking (Part 1: RPC Client)
**Summary:** Add ability to connect to external Ethereum RPC provider.

**Description:**
Implement RPC client that can connect to external Ethereum nodes (e.g., Infura, Alchemy) to fetch state and blocks for forking.

**Acceptance Criteria:**
- [ ] Add HTTP client dependency (e.g., reqwest)
- [ ] Create `core/src/fork_client.rs` module
- [ ] Implement methods to fetch blocks, state, code, storage
- [ ] Handle network errors gracefully
- [ ] Add timeout configuration

**Required Tests:**
- [ ] Mock tests for RPC client methods
- [ ] Test error handling
- [ ] Integration test with public testnet (if practical)

**Files to create:**
- `core/src/fork_client.rs`

**Files to modify:**
- `core/Cargo.toml` - Add HTTP client dependency
- `core/src/lib.rs` - Export fork_client module

---

### Issue 8.4: Implement Network Forking (Part 2: State Import)
**Summary:** Import state from external network at a specific block.

**Description:**
Implement functionality to fork from an external network by importing state at a specific block height. Subsequent queries should fall back to the fork network for data not in local state.

**Acceptance Criteria:**
- [ ] Add fork configuration (URL, block number)
- [ ] Import block data from fork network
- [ ] Import account state on-demand
- [ ] Implement fallback database wrapper
- [ ] Cache imported data locally

**Required Tests:**
- [ ] Test forking at specific block
- [ ] Test fallback queries work
- [ ] Test local changes take precedence
- [ ] Test with non-existent fork block

**Files to modify:**
- `core/src/state.rs` - Add fork fallback logic
- `node/src/config.rs` - Add fork configuration
- `node/src/main.rs` - Initialize fork if configured

---

## Phase 9: Performance and Optimization

These issues focus on improving performance and scalability.

### Issue 9.1: Add State Caching Layer
**Summary:** Implement caching for frequently accessed state.

**Description:**
Add an LRU cache layer for account and storage queries to reduce redundant lookups and improve performance.

**Acceptance Criteria:**
- [ ] Add LRU cache dependency (e.g., lru crate)
- [ ] Wrap State with cache layer
- [ ] Cache account queries
- [ ] Cache storage queries
- [ ] Invalidate cache on state changes
- [ ] Make cache size configurable

**Required Tests:**
- [ ] Test cache hit improves performance
- [ ] Test cache invalidation on updates
- [ ] Test cache size limit enforcement
- [ ] Benchmark with/without cache

**Files to modify:**
- `core/src/state.rs` - Add cache layer

---

### Issue 9.2: Optimize Block Storage with Database
**Summary:** Replace in-memory block storage with database backend.

**Description:**
For large chains, storing all blocks in memory is not scalable. Replace with a database backend (e.g., RocksDB, SQLite) for block storage.

**Acceptance Criteria:**
- [ ] Add database dependency
- [ ] Implement database wrapper for blocks
- [ ] Migrate Chain to use database
- [ ] Keep recent blocks in memory cache
- [ ] Support database path configuration

**Required Tests:**
- [ ] Test block retrieval from database
- [ ] Test database persistence across restarts
- [ ] Test with large number of blocks
- [ ] Benchmark performance vs. in-memory

**Files to modify:**
- `core/Cargo.toml` - Add database dependency
- `core/src/chain.rs` - Add database backend

---

### Issue 9.3: Add Parallel Transaction Execution
**Summary:** Execute independent transactions in parallel when mining blocks.

**Description:**
Implement parallel execution of transactions that don't conflict (touch different accounts/storage) to speed up block mining.

**Acceptance Criteria:**
- [ ] Analyze transaction dependencies
- [ ] Group independent transactions
- [ ] Execute groups in parallel
- [ ] Merge results correctly
- [ ] Make parallel execution optional/configurable

**Required Tests:**
- [ ] Test parallel execution produces same results as sequential
- [ ] Test with conflicting transactions
- [ ] Test with independent transactions
- [ ] Benchmark parallel vs. sequential

**Files to modify:**
- `core/src/chain.rs` - Add parallel execution logic
- `core/src/evm.rs` - Support parallel execution

---

## Phase 10: Multi-Chain and Cross-Chain (Future)

These are longer-term features for multi-chain simulation.

### Issue 10.1: Design Multi-Chain Architecture
**Summary:** Design architecture for supporting multiple chains in one node.

**Description:**
Create a design document for how BlazeEVM will support multiple chains (different chain IDs, separate states) running simultaneously in one node.

**Acceptance Criteria:**
- [ ] Create design doc in `docs/multi_chain_design.md`
- [ ] Define how chains are identified
- [ ] Define how RPC routes to specific chains
- [ ] Define shared vs. per-chain resources
- [ ] Get feedback and approval

**Files to create:**
- `docs/multi_chain_design.md`

---

### Issue 10.2: Implement Chain Registry
**Summary:** Add registry to manage multiple chains in a single node.

**Description:**
Implement a ChainRegistry that can store and manage multiple chain instances, each with their own state and configuration.

**Acceptance Criteria:**
- [ ] Create ChainRegistry struct
- [ ] Support adding/removing chains
- [ ] Support querying chain by ID
- [ ] Thread-safe access
- [ ] Integrate with NodeState

**Required Tests:**
- [ ] Test adding multiple chains
- [ ] Test chain isolation
- [ ] Test concurrent access to different chains

**Files to create:**
- `node/src/chain_registry.rs`

---

### Issue 10.3: Add Chain-Specific RPC Routing
**Summary:** Route RPC requests to the appropriate chain based on configuration.

**Description:**
Modify RPC handlers to support multi-chain by routing requests to the appropriate chain instance based on request parameters or URL path.

**Acceptance Criteria:**
- [ ] Support chain ID in request routing
- [ ] Support chain-specific URL paths (e.g., /chain/1, /chain/137)
- [ ] Default to primary chain if not specified
- [ ] Return error for non-existent chains

**Required Tests:**
- [ ] Test routing to different chains
- [ ] Test default chain behavior
- [ ] Test error handling for invalid chains

**Files to modify:**
- `node/src/rpc/eth.rs` - Add chain routing
- `node/src/server.rs` - Setup chain-specific routes

---

### Issue 10.4: Design Cross-Chain Messaging Protocol
**Summary:** Design protocol for simulating cross-chain messages.

**Description:**
Create a design document for how BlazeEVM will simulate cross-chain messages (like IBC, LayerZero, or custom protocols) between chains in the same node.

**Acceptance Criteria:**
- [ ] Create design doc in `docs/cross_chain_messaging.md`
- [ ] Define message format
- [ ] Define delivery guarantees
- [ ] Define validation rules
- [ ] Get feedback and approval

**Files to create:**
- `docs/cross_chain_messaging.md`

---

## Phase 11: Documentation and Examples

These issues improve documentation and provide usage examples.

### Issue 11.1: Write Getting Started Guide
**Summary:** Create comprehensive getting started guide for new users.

**Description:**
Write a detailed getting started guide that walks through installation, basic usage, and common workflows.

**Acceptance Criteria:**
- [ ] Installation instructions for all platforms
- [ ] Basic usage examples
- [ ] Deploying a contract example
- [ ] Interacting with contracts example
- [ ] Common troubleshooting tips

**Files to create:**
- `docs/getting_started.md`

---

### Issue 11.2: Document All RPC Methods
**Summary:** Create API reference documentation for all RPC methods.

**Description:**
Write comprehensive API documentation for all implemented JSON-RPC methods including parameters, return values, and examples.

**Acceptance Criteria:**
- [ ] Document all standard Ethereum RPC methods
- [ ] Document all custom/EVM methods
- [ ] Include parameter descriptions
- [ ] Include example requests and responses
- [ ] Include error codes

**Files to create:**
- `docs/rpc_api.md`

---

### Issue 11.3: Create Example Scripts and Tutorials
**Summary:** Provide example scripts showing common use cases.

**Description:**
Create example scripts (JavaScript/TypeScript with ethers.js or web3.js) demonstrating common BlazeEVM usage patterns.

**Acceptance Criteria:**
- [ ] Example: Deploy and call a contract
- [ ] Example: Use snapshots for testing
- [ ] Example: Time manipulation
- [ ] Example: Fork from mainnet
- [ ] Examples are well-commented

**Files to create:**
- `examples/` directory with multiple example scripts

---

### Issue 11.4: Add Architecture Decision Records (ADRs)
**Summary:** Document key architectural decisions made during development.

**Description:**
Create ADR (Architecture Decision Record) documents for major technical decisions to help future contributors understand the rationale.

**Acceptance Criteria:**
- [ ] Create ADR template in `docs/adr/template.md`
- [ ] Write ADRs for key past decisions
- [ ] Include context, decision, and consequences
- [ ] Update ARCHITECTURE.md to reference ADRs

**Files to create:**
- `docs/adr/` directory with ADR documents

---

## Implementation Guidelines

### For Each Issue:

1. **Read the ARCHITECTURE.md carefully** - Ensure your changes align with the design principles
2. **Keep scope minimal** - Only modify what's necessary for the specific issue
3. **Follow existing patterns** - Look at existing code for style and structure
4. **Write tests first** - TDD approach when practical
5. **Update documentation** - Keep README and docs in sync with changes
6. **No extra dependencies** - Only add dependencies if absolutely necessary and specified in the issue

### Testing Strategy:

- **Unit tests** in each module (e.g., `core/src/state.rs` has `mod tests`)
- **Integration tests** in `node/tests/` for RPC endpoints
- Run `cargo test --workspace` before submitting
- Ensure all tests pass

### Common Pitfalls to Avoid:

- Don't add networking/IO to `core/` - keep it pure
- Don't change function signatures without an issue
- Don't refactor unrelated code
- Don't add features not specified in the issue
- Don't introduce breaking changes to public APIs without discussion

---

## Notes

This roadmap is a living document. As the project evolves, issues may be added, modified, or reprioritized based on feedback and changing requirements. Each issue should be tracked in GitHub Issues using the feature template at `.github/ISSUE_TEMPLATE/feature.md`.
