use primitive_types::H256;

/// Represents a block header containing metadata about a block
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// Block number (height in the chain)
    pub number: u64,
    /// Hash of the parent block
    pub parent_hash: H256,
    /// State root hash (placeholder for state trie root)
    pub state_root: H256,
    /// Block timestamp (Unix timestamp in seconds)
    pub timestamp: u64,
}

impl Header {
    /// Creates a new header with the given values
    pub fn new(number: u64, parent_hash: H256, state_root: H256, timestamp: u64) -> Self {
        Self {
            number,
            parent_hash,
            state_root,
            timestamp,
        }
    }
}

impl Default for Header {
    /// Creates a default header (genesis block)
    fn default() -> Self {
        Self {
            number: 0,
            parent_hash: H256::zero(),
            state_root: H256::zero(),
            timestamp: 0,
        }
    }
}

/// Represents a block in the blockchain
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Block header
    pub header: Header,
}

impl Block {
    /// Creates a new block with the given header
    pub fn new(header: Header) -> Self {
        Self { header }
    }

    /// Creates a new block with specified header values
    pub fn with_header(number: u64, parent_hash: H256, state_root: H256, timestamp: u64) -> Self {
        Self {
            header: Header::new(number, parent_hash, state_root, timestamp),
        }
    }
}

impl Default for Block {
    /// Creates a default block (genesis block)
    fn default() -> Self {
        Self {
            header: Header::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_header() {
        let header = Header::default();
        assert_eq!(header.number, 0);
        assert_eq!(header.parent_hash, H256::zero());
        assert_eq!(header.state_root, H256::zero());
        assert_eq!(header.timestamp, 0);
    }

    #[test]
    fn test_new_header() {
        let number = 42;
        let parent_hash = H256::from_low_u64_be(1);
        let state_root = H256::from_low_u64_be(2);
        let timestamp = 1234567890;

        let header = Header::new(number, parent_hash, state_root, timestamp);

        assert_eq!(header.number, number);
        assert_eq!(header.parent_hash, parent_hash);
        assert_eq!(header.state_root, state_root);
        assert_eq!(header.timestamp, timestamp);
    }

    #[test]
    fn test_default_block() {
        let block = Block::default();
        assert_eq!(block.header.number, 0);
        assert_eq!(block.header.parent_hash, H256::zero());
        assert_eq!(block.header.state_root, H256::zero());
        assert_eq!(block.header.timestamp, 0);
    }

    #[test]
    fn test_instantiate_block() {
        let header = Header::new(
            1,
            H256::from_low_u64_be(100),
            H256::from_low_u64_be(200),
            1000,
        );
        let block = Block::new(header.clone());

        assert_eq!(block.header.number, 1);
        assert_eq!(block.header.parent_hash, H256::from_low_u64_be(100));
        assert_eq!(block.header.state_root, H256::from_low_u64_be(200));
        assert_eq!(block.header.timestamp, 1000);
    }

    #[test]
    fn test_block_with_header() {
        let block = Block::with_header(
            5,
            H256::from_low_u64_be(50),
            H256::from_low_u64_be(75),
            5000,
        );

        assert_eq!(block.header.number, 5);
        assert_eq!(block.header.parent_hash, H256::from_low_u64_be(50));
        assert_eq!(block.header.state_root, H256::from_low_u64_be(75));
        assert_eq!(block.header.timestamp, 5000);
    }

    #[test]
    fn test_genesis_block() {
        // Test creating a genesis block (block 0)
        let genesis = Block::default();
        assert_eq!(genesis.header.number, 0);
        assert_eq!(genesis.header.parent_hash, H256::zero());
    }

    #[test]
    fn test_block_equality() {
        let header1 = Header::new(
            1,
            H256::from_low_u64_be(10),
            H256::from_low_u64_be(20),
            1000,
        );
        let header2 = Header::new(
            1,
            H256::from_low_u64_be(10),
            H256::from_low_u64_be(20),
            1000,
        );
        let header3 = Header::new(
            2,
            H256::from_low_u64_be(10),
            H256::from_low_u64_be(20),
            1000,
        );

        let block1 = Block::new(header1);
        let block2 = Block::new(header2);
        let block3 = Block::new(header3);

        assert_eq!(block1, block2);
        assert_ne!(block1, block3);
    }
}
