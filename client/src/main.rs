use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::swapchain::Surface;
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode, ColorSpace, FullscreenExclusive};
use vulkano::image::swapchain::SwapchainImage;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain;
use vulkano::sync;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::format::Format;

use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};
use winit::window::WindowBuilder;
use winit::window::Window;
use winit::event_loop::ControlFlow;
use vulkano::swapchain::{AcquireError, SwapchainCreationError};

use std::sync::Arc;
use std::vec::Vec;

#[derive(Default, Debug, Clone)]
struct Vertex { position: [f32; 2] }
vulkano::impl_vertex!(Vertex, position);

fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0 .. 1.0,
    };
    dynamic_state.viewports = Some(vec!(viewport));

    images.iter().map(|image| {
        Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(image.clone()).unwrap()
                .build().unwrap()
        ) as Arc<dyn FramebufferAbstract + Send + Sync>
    }).collect::<Vec<_>>()
}

pub struct Renderer {
    instance : Arc<Instance>,
    render_device : RenderDevice,
    render_stage : RenderStage,
    window_handle : WindowHandle,
    
    vertex_buffer : Vec<Arc<dyn vulkano::buffer::BufferAccess + Send + Sync>>,
}

pub struct RenderDevice {
    device : Arc<Device>,
	queue : Arc<vulkano::device::Queue>,
}

pub struct RenderStage {
    render_pass : Arc<dyn RenderPassAbstract + Send + Sync>,
	pipeline : Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
}

pub struct WindowHandle {
    surface : Arc<Surface<Window>>,
	swapchain : Arc<Swapchain<Window>>,
    images : Vec<Arc<SwapchainImage<Window>>>,
    dynamic_state : DynamicState,
	framebuffers : Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
	recreate_swapchain : bool,
	previous_frame_end : Option<Box<dyn GpuFuture>>,
}

impl Renderer {
	pub fn new(event_loop: &EventLoop<()>) -> Self {
		// Instance
		let instance = Instance::new(None, &vulkano_win::required_extensions(), None)
		.expect("failed to create instance");

		// EventLoop + Surface setup
		let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone()).unwrap();

        // Choose queue family
        println!("Available physical devices:");
        for dev in PhysicalDevice::enumerate(&instance) {
            println!("\t{}. {}, API: {}", dev.index(), dev.name(), dev.api_version());
        }
        let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");
        println!("Using {} as physical device.", physical.name());
 
        println!("Available queue families:");
		for family in physical.queue_families() {
            println!("ID: {} Queue count: {} Graphics: {} Compute: {} Transfer: {} Sparse bindings: {}", 
            family.id(),
            family.queues_count(),
            family.supports_graphics(),
            family.supports_compute(),
            family.explicitly_supports_transfers(),
            family.supports_sparse_binding());
		}
		let queue_family = physical.queue_families()
			.find(|&q| q.supports_graphics()  && surface.is_supported(q).unwrap_or(false))
			.expect("couldn't find a graphical queue family");

		// Device + queues
		let (device, mut queues) = {
			let device_ext = vulkano::device::DeviceExtensions {
				khr_swapchain: true,
				.. vulkano::device::DeviceExtensions::none()
			};
		
			Device::new(physical, physical.supported_features(), &device_ext,
						[(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
		};
		let queue = queues.next().unwrap();

		let (swapchain, images) = {
			// Querying the capabilities of the surface. When we create the swapchain we can only
			// pass values that are allowed by the capabilities.
			let caps = surface.capabilities(physical).unwrap();
			let usage = caps.supported_usage_flags;
	
			// The alpha mode indicates how the alpha value of the final image will behave. For example
			// you can choose whether the window will be opaque or transparent.
			let alpha = caps.supported_composite_alpha.iter().next().unwrap();
	
            // Choosing the internal format that the images will have.
            println!("{:?}",caps.supported_formats);
            //let format = caps.supported_formats[0].0;
			let format = vulkano::format::Format::B8G8R8A8Unorm;
	
			// The dimensions of the window, only used to initially setup the swapchain.
			// NOTE:
			// On some drivers the swapchain dimensions are specified by `caps.current_extent` and the
			// swapchain size must use these dimensions.
			// These dimensions are always the same as the window dimensions
			//
			// However other drivers dont specify a value i.e. `caps.current_extent` is `None`
			// These drivers will allow anything but the only sensible value is the window dimensions.
			//
			// Because for both of these cases, the swapchain needs to be the window dimensions, we just use that.
			let dimensions: [u32; 2] = surface.window().inner_size().into();
	
			// Please take a look at the docs for the meaning of the parameters we didn't mention.
			Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
				dimensions, 1, usage, &queue, SurfaceTransform::Identity, alpha,
				PresentMode::Fifo, FullscreenExclusive::Default, true, ColorSpace::SrgbNonLinear).unwrap()
	
		};
	

		mod vs {
			vulkano_shaders::shader!{
				ty: "vertex",
				src: "
					#version 450
					layout(location = 0) in vec2 position;
					void main() {
						gl_Position = vec4(position, 0.0, 1.0);
					}
				"
			}
		}
	
		mod fs {
			vulkano_shaders::shader!{
				ty: "fragment",
				src: "
					#version 450
					layout(location = 0) out vec4 f_color;
					void main() {
						f_color = vec4(1.0, 0.0, 0.0, 1.0);
					}
				"
			}
		}
	
		let vs = vs::Shader::load(device.clone()).unwrap();
		let fs = fs::Shader::load(device.clone()).unwrap();

		let render_pass = Arc::new(vulkano::single_pass_renderpass!(
			device.clone(),
			attachments: {
				// `color` is a custom name we give to the first and only attachment.
				color: {
					// `load: Clear` means that we ask the GPU to clear the content of this
					// attachment at the start of the drawing.
					load: Clear,
					// `store: Store` means that we ask the GPU to store the output of the draw
					// in the actual image. We could also ask it to discard the result.
					store: Store,
					// `format: <ty>` indicates the type of the format of the image. This has to
					// be one of the types of the `vulkano::format` module (or alternatively one
					// of your structs that implements the `FormatDesc` trait). Here we use the
					// same format as the swapchain.
					format: swapchain.format(),
					// TODO:
					samples: 1,
				}
			},
			pass: {
				// We use the attachment named `color` as the one and only color attachment.
				color: [color],
				// No depth-stencil attachment is indicated with empty brackets.
				depth_stencil: {}
			}
		).unwrap());

		let pipeline = Arc::new(GraphicsPipeline::start()
        // We need to indicate the layout of the vertices.
        // The type `SingleBufferDefinition` actually contains a template parameter corresponding
        // to the type of each vertex. But in this code it is automatically inferred.
        .vertex_input_single_buffer::<Vertex>()
        // A Vulkan shader can in theory contain multiple entry points, so we have to specify
        // which one. The `main` word of `main_entry_point` actually corresponds to the name of
        // the entry point.
        .vertex_shader(vs.main_entry_point(), ())
        // The content of the vertex buffer describes a list of triangles.
        .triangle_list()
        // Use a resizable viewport set to draw over the entire window
        .viewports_dynamic_scissors_irrelevant(1)
        // See `vertex_shader`.
        .fragment_shader(fs.main_entry_point(), ())
        // We have to indicate which subpass of which render pass this pipeline is going to be used
        // in. The pipeline will only be usable from this particular subpass.
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
        .build(device.clone())
		.unwrap());

		let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, [
            Vertex { position: [-0.5, -0.25] },
            Vertex { position: [0.0, 0.5] },
            Vertex { position: [0.25, -0.1] }
		].iter().cloned()).unwrap();
		
		let mut dynamic_state = DynamicState { line_width: None, viewports: None, scissors: None, compare_mask: None, write_mask: None, reference: None };
		let framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);
		let recreate_swapchain = false;
		let previous_frame_end = Some(Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>);

        let render_device = RenderDevice {
            device,
			queue,
        };

        let render_stage = RenderStage {
            render_pass,
            pipeline
        };

        let window_handle = WindowHandle {
            surface,
			swapchain,
            images,
            dynamic_state,
			framebuffers,
            recreate_swapchain,
            previous_frame_end,
        };

		return Renderer{
			instance,
            render_device,
            render_stage,
            window_handle,
			
			vertex_buffer: vec![vertex_buffer],
		};
	}

	fn run_impl(&mut self, event : Event<'_, ()>, control_flow: &mut ControlFlow) {
		match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                self.window_handle.recreate_swapchain = true;
            },
            Event::RedrawEventsCleared => {
                // It is important to call this function from time to time, otherwise resources will keep
                // accumulating and you will eventually reach an out of memory error.
                // Calling this function polls various fences in order to determine what the GPU has
                // already processed, and frees the resources that are no longer needed.
                self.window_handle.previous_frame_end.as_mut().unwrap().cleanup_finished();

                // Whenever the window resizes we need to recreate everything dependent on the window size.
                // In this example that includes the swapchain, the framebuffers and the dynamic state viewport.
                if self.window_handle.recreate_swapchain {
                    // Get the new dimensions of the window.
                    let dimensions: [u32; 2] = self.window_handle.surface.window().inner_size().into();
                    let (new_swapchain, new_images) = match self.window_handle.swapchain.recreate_with_dimensions(dimensions) {
                        Ok(r) => r,
                        // This error tends to happen when the user is manually resizing the window.
                        // Simply restarting the loop is the easiest way to fix this issue.
                        Err(SwapchainCreationError::UnsupportedDimensions) => return,
                        Err(e) => panic!("Failed to recreate swapchain: {:?}", e)
                    };

                    self.window_handle.swapchain = new_swapchain;
                    // Because framebuffers contains an Arc on the old swapchain, we need to
                    // recreate framebuffers as well.
                    self.window_handle.framebuffers = window_size_dependent_setup(&new_images, self.render_stage.render_pass.clone(), &mut self.window_handle.dynamic_state);
                    self.window_handle.recreate_swapchain = false;
                }

                // Before we can draw on the output, we have to *acquire* an image from the swapchain. If
                // no image is available (which happens if you submit draw commands too quickly), then the
                // function will block.
                // This operation returns the index of the image that we are allowed to draw upon.
                //
                // This function can block if no image is available. The parameter is an optional timeout
                // after which the function call will return an error.
                let (image_num, suboptimal, acquire_future) = match swapchain::acquire_next_image(self.window_handle.swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        self.window_handle.recreate_swapchain = true;
                        return;
                    },
                    Err(e) => panic!("Failed to acquire next image: {:?}", e)
                };

                // acquire_next_image can be successful, but suboptimal. This means that the swapchain image
                // will still work, but it may not display correctly. With some drivers this can be when
                // the window resizes, but it may not cause the swapchain to become out of date.
                if suboptimal {
                    self.window_handle.recreate_swapchain = true;
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
                let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(self.render_device.device.clone(), self.render_device.queue.family()).unwrap()
                    // Before we can draw, we have to *enter a render pass*. There are two methods to do
                    // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
                    // not covered here.
                    //
                    // The third parameter builds the list of values to clear the attachments with. The API
                    // is similar to the list of attachments when building the framebuffers, except that
                    // only the attachments that use `load: Clear` appear in the list.
                    .begin_render_pass(self.window_handle.framebuffers[image_num].clone(), false, clear_values).unwrap()

                    // We are now inside the first subpass of the render pass. We add a draw command.
                    //
                    // The last two parameters contain the list of resources to pass to the shaders.
                    // Since we used an `EmptyPipeline` object, the objects have to be `()`.
                    .draw(self.render_stage.pipeline.clone(), &self.window_handle.dynamic_state, self.vertex_buffer.clone(), (), ()).unwrap()

                    // We leave the render pass by calling `draw_end`. Note that if we had multiple
                    // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
                    // next subpass.
                    .end_render_pass().unwrap()

                    // Finish building the command buffer by calling `build`.
                    .build().unwrap();

                let future = self.window_handle.previous_frame_end.take().unwrap()
                    .join(acquire_future)
                    .then_execute(self.render_device.queue.clone(), command_buffer).unwrap()

                    // The color output is now expected to contain our triangle. But in order to show it on
                    // the screen, we have to *present* the image by calling `present`.
                    //
                    // This function does not actually present the image immediately. Instead it submits a
                    // present command at the end of the queue. This means that it will only be presented once
                    // the GPU has finished executing the command buffer that draws the triangle.
                    .then_swapchain_present(self.render_device.queue.clone(), self.window_handle.swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        self.window_handle.previous_frame_end = Some(Box::new(future) as Box<_>);
                    },
                    Err(FlushError::OutOfDate) => {
                        self.window_handle.recreate_swapchain = true;
                        self.window_handle.previous_frame_end = Some(Box::new(sync::now(self.render_device.device.clone())) as Box<_>);
                    }
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        self.window_handle.previous_frame_end = Some(Box::new(sync::now(self.render_device.device.clone())) as Box<_>);
                    }
                }
            },
            _ => ()
        }
	}
}

fn main() {
	let event_loop = EventLoop::new();
	let mut renderer = Renderer::new(&event_loop);
	event_loop.run(move |event, _, control_flow| { renderer.run_impl(event, control_flow)});
}