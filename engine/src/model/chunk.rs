use crate::config::base_config;
use crate::model::block;

const CHUNK_SIZE_64: u64 = base_config::CHUNK_SIZE as u64;
const SUBCHUNK_VOLUME: u64 = CHUNK_SIZE_64 * CHUNK_SIZE_64 * CHUNK_SIZE_64;
const CHUNK_VOLUME: u64 = CHUNK_SIZE_64 * CHUNK_SIZE_64 * base_config::BUILD_LIMIT as u64;
const SUBCHUNK_COUNT: u8 = base_config::BUILD_LIMIT / base_config::CHUNK_SIZE;

#[derive(Copy, Clone)]
pub struct SubChunk {
    blocks: [block::Block; CHUNK_VOLUME as usize]
}

#[derive(Copy, Clone)]
pub struct Chunk {
    sub_chunks: [SubChunk; SUBCHUNK_COUNT as usize]
}

impl SubChunk {
    pub fn new() -> Self { return SubChunk{blocks:[block::Block::new(); CHUNK_VOLUME as usize]}; }
}

impl Chunk {
    pub fn new() -> Self { return Chunk{sub_chunks:[SubChunk::new(); SUBCHUNK_COUNT as usize]}; }
}