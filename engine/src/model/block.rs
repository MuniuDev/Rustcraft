
pub enum BlockType {
    Air,
    Grass
}

type BlockData = u16;

struct BitRange {
    begin: u8,
    size: u8
}

const TYPE_BITS: BitRange = BitRange{begin:0, size:10};
const VISIBLE_BITS: BitRange = BitRange{begin:10, size:6};

const fn validate_bit_range<T>(ranges: &[BitRange], ) -> bool {
    // let bit_mask: T = 0 as T;
    
    // let total_bits = std::mem::size_of::<BlockData>() * 8;
    // let mut used_bits = 0;
    // for range in ranges:
    // used_bits
    return true;
}

//const ALL_BITS: usize = TYPE_BITS + VISIBLE_BITS;
const_assert!( validate_bit_range::<BlockData>(&[TYPE_BITS, VISIBLE_BITS]) );


#[derive(Debug, Copy, Clone)]
pub struct Block {
    data: BlockData
}

impl Block {
    pub fn new() -> Self { return Block{ data:0 }; }

    pub fn block_type(&self) -> BlockType {
        // TODO
        return BlockType::Air;
    }
}