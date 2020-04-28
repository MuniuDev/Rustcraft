use crate::config::base_config;
use crate::model::chunk;

const MAP_AREA: u64 = base_config::MAP_SIZE * base_config::MAP_SIZE;

#[derive(Copy, Clone)]
pub struct Map {
    chunks: [chunk::Chunk; MAP_AREA as usize ]
}

impl Map {
    pub fn new() -> Self { return Map{chunks:[chunk::Chunk::new(); MAP_AREA as usize]}; }
}