
use vulkano::instance::Instance;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder};
use vulkano::swapchain;
use vulkano::sync;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;
use vulkano::device::Device;


use std::sync::Arc;

use crate::core::*;
use crate::rendering::vertex::Vertex;

pub use crate::core::*;

pub type GeometryId = u32;

pub struct Geometry {
    pub vertex_buffer : Vec<Arc<dyn vulkano::buffer::BufferAccess + Send + Sync>>
}

impl Geometry {
    pub fn from_data(device: Arc<Device>, data: &Vec<na::Vector3<FpScalar>>) -> Self {
        let mapper = |v3: &na::Vector3<FpScalar>|{ return Vertex{position: [v3.x as f32, v3.y as f32, v3.z as f32]}; };
        let map_handle = data.into_iter().map(mapper);
        
        let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, map_handle).unwrap();

        return Geometry{ vertex_buffer: vec![vertex_buffer] }
    }
}