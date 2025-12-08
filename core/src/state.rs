use primitive_types::{H160, U256};
use std::collections::HashMap;
use crate::account::Account;

/// Global state managing all accounts in the EVM
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    /// Map of addresses to accounts
    accounts: HashMap<H160, Account>,
}

impl State {
    /// Creates a new empty state
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    /// Gets an account by address, returns None if it doesn't exist
    pub fn get_account(&self, address: &H160) -> Option<&Account> {
        self.accounts.get(address)
    }

    /// Gets a mutable reference to an account by address, returns None if it doesn't exist
    pub fn get_account_mut(&mut self, address: &H160) -> Option<&mut Account> {
        self.accounts.get_mut(address)
    }

    /// Gets an existing account or creates a new default account if it doesn't exist
    pub fn get_or_create_account(&mut self, address: &H160) -> &mut Account {
        self.accounts.entry(*address).or_insert_with(Account::default)
    }

    /// Sets the balance of an account, creating the account if it doesn't exist
    pub fn set_balance(&mut self, address: &H160, balance: U256) {
        let account = self.get_or_create_account(address);
        account.balance = balance;
    }

    /// Increments the nonce of an account, creating the account if it doesn't exist
    pub fn increment_nonce(&mut self, address: &H160) {
        let account = self.get_or_create_account(address);
        account.nonce += 1;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = State::new();
        assert_eq!(state.accounts.len(), 0);
    }

    #[test]
    fn test_account_creation() {
        let mut state = State::new();
        let address = H160::from_low_u64_be(1);

        // Initially, account should not exist
        assert!(state.get_account(&address).is_none());

        // Create account by getting or creating it
        let account = state.get_or_create_account(&address);
        assert_eq!(account.balance, U256::zero());
        assert_eq!(account.nonce, 0);

        // Now account should exist
        assert!(state.get_account(&address).is_some());
    }

    #[test]
    fn test_set_balance() {
        let mut state = State::new();
        let address = H160::from_low_u64_be(1);
        let balance = U256::from(1000);

        // Set balance on non-existent account (should create it)
        state.set_balance(&address, balance);

        let account = state.get_account(&address).unwrap();
        assert_eq!(account.balance, balance);
        assert_eq!(account.nonce, 0);
    }

    #[test]
    fn test_increment_nonce() {
        let mut state = State::new();
        let address = H160::from_low_u64_be(1);

        // Increment nonce on non-existent account (should create it)
        state.increment_nonce(&address);

        let account = state.get_account(&address).unwrap();
        assert_eq!(account.nonce, 1);
        assert_eq!(account.balance, U256::zero());

        // Increment again
        state.increment_nonce(&address);
        let account = state.get_account(&address).unwrap();
        assert_eq!(account.nonce, 2);
    }

    #[test]
    fn test_multiple_accounts() {
        let mut state = State::new();
        let addr1 = H160::from_low_u64_be(1);
        let addr2 = H160::from_low_u64_be(2);

        state.set_balance(&addr1, U256::from(100));
        state.set_balance(&addr2, U256::from(200));
        state.increment_nonce(&addr1);

        let account1 = state.get_account(&addr1).unwrap();
        assert_eq!(account1.balance, U256::from(100));
        assert_eq!(account1.nonce, 1);

        let account2 = state.get_account(&addr2).unwrap();
        assert_eq!(account2.balance, U256::from(200));
        assert_eq!(account2.nonce, 0);
    }
}
