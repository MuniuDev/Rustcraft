use winit::event::Event;
use winit::event_loop::ControlFlow;

pub trait App {
    fn onTick(&mut self, dt: std::time::Duration);
}