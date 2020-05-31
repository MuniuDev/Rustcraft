mod client;

use winit::event_loop::EventLoop;
use client::ClientApp;

fn main() {
    println!("{:?}", rustcraft_engine::Features::enabled());
    
    let event_loop = EventLoop::new();
    let mut app = ClientApp::new(&event_loop);
    event_loop.run(move |event, elwt, control_flow| { app.on_event(event, elwt, control_flow) });
}