use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

use crate::app::App;
use crate::rendering::RenderingSystem;

pub struct ClientApp {
    rendering_system : RenderingSystem
}

impl ClientApp {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let rendering_system = RenderingSystem::new(&event_loop);
        return ClientApp{
            rendering_system
        };
    }

    pub fn onEvent(&mut self, event : Event<'_, ()>, control_flow: &mut ControlFlow) {
        self.rendering_system.run_impl(event, control_flow);
    }
}

impl App for ClientApp {
    fn onTick(&mut self, dt: std::time::Duration) {
        println!("Client tick: dt={:?}", dt);
    }
}