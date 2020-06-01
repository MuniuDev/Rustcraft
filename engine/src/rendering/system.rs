use vulkano::instance::Instance;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder};
use vulkano::swapchain;
use vulkano::sync;
use vulkano::sync::FlushError;
use vulkano::sync::GpuFuture;

use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoopWindowTarget;
use winit::dpi::PhysicalSize;
use winit::window::WindowId;
use vulkano::swapchain::{AcquireError, SwapchainCreationError};

use std::sync::Arc;
use std::vec::Vec;

use crate::rendering::common::*;
use crate::rendering::vertex::Vertex;
use crate::rendering::context::RenderContext;
use crate::rendering::renderer::Renderer;
use crate::rendering::error::RenderingError;

use crate::rendering::GeometryId;

use crate::core::*;

pub struct RenderingSystem {
    instance : Arc<Instance>,
    context : RenderContext,
    renderer : Renderer,
    
    // TEMPORARY
    vertex_buffer : Vec<Arc<dyn vulkano::buffer::BufferAccess + Send + Sync>>,
}

impl RenderingSystem {
    pub fn new(elwt : &EventLoopWindowTarget<()>) -> Self {
        // Instance
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
        .expect("failed to create instance");

        let context = RenderContext::new(elwt, instance.clone());
        let renderer = Renderer::new(context.device.clone(), context.default_window_render_pass.clone());

        // TEMPORARY BEGIN
        let vertex_buffer = CpuAccessibleBuffer::from_iter(context.device.clone(), BufferUsage::all(), false, [
            Vertex { position: [-0.5, -0.25, 0.0] },
            Vertex { position: [0.0, 0.5, 0.0] },
            Vertex { position: [0.25, -0.1, 0.0] }
        ].iter().cloned()).unwrap();
        // TEMPORARY END

        return RenderingSystem{
            instance,
            context,
            renderer,
            
            vertex_buffer: vec![vertex_buffer],
        };
    }

    pub fn open_window(&mut self, elwt : &EventLoopWindowTarget<()>, window_name: &str) -> WindowId {
        return self.context.create_window(elwt, window_name);
    }

    pub fn window_resized(&mut self, window_id: WindowId, new_size: PhysicalSize<u32>) {
        self.context.windows.get_mut(&window_id).unwrap().on_resize();
    }

    pub fn close_window(&mut self, window_id: WindowId) -> bool {
        self.context.close_window(window_id);
        return self.context.window_count() == 0;
    }

    pub fn create_geometry(&mut self, data: &Vec<na::Vector3<FpScalar>>) -> GeometryId {
        return self.context.create_geometry(data);
    }

    pub fn end_frame(&mut self) {
        for window in self.context.windows.values_mut() {
            let (image_num, acquire_future) = match window.acquire_next_image() {
                Ok(r) => r,
                Err(RenderingError::ImageAcquireFailed) => { return; },
                Err(RenderingError::RecreateSwapchainFailed) => { return; },
                Err(e) => { panic!("Acquire failed! {:?}", e)},
            };

            let g = &self.context.geometries.values().next().unwrap();
            window.draw(image_num, acquire_future, &self.renderer, g);
        }
    }
}