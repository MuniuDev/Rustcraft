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

pub struct RenderTarget {
    pub images : Vec<Arc<SwapchainImage<Window>>>,
    pub framebuffers : Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    pub render_pass : Arc<dyn RenderPassAbstract + Send + Sync>,
}
