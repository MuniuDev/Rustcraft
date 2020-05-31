use crate::model::config;
use crate::model::chunk;

const MAP_AREA: u64 = config::MAP_SIZE * config::MAP_SIZE;

#[derive(Copy, Clone)]
pub struct Map {
    chunks: [chunk::Chunk; MAP_AREA as usize ]
}

impl Map {
    pub fn new() -> Self { return Map{chunks:[chunk::Chunk::new(); MAP_AREA as usize]}; }
}