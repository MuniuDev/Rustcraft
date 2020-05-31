use crate::model::config;
use crate::model::block;

const CHUNK_SIZE_64: u64 = config::CHUNK_SIZE as u64;
const SUBCHUNK_VOLUME: u64 = CHUNK_SIZE_64 * CHUNK_SIZE_64 * CHUNK_SIZE_64;
const CHUNK_VOLUME: u64 = CHUNK_SIZE_64 * CHUNK_SIZE_64 * config::BUILD_LIMIT as u64;
const SUBCHUNK_COUNT: u8 = config::BUILD_LIMIT / config::CHUNK_SIZE;

#[derive(Copy, Clone)]
pub struct SubChunk {
    blocks: [block::Block; SUBCHUNK_VOLUME as usize]
}

#[derive(Copy, Clone)]
pub struct Chunk {
    sub_chunks: [SubChunk; SUBCHUNK_COUNT as usize]
}

impl SubChunk {
    pub fn new() -> Self { 
        return SubChunk{
            blocks:[block::Block::new(); SUBCHUNK_VOLUME as usize],
        }; 
    }
}

impl Chunk {
    pub fn new() -> Self { return Chunk{sub_chunks:[SubChunk::new(); SUBCHUNK_COUNT as usize]}; }
}