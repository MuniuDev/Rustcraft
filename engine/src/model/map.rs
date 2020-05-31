use crate::model::config;
use crate::model::chunk;

const MAP_AREA: u64 = config::MAP_SIZE * config::MAP_SIZE;

#[derive(Copy, Clone)]
pub struct Map {
    chunks: [chunk::Chunk; MAP_AREA as usize ]
}

impl Map {
    pub fn new() -> Box<Self> { 
        let chunks = [chunk::Chunk::new(); MAP_AREA as usize];
        return Box::new(Map{
            chunks,
        }); }
}