
#[derive(Debug, Copy, Clone)]
pub struct Block {
    block_type: u16
}

impl Block {
    pub fn new() -> Self { return Block{block_type:0}; }
}