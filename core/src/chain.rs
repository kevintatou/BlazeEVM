use crate::block::Block;

/// Represents a blockchain with a sequence of blocks
#[derive(Debug, Clone)]
pub struct Chain {
    /// Vector of blocks in the chain
    blocks: Vec<Block>,
}

impl Chain {
    /// Creates a new chain with a genesis block
    pub fn new() -> Self {
        let genesis = Block::default();
        Self {
            blocks: vec![genesis],
        }
    }

    /// Appends a new block to the chain
    pub fn append_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Returns the current length of the chain
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Checks if the chain is empty
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Header;
    use primitive_types::H256;

    #[test]
    fn test_new_chain_creates_genesis() {
        let chain = Chain::new();
        assert_eq!(chain.len(), 1);
        assert_eq!(chain.blocks[0].header.number, 0);
        assert_eq!(chain.blocks[0].header.parent_hash, H256::zero());
    }

    #[test]
    fn test_chain_length_increases() {
        let mut chain = Chain::new();
        assert_eq!(chain.len(), 1);

        let block1 = Block::with_header(1, H256::from_low_u64_be(1), H256::zero(), 1000);
        chain.append_block(block1);
        assert_eq!(chain.len(), 2);

        let block2 = Block::with_header(2, H256::from_low_u64_be(2), H256::zero(), 2000);
        chain.append_block(block2);
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn test_append_block() {
        let mut chain = Chain::new();
        
        let header = Header::new(1, H256::from_low_u64_be(100), H256::zero(), 5000);
        let block = Block::new(header);
        
        chain.append_block(block.clone());
        
        assert_eq!(chain.len(), 2);
        assert_eq!(chain.blocks[1], block);
    }

    #[test]
    fn test_default_chain() {
        let chain = Chain::default();
        assert_eq!(chain.len(), 1);
        assert_eq!(chain.blocks[0].header.number, 0);
    }
}
