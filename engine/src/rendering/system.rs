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

pub struct RenderingSystem {
    instance : Arc<Instance>,
    context : RenderContext,
    renderer : Renderer,
    
    // TEMPORARY
    vertex_buffer : Vec<Arc<dyn vulkano::buffer::BufferAccess + Send + Sync>>,
}

impl RenderingSystem {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        // Instance
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
        .expect("failed to create instance");

        let context = RenderContext::new(event_loop, instance.clone());
        let renderer = Renderer::new(context.device.clone(), context.render_target.render_pass.clone());

        // TEMPORARY BEGIN
        let vertex_buffer = CpuAccessibleBuffer::from_iter(context.device.clone(), BufferUsage::all(), false, [
            Vertex { position: [-0.5, -0.25] },
            Vertex { position: [0.0, 0.5] },
            Vertex { position: [0.25, -0.1] }
        ].iter().cloned()).unwrap();
        // TEMPORARY END

        return RenderingSystem{
            instance,
            context,
            renderer,
            
            vertex_buffer: vec![vertex_buffer],
        };
    }

    pub fn open_window(&mut self, elwt : &EventLoopWindowTarget<()>, window_name: &str) /*-> WindowId*/ {
    }

    pub fn window_resized(&mut self, window_id: WindowId, new_size: PhysicalSize<u32>) {
        self.context.recreate_swapchain = true;
    }

    pub fn close_window(&mut self, window_id: WindowId) {
        
    }

    pub fn end_frame(&mut self) {
        // It is important to call this function from time to time, otherwise resources will keep
        // accumulating and you will eventually reach an out of memory error.
        // Calling this function polls various fences in order to determine what the GPU has
        // already processed, and frees the resources that are no longer needed.
        self.context.previous_frame_end.as_mut().unwrap().cleanup_finished();

        // Whenever the window resizes we need to recreate everything dependent on the window size.
        // In this example that includes the swapchain, the framebuffers and the dynamic state viewport.
        if self.context.recreate_swapchain {
            // Get the new dimensions of the window.
            let dimensions: [u32; 2] = self.context.surface.window().inner_size().into();
            let (new_swapchain, new_images) = match self.context.swapchain.recreate_with_dimensions(dimensions) {
                Ok(r) => r,
                // This error tends to happen when the user is manually resizing the window.
                // Simply restarting the loop is the easiest way to fix this issue.
                Err(SwapchainCreationError::UnsupportedDimensions) => return,
                Err(e) => panic!("Failed to recreate swapchain: {:?}", e)
            };

            self.context.swapchain = new_swapchain;
            // Because framebuffers contains an Arc on the old swapchain, we need to
            // recreate framebuffers as well.
            self.context.render_target.framebuffers = window_size_dependent_setup(&new_images, self.context.render_target.render_pass.clone(), &mut self.context.dynamic_state);
            self.context.recreate_swapchain = false;
        }

        // Before we can draw on the output, we have to *acquire* an image from the swapchain. If
        // no image is available (which happens if you submit draw commands too quickly), then the
        // function will block.
        // This operation returns the index of the image that we are allowed to draw upon.
        //
        // This function can block if no image is available. The parameter is an optional timeout
        // after which the function call will return an error.
        let (image_num, suboptimal, acquire_future) = match swapchain::acquire_next_image(self.context.swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                self.context.recreate_swapchain = true;
                return;
            },
            Err(e) => panic!("Failed to acquire next image: {:?}", e)
        };

        // acquire_next_image can be successful, but suboptimal. This means that the swapchain image
        // will still work, but it may not display correctly. With some drivers this can be when
        // the window resizes, but it may not cause the swapchain to become out of date.
        if suboptimal {
            self.context.recreate_swapchain = true;
        }

        // Specify the color to clear the framebuffer with i.e. blue
        let clear_values = vec!([0.0, 0.0, 1.0, 1.0].into());

        // In order to draw, we have to build a *command buffer*. The command buffer object holds
        // the list of commands that are going to be executed.
        //
        // Building a command buffer is an expensive operation (usually a few hundred
        // microseconds), but it is known to be a hot path in the driver and is expected to be
        // optimized.
        //
        // Note that we have to pass a queue family when we create the command buffer. The command
        // buffer will only be executable on that given queue family.
        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(self.context.device.clone(), self.context.queue.family()).unwrap()
            // Before we can draw, we have to *enter a render pass*. There are two methods to do
            // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
            // not covered here.
            //
            // The third parameter builds the list of values to clear the attachments with. The API
            // is similar to the list of attachments when building the framebuffers, except that
            // only the attachments that use `load: Clear` appear in the list.
            .begin_render_pass(self.context.render_target.framebuffers[image_num].clone(), false, clear_values).unwrap()

            // We are now inside the first subpass of the render pass. We add a draw command.
            //
            // The last two parameters contain the list of resources to pass to the shaders.
            // Since we used an `EmptyPipeline` object, the objects have to be `()`.
            .draw(self.renderer.pipeline.clone(), &self.context.dynamic_state, self.vertex_buffer.clone(), (), ()).unwrap()

            // We leave the render pass by calling `draw_end`. Note that if we had multiple
            // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
            // next subpass.
            .end_render_pass().unwrap()

            // Finish building the command buffer by calling `build`.
            .build().unwrap();

        let future = self.context.previous_frame_end.take().unwrap()
            .join(acquire_future)
            .then_execute(self.context.queue.clone(), command_buffer).unwrap()

            // The color output is now expected to contain our triangle. But in order to show it on
            // the screen, we have to *present* the image by calling `present`.
            //
            // This function does not actually present the image immediately. Instead it submits a
            // present command at the end of the queue. This means that it will only be presented once
            // the GPU has finished executing the command buffer that draws the triangle.
            .then_swapchain_present(self.context.queue.clone(), self.context.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.context.previous_frame_end = Some(Box::new(future) as Box<_>);
            },
            Err(FlushError::OutOfDate) => {
                self.context.recreate_swapchain = true;
                self.context.previous_frame_end = Some(Box::new(sync::now(self.context.device.clone())) as Box<_>);
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.context.previous_frame_end = Some(Box::new(sync::now(self.context.device.clone())) as Box<_>);
            }
        }
    }
}