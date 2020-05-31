use crate::core::*;

use crate::model::config;
use crate::model::chunk;
use nalgebra::Vector2;
 
const MAP_AREA: usize = config::MAP_SIZE as usize * config::MAP_SIZE as usize;

#[derive(Clone)]
pub struct Map {
    loaded_chunks: Box<[chunk::Chunk]>
}

impl Map {
    pub fn new() -> Box<Self> {
        let mut chunks = Vec::<chunk::Chunk>::new();
        chunks.reserve_exact(MAP_AREA);
        
        let signed_size = config::MAP_SIZE as PosScalar;
        
        for y in -signed_size..signed_size {
            for x in -signed_size..signed_size {
                let pos = Vector2::<PosScalar>::new(x,y);
                chunks.push(chunk::Chunk::generate(pos));
            }
        }

        return Box::new(Map{
            loaded_chunks : chunks.into_boxed_slice(),
        }); }

    fn pos_to_idx(pos: Vector2<PosScalar>) -> usize {
        let signed_map_size = config::MAP_SIZE as PosScalar;
        let abs_x = signed_map_size + pos.x;
        let abs_y = signed_map_size - pos.y;
        assert!(abs_x >= 0);
        assert!(abs_y >= 0);
        return (abs_y * signed_map_size + abs_x) as usize
    }

    pub fn loaded_chunk(&self, pos: Vector2<PosScalar>) -> Option<&chunk::Chunk> {
        return Some(&self.loaded_chunks[Map::pos_to_idx(pos)]);
    }
}