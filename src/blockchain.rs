use crate::block::Block;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
