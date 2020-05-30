mod client;

use winit::event_loop::EventLoop;
use rustcraft_engine::app::App;

use client::ClientApp;

fn main() {
    rustcraft_engine::app::print_feat();
    let event_loop = EventLoop::new();
    let mut app = ClientApp::new(&event_loop);
    event_loop.run(move |event, _, control_flow| { app.on_event(event, control_flow) });
}