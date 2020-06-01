#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RenderingError {
    RecreateSwapchainFailed,
    ImageAcquireFailed,
    WindowNotFound,
}