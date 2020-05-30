use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::swapchain::Surface;
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode, ColorSpace, FullscreenExclusive};
use vulkano::command_buffer::DynamicState;
use vulkano::sync;
use vulkano::sync::GpuFuture;

use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::window::Window;

use std::sync::Arc;

use crate::rendering::common::*;
use crate::rendering::target::RenderTarget;

pub struct RenderContext {
    pub device : Arc<Device>,
    pub queue : Arc<vulkano::device::Queue>,
    pub render_target : RenderTarget,
    
    pub surface : Arc<Surface<Window>>,
    pub swapchain : Arc<Swapchain<Window>>,

    pub dynamic_state : DynamicState,
    pub recreate_swapchain : bool,
    pub previous_frame_end : Option<Box<dyn GpuFuture>>,
}

impl RenderContext {
    pub fn new(event_loop: &EventLoop<()>, instance : Arc<Instance>) -> Self {
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
            //let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let alpha = vulkano::swapchain::CompositeAlpha::Opaque;
    
            // Choosing the internal format that the images will have.
            println!("{:?}",caps.supported_formats);
            println!("{:?}",caps.supported_usage_flags);
            println!("{:?}",vulkano::swapchain::CompositeAlpha::Opaque);
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

        let mut dynamic_state = DynamicState { line_width: None, viewports: None, scissors: None, compare_mask: None, write_mask: None, reference: None };
        let framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);
        let recreate_swapchain = false;
        let previous_frame_end = Some(Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>);

        let render_target = RenderTarget {
            images,
            render_pass,
            framebuffers,
        };

        return RenderContext{
            device,
            queue,
            surface,
            swapchain,
            render_target,

            dynamic_state,
            recreate_swapchain,
            previous_frame_end,
        };
    }
}