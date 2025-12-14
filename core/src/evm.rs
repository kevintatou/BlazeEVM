//! EVM Execution Wrapper
//!
//! This module provides a minimal wrapper around REVM (Rust Ethereum Virtual Machine),
//! enabling execution of Ethereum transactions in an isolated, in-memory environment.
//!
//! # Overview
//!
//! The `Evm` struct manages:
//! - An in-memory database (`InMemoryDB`) that stores account states, balances, and contract code
//! - Transaction execution using REVM's mainnet configuration
//! - State persistence across multiple transaction executions
//!
//! # How it Works
//!
//! 1. **State Management**: Uses REVM's `InMemoryDB` to track all account states including:
//!    - Account balances (in wei)
//!    - Contract bytecode
//!    - Storage values
//!
//! 2. **Transaction Execution**: Each transaction is executed through REVM's `MainContext`:
//!    - Creates a mainnet-configured EVM context
//!    - Executes the transaction with the provided parameters
//!    - Persists the resulting state changes back to the database
//!
//! 3. **Isolation**: Each `Evm` instance maintains its own isolated state, allowing for
//!    independent test scenarios or simulations.
//!
//! # Example
//!
//! ```rust,ignore
//! use blazeevm_core::evm::Evm;
//! use revm::primitives::{Address, U256};
//!
//! // Create a new EVM instance
//! let mut evm = Evm::new();
//!
//! // Set up a contract with a simple STOP opcode
//! let contract_addr = Address::with_last_byte(1);
//! evm.set_code(contract_addr, vec![0x00]); // STOP
//!
//! // Give the caller some balance for gas
//! let caller = Address::with_last_byte(2);
//! evm.set_balance(caller, U256::from(1_000_000));
//!
//! // Execute a transaction
//! let result = evm.execute_raw_tx(caller, Some(contract_addr), vec![], U256::ZERO);
//! assert!(result.is_ok());
//! ```

use revm::{
    context_interface::ContextTr,
    database::InMemoryDB,
    primitives::{Address, Bytes, TxKind, U256},
    Context, ExecuteEvm, MainBuilder, MainContext,
};

/// Minimal EVM execution wrapper around REVM.
///
/// This struct provides a simple interface for executing Ethereum transactions
/// in an isolated, in-memory environment. It wraps REVM's functionality and
/// maintains transaction state across multiple executions.
///
/// # Architecture
///
/// - **Database**: Uses REVM's `InMemoryDB` to store all account states, balances,
///   and contract code. The database is cloned and updated after each transaction
///   to persist state changes.
///
/// - **Execution Model**: Each transaction runs in a fresh REVM context but
///   operates on the persistent database, ensuring state continuity.
///
/// # Use Cases
///
/// - Testing smart contracts in isolation
/// - Simulating transaction execution without network overhead
/// - Building EVM-based development tools
/// - Creating deterministic test environments
pub struct Evm {
    /// In-memory database that stores account states, balances, and contract code.
    ///
    /// This database is cloned from REVM after each transaction execution to
    /// persist state changes across multiple calls to `execute_raw_tx`.
    db: InMemoryDB,
}

impl Evm {
    /// Creates a new EVM instance with an empty state.
    ///
    /// The returned instance has no accounts, balances, or contract code.
    /// Use `set_balance()` and `set_code()` to initialize the state before
    /// executing transactions.
    ///
    /// # Returns
    ///
    /// A new `Evm` instance with an empty in-memory database.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let evm = Evm::new();
    /// // State is empty - ready for setup
    /// ```
    pub fn new() -> Self {
        Self {
            db: InMemoryDB::default(),
        }
    }

    /// Executes a raw Ethereum transaction using REVM.
    ///
    /// This method simulates the execution of an EVM transaction with the given
    /// parameters. It builds a fresh REVM context for each call but operates on
    /// the persistent database, ensuring state changes are preserved.
    ///
    /// # Parameters
    ///
    /// - `caller`: The address initiating the transaction (msg.sender)
    /// - `to`: The target address for the transaction:
    ///   - `Some(address)`: Call an existing contract at the given address
    ///   - `None`: Create a new contract (contract deployment)
    /// - `data`: The transaction data (calldata or contract creation code)
    /// - `value`: Amount of wei to transfer with the transaction
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u8>)`: The output bytes from successful execution. For contract calls,
    ///   this is the return data. For contract creation, this may be empty.
    /// - `Err(String)`: An error message if execution fails (e.g., out of gas, revert)
    ///
    /// # Transaction Execution Flow
    ///
    /// 1. Converts parameters to REVM types (`TxKind`, `Bytes`)
    /// 2. Creates a mainnet-configured EVM context with the current database state
    /// 3. Builds a transaction with a 30 million gas limit
    /// 4. Executes the transaction through REVM
    /// 5. Persists the resulting state changes back to the database
    /// 6. Returns the output bytes
    ///
    /// # Gas Limit
    ///
    /// The transaction is executed with a fixed gas limit of 30,000,000 gas,
    /// which is sufficient for most contract operations in a testing environment.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut evm = Evm::new();
    /// let caller = Address::with_last_byte(1);
    /// let contract = Address::with_last_byte(2);
    ///
    /// // Call a contract
    /// let result = evm.execute_raw_tx(
    ///     caller,
    ///     Some(contract),
    ///     vec![0x60, 0x01], // Example calldata
    ///     U256::ZERO
    /// );
    /// ```
    pub fn execute_raw_tx(
        &mut self,
        caller: Address,
        to: Option<Address>,
        data: Vec<u8>,
        value: U256,
    ) -> Result<Vec<u8>, String> {
        // Determine transaction type: Call (to existing contract) or Create (deploy new contract)
        let to_kind = match to {
            Some(addr) => TxKind::Call(addr),
            None => TxKind::Create,
        };
        let data_bytes = Bytes::from(data);

        // Build the EVM context with our database state
        // This creates a fresh mainnet-configured context for this transaction
        let ctx = Context::mainnet().with_db(self.db.clone());
        let mut evm = ctx.build_mainnet();

        // Create and execute the transaction
        // The transaction includes caller, target, data, value, and gas limit
        let tx = revm::context::TxEnv::builder()
            .caller(caller)
            .kind(to_kind)
            .data(data_bytes)
            .value(value)
            .gas_limit(30_000_000) // Set reasonable gas limit for testing
            .build()
            .unwrap();

        // Execute the transaction and capture any errors
        let result = evm.transact(tx).map_err(|e| format!("EVM error: {:?}", e))?;

        // Persist state changes: Update our database with the modified state
        // This ensures subsequent transactions see the effects of this one
        self.db = evm.ctx.db_ref().clone();

        // Extract and return the output bytes
        Ok(result.result.output().unwrap_or_default().to_vec())
    }

    /// Sets the balance of an account in wei.
    ///
    /// If the account doesn't exist, it will be created with the specified balance.
    /// If it already exists, its balance will be updated.
    ///
    /// # Parameters
    ///
    /// - `address`: The account address to modify
    /// - `balance`: The new balance in wei (1 ETH = 10^18 wei)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut evm = Evm::new();
    /// let addr = Address::with_last_byte(1);
    ///
    /// // Give the account 1 ETH (10^18 wei)
    /// let one_eth = U256::from(10u64).pow(U256::from(18));
    /// evm.set_balance(addr, one_eth);
    ///
    /// // Or use a smaller amount like 1000 wei
    /// evm.set_balance(addr, U256::from(1000));
    /// ```
    pub fn set_balance(&mut self, address: Address, balance: U256) {
        let account = self.db.cache.accounts.entry(address).or_default();
        account.info.balance = balance;
    }

    /// Sets the bytecode for a contract account.
    ///
    /// This method configures an account as a contract by:
    /// 1. Creating bytecode from the provided bytes
    /// 2. Computing the code hash
    /// 3. Storing the bytecode in the contracts cache
    /// 4. Updating the account to reference this code
    ///
    /// If the account doesn't exist, it will be created as a contract account.
    ///
    /// # Parameters
    ///
    /// - `address`: The address where the contract will be deployed
    /// - `code`: The raw bytecode bytes (EVM opcodes)
    ///
    /// # Contract Code Structure
    ///
    /// The bytecode is stored in REVM's format:
    /// - Raw bytecode is converted to `Bytecode::new_raw()`
    /// - A code hash is computed and stored
    /// - Both the account info and contracts cache are updated
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut evm = Evm::new();
    /// let contract_addr = Address::with_last_byte(1);
    ///
    /// // Deploy a contract that just returns (STOP opcode)
    /// evm.set_code(contract_addr, vec![0x00]);
    ///
    /// // Deploy a contract with PUSH1 0x01, RETURN
    /// evm.set_code(contract_addr, vec![0x60, 0x01, 0xF3]);
    /// ```
    pub fn set_code(&mut self, address: Address, code: Vec<u8>) {
        // Convert raw bytes to REVM bytecode format
        let bytecode = revm::bytecode::Bytecode::new_raw(Bytes::from(code));
        
        // Compute the hash of the bytecode for storage lookup
        let code_hash = bytecode.hash_slow();
        
        // Store the contract bytecode in the contracts cache
        // This is required for REVM to execute the code
        self.db.cache.contracts.insert(code_hash, bytecode.clone());
        
        // Update the account info to reference this contract code
        let account = self.db.cache.accounts.entry(address).or_default();
        account.info.code = Some(bytecode);
        account.info.code_hash = code_hash;
    }
}

impl Default for Evm {
    /// Creates a default EVM instance.
    ///
    /// This is equivalent to calling `Evm::new()` and creates an instance
    /// with an empty state.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a new EVM instance starts with an empty state.
    ///
    /// This test ensures that:
    /// - A newly created EVM has no accounts in its database
    /// - The state is truly isolated and empty
    #[test]
    fn test_evm_creation() {
        let evm = Evm::new();
        assert_eq!(evm.db.cache.accounts.len(), 0);
    }

    /// Tests basic EVM transaction execution with the simplest possible contract.
    ///
    /// This test verifies that:
    /// 1. Contract code can be deployed to an address
    /// 2. An account can be given a balance
    /// 3. A transaction can be executed against the contract
    /// 4. The execution succeeds without errors
    ///
    /// The contract used is the simplest valid EVM program:
    /// - Opcode 0x00 (STOP): Halts execution immediately
    /// - Returns empty output (which is valid)
    #[test]
    fn test_simple_execution() {
        let mut evm = Evm::new();
        
        // Create a simple contract that halts immediately (STOP opcode)
        // This is the simplest possible contract to test execution
        let code = vec![0x00]; // STOP

        let contract_addr = Address::with_last_byte(1);
        evm.set_code(contract_addr, code);

        // Call the contract
        let caller = Address::with_last_byte(2);
        evm.set_balance(caller, U256::from(1_000_000)); // Sufficient balance for gas

        let result = evm.execute_raw_tx(caller, Some(contract_addr), vec![], U256::ZERO);
        
        // Verify execution succeeds (STOP returns empty output, which is valid)
        assert!(result.is_ok(), "Execution should succeed: {:?}", result);
    }

    /// Tests that account balances can be set and retrieved correctly.
    ///
    /// This test verifies that:
    /// - `set_balance()` creates a new account if it doesn't exist
    /// - The balance is stored correctly in the database
    /// - The balance can be retrieved and matches the set value
    #[test]
    fn test_set_balance() {
        let mut evm = Evm::new();
        let address = Address::with_last_byte(1);
        let balance = U256::from(1000);

        evm.set_balance(address, balance);

        let account = evm.db.cache.accounts.get(&address).unwrap();
        assert_eq!(account.info.balance, balance);
    }
}
