#[macro_use]
extern crate static_assertions;
#[macro_use]
extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;

mod client;
mod primitives;

use winit::event_loop::EventLoop;
use client::ClientApp;

fn main() {
    println!("{:?}", rustcraft_engine::Features::enabled());
    
    let event_loop = EventLoop::new();
    let mut app = ClientApp::new(&event_loop);
    event_loop.run(move |event, elwt, control_flow| { app.on_event(event, elwt, control_flow) });
}