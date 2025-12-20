use revm::{
    context_interface::ContextTr,
    database::InMemoryDB,
    primitives::{Address, Bytes, TxKind, U256},
    Context, ExecuteEvm, MainBuilder, MainContext,
};

/// Minimal EVM execution wrapper around REVM
pub struct Evm {
    /// In-memory database for state
    db: InMemoryDB,
}

impl Evm {
    /// Creates a new EVM instance with empty state
    pub fn new() -> Self {
        Self {
            db: InMemoryDB::default(),
        }
    }

    /// Executes a raw transaction
    /// Returns the output bytes if successful, or an error message
    pub fn execute_raw_tx(
        &mut self,
        caller: Address,
        to: Option<Address>,
        data: Vec<u8>,
        value: U256,
    ) -> Result<Vec<u8>, String> {
        let to_kind = match to {
            Some(addr) => TxKind::Call(addr),
            None => TxKind::Create,
        };
        let data_bytes = Bytes::from(data);

        // Build the EVM context with our database
        let ctx = Context::mainnet().with_db(self.db.clone());
        let mut evm = ctx.build_mainnet();

        // Create and execute transaction
        let tx = revm::context::TxEnv::builder()
            .caller(caller)
            .kind(to_kind)
            .data(data_bytes)
            .value(value)
            .gas_limit(30_000_000) // Set reasonable gas limit
            .build()
            .unwrap();

        let result = evm
            .transact(tx)
            .map_err(|e| format!("EVM error: {:?}", e))?;

        // Update our database with the resulting state
        self.db = evm.ctx.db_ref().clone();

        // Return the output bytes
        Ok(result.result.output().unwrap_or_default().to_vec())
    }

    /// Sets the balance of an account
    pub fn set_balance(&mut self, address: Address, balance: U256) {
        let account = self.db.cache.accounts.entry(address).or_default();
        account.info.balance = balance;
    }

    /// Sets the code for an account
    pub fn set_code(&mut self, address: Address, code: Vec<u8>) {
        let bytecode = revm::bytecode::Bytecode::new_raw(Bytes::from(code));
        let code_hash = bytecode.hash_slow();

        // Store the contract in the contracts map
        self.db.cache.contracts.insert(code_hash, bytecode.clone());

        // Update the account info
        let account = self.db.cache.accounts.entry(address).or_default();
        account.info.code = Some(bytecode);
        account.info.code_hash = code_hash;
    }
}

impl Default for Evm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_creation() {
        let evm = Evm::new();
        assert_eq!(evm.db.cache.accounts.len(), 0);
    }

    #[test]
    fn test_simple_execution() {
        let mut evm = Evm::new();

        // Create a simpler contract that just returns empty (STOP opcode)
        // This is the simplest possible contract to test execution
        let code = vec![0x00]; // STOP

        let contract_addr = Address::with_last_byte(1);
        evm.set_code(contract_addr, code);

        // Call the contract
        let caller = Address::with_last_byte(2);
        evm.set_balance(caller, U256::from(1_000_000)); // More balance for gas

        let result = evm.execute_raw_tx(caller, Some(contract_addr), vec![], U256::ZERO);

        // Verify execution succeeds (STOP returns empty output, which is valid)
        assert!(result.is_ok(), "Execution should succeed: {:?}", result);
    }

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
