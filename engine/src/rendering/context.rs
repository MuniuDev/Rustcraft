use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::swapchain::Surface;
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode, ColorSpace, FullscreenExclusive};
use vulkano::image::swapchain::SwapchainImage;
use vulkano::command_buffer::DynamicState;
use vulkano::sync;
use vulkano::sync::GpuFuture;
use vulkano::framebuffer::{Subpass, RenderPassAbstract};

use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::window::Window;
use winit::window::WindowId;
use winit::event_loop::EventLoopWindowTarget;

use std::sync::Arc;
use std::collections::HashMap;


use crate::rendering::common::*;
use crate::rendering::target::RenderTarget;
use crate::rendering::error::RenderingError;
use crate::rendering::config;
use crate::rendering::window::WindowContext;

use crate::rendering::geometry::{Geometry, GeometryId};

use crate::core::*;


pub struct RenderContext {
    pub device : Arc<Device>,
    pub queue : Arc<vulkano::device::Queue>,
    pub default_window_render_pass : Arc<dyn RenderPassAbstract + Send + Sync>,
    pub windows : HashMap<WindowId, WindowContext>,
    
    geometry_id_counter : GeometryId,
    pub geometries : HashMap<GeometryId, Geometry>,
}

impl RenderContext {
    pub fn new(elwt : &EventLoopWindowTarget<()>, instance : Arc<Instance>) -> Self {
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
            .find(|&q| q.supports_graphics()  /*&& surface.is_supported(q).unwrap_or(false)*/)
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
        
        let default_window_render_pass = Arc::new(vulkano::single_pass_renderpass!(
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
                    format: config::DEFAULT_WINDOW_FORMAT,
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

        //let window_context = WindowContext::new(elwt, instance.clone(), device.clone(), queue.clone(), default_window_render_pass.clone());
        //let window_id = window_context.id();
        let mut windows = HashMap::<WindowId, WindowContext>::new();
        //windows.insert(window_id, window_context);

        return RenderContext{
            device,
            queue,
            default_window_render_pass, 
            windows,
            geometry_id_counter: 0,
            geometries: HashMap::new()
        };
    }

    pub fn create_window(&mut self, elwt: &EventLoopWindowTarget<()>, name: &str) -> WindowId {
        let window_context = WindowContext::new(
            elwt, 
            self.device.instance().clone(), 
            self.device.clone(), 
            self.queue.clone(), 
            self.default_window_render_pass.clone());
        
        let window_id = window_context.id();
        self.windows.insert(window_id, window_context);
        return window_id;
    }

    pub fn close_window(&mut self, window_id: WindowId) -> Result<(), RenderingError> {
        match self.windows.remove(&window_id) {
            Some(_) => { return Ok(()) },
            None => { return Err(RenderingError::WindowNotFound)}
        }
    }

    pub fn window_count(&self) -> usize { return self.windows.len(); }

    pub fn create_geometry(&mut self, data: &Vec<na::Vector3<FpScalar>>) -> GeometryId {
        let geometry_id = self.geometry_id_counter;
        self.geometry_id_counter += 1;
        self.geometries.insert(geometry_id, Geometry::from_data(self.device.clone(), data));
        return geometry_id;
    }
}