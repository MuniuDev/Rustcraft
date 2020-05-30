use winit::event_loop::EventLoop;
use rustcraft_engine::app::ClientApp;

fn main() {
    let event_loop = EventLoop::new();
    let mut app = ClientApp::new(&event_loop);
    event_loop.run(move |event, _, control_flow| { app.onEvent(event, control_flow) });
}