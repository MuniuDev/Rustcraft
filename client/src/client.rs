use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

use std::time::{Duration, Instant};

use rustcraft_engine::app::App;
use rustcraft_engine::rendering::RenderingSystem;

pub struct ClientApp {
    rendering_system : RenderingSystem,
    last_tick_instant : Instant
}

impl ClientApp {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let rendering_system = RenderingSystem::new(&event_loop);
        return ClientApp{
            rendering_system,
            last_tick_instant: Instant::now()
        };
    }

    pub fn on_event(&mut self, event : Event<'_, ()>, control_flow: &mut ControlFlow) {
        println!("{:?}",event);
        match event {
            // Emitted before any events in specific frame.
            Event::NewEvents(start_cause) => {
                match start_cause {
                    Init => {
                        self.last_tick_instant = Instant::now();
                    }
                    _ => {}
                }
            },
            // Window related events category
            Event::WindowEvent{window_id, event} => {},
            // User I/O related events category
            Event::DeviceEvent{device_id, event} => {},
            // Custom user event
            Event::UserEvent(user_data) => {},
            // Emmited when the application gets suspended.
            Event::Suspended => {},
            // Emmited when the application gets unsuspended.
            Event::Resumed => {},
            // Emmited after all non-rendering events got processed.
            Event::MainEventsCleared => {
                let now_instant = Instant::now();
                let dt = now_instant - self.last_tick_instant;
                self.on_tick(dt);
                self.last_tick_instant = now_instant;
            },
            
            // Emmited when a window should be redrawn
            Event::RedrawRequested(window_id) => {},
            // Emmited after all RedrawRequested events have been processed.
            Event::RedrawEventsCleared => {},
            
            // Emitted when the event loop is being shut down.
            Event::LoopDestroyed => {},
        }
    }
}

impl App for ClientApp {
    fn on_tick(&mut self, dt: std::time::Duration) {
        println!("Server tick: dt={:?}", dt);
        let ms16 = std::time::Duration::from_millis(16);
        if dt < ms16 {
            std::thread::sleep(ms16 - dt);
        }
    }
}