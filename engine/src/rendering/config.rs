use vulkano::swapchain::{ColorSpace, CompositeAlpha};
use vulkano::format::Format;


pub const DEFAULT_COLOR_SPACE: ColorSpace = ColorSpace::SrgbNonLinear;

pub const DEFAULT_WINDOW_FORMAT: Format = Format::B8G8R8A8Unorm;
pub const DEFAULT_WINDOW_ALPHA: CompositeAlpha = CompositeAlpha::Opaque;