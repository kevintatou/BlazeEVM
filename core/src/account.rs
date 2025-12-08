use primitive_types::U256;
use std::collections::HashMap;

/// Represents an Ethereum account with balance, nonce, and storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    /// Account balance in wei
    pub balance: U256,
    /// Transaction nonce (number of transactions sent from this account)
    pub nonce: u64,
    /// Contract storage mapping
    pub storage: HashMap<U256, U256>,
}

impl Account {
    /// Creates a new account with the given balance and nonce.
    /// Storage is initialized as empty.
    pub fn new(balance: U256, nonce: u64) -> Self {
        Self {
            balance,
            nonce,
            storage: HashMap::new(),
        }
    }
}

impl Default for Account {
    /// Creates a default account with zero balance, zero nonce, and empty storage.
    fn default() -> Self {
        Self {
            balance: U256::zero(),
            nonce: 0,
            storage: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_account() {
        let account = Account::default();
        assert_eq!(account.balance, U256::zero());
        assert_eq!(account.nonce, 0);
        assert!(account.storage.is_empty());
    }

    #[test]
    fn test_new_account() {
        let balance = U256::from(1000);
        let nonce = 5;
        let account = Account::new(balance, nonce);

        assert_eq!(account.balance, U256::from(1000));
        assert_eq!(account.balance, balance);
        assert_eq!(account.nonce, nonce);
        assert!(account.storage.is_empty());
    }

    #[test]
    fn test_update_balance() {
        // Create account with initial balance of 100
        let mut account = Account::new(U256::from(100), 0);
        assert_eq!(account.balance, U256::from(100));

        // Update balance to 500
        account.balance = U256::from(500);
        assert_eq!(account.balance, U256::from(500));

        // Update balance to 1000
        account.balance = U256::from(1000);
        assert_eq!(account.balance, U256::from(1000));
    }

    #[test]
    fn test_update_nonce() {
        // Create account with initial nonce of 5
        let mut account = Account::new(U256::zero(), 5);
        assert_eq!(account.nonce, 5);

        // Update nonce to 10
        account.nonce = 10;
        assert_eq!(account.nonce, 10);

        // Increment nonce
        account.nonce += 1;
        assert_eq!(account.nonce, 11);
    }

    #[test]
    fn test_storage_operations() {
        let mut account = Account::default();
        assert!(account.storage.is_empty());

        // Insert storage values
        let key1 = U256::from(1);
        let value1 = U256::from(100);
        account.storage.insert(key1, value1);

        assert_eq!(account.storage.len(), 1);
        assert_eq!(account.storage.get(&key1), Some(&value1));

        // Update storage value
        let value2 = U256::from(200);
        account.storage.insert(key1, value2);
        assert_eq!(account.storage.get(&key1), Some(&value2));
    }
}
