use crate::core::*;
use crate::model::config;
use crate::model::block;
use nalgebra::{Vector2, Vector3};

const SUBCHUNK_VOLUME: usize = (config::SUBCHUNK_SIZE * config::SUBCHUNK_SIZE * config::SUBCHUNK_SIZE) as usize;
const SUBCHUNK_COUNT: usize = config::BUILD_LIMIT as usize;

#[derive(Copy, Clone)]
pub struct SubChunk {
    blocks: [block::Block; SUBCHUNK_VOLUME]
}

#[derive(Copy, Clone)]
pub struct Chunk {
    pos: Vector2<PosScalar>,
    sub_chunks: [SubChunk; SUBCHUNK_COUNT]
}

impl SubChunk {
    pub fn empty() -> Self { 
        return SubChunk{
            blocks:[block::Block::new(); SUBCHUNK_VOLUME],
        }; 
    }

    fn pos_to_idx(local_pos: Vector3<PosScalar>) -> usize {
        let signed_map_size = config::MAP_SIZE as PosScalar;
        let signed_layer_area = signed_map_size * signed_map_size;
        let abs_x = signed_map_size + local_pos.x;
        let abs_y = signed_map_size - local_pos.y;
        assert!(abs_x >= 0);
        assert!(abs_y >= 0);
        assert!(local_pos.z >= 0);
        return (local_pos.z * signed_layer_area + abs_y * signed_map_size + abs_x) as usize
    }

    pub fn block(&self, local_pos: Vector3<PosScalar>) -> Option<&block::Block> {
        return Some(&self.blocks[SubChunk::pos_to_idx(local_pos)]);
    }
}

impl Chunk {
    pub fn generate(pos: Vector2<PosScalar>) -> Self {
        let sub_chunks = [SubChunk::empty(); SUBCHUNK_COUNT];
        return Chunk{pos, sub_chunks};
    }

    fn pos_to_idx(pos: Vector3<PosScalar>) -> usize {
        return (pos.z as SizeScalar / config::SUBCHUNK_SIZE) as usize;
    }

    fn local_subchunk_pos(pos: Vector3<PosScalar>) -> Vector3<PosScalar> {
        let signed_size = config::SUBCHUNK_SIZE as i32;
        return Vector3::<PosScalar>::new(pos.x % signed_size, pos.y % signed_size, pos.z % signed_size);
    }

    pub fn subchunk(&self, pos: Vector3<PosScalar>) -> Option<&SubChunk> {
        return Some(&self.sub_chunks[Chunk::pos_to_idx(pos)]);
    }

    pub fn block(&self, pos: Vector3<PosScalar>) -> Option<&block::Block> {
        match self.subchunk(pos) {
            Some(subchunk) => { return subchunk.block(Chunk::local_subchunk_pos(pos)); }
            None => { return None; }
        }
    }
}

