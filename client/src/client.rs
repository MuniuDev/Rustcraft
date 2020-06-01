use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;
use winit::event::DeviceId;
use winit::event::DeviceEvent;
use winit::event::StartCause;

use std::time::{Duration, Instant};

use rustcraft_engine::rendering::{RenderingSystem};
use rustcraft_engine::engine::Engine;
use rustcraft_engine::model::World;
use rustcraft_engine::core::*;

use crate::primitives;

use std::sync::Arc;

pub struct ClientApp {
    rendering_system : RenderingSystem,
    engine: Engine,
    world: Option<World>,

    last_tick_instant : Instant
}

impl ClientApp {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let engine = Engine::new();
        let mut rendering_system = RenderingSystem::new(&event_loop);
        rendering_system.open_window(event_loop, "Rustcraft client");
        let geometry_id = rendering_system.create_geometry(&primitives::generate_box(1.0));
        return ClientApp{
            engine,
            rendering_system,
            world: Some(World::new()),
            last_tick_instant: Instant::now()
        };
    }

    // Power states
    fn on_init(&mut self) {
        self.last_tick_instant = Instant::now();
    }
    fn on_suspend(&mut self) {}
    fn on_resume(&mut self) {}
    fn on_close(&mut self) {}
    
    // Main Loop
    
    fn on_update(&mut self) {
        let now_instant = Instant::now();
        let dt = now_instant - self.last_tick_instant;
        self.update(dt);
        self.last_tick_instant = now_instant;
    }
    fn on_redraw(&mut self, window_id: WindowId) {}
    fn on_draw(&mut self) { self.rendering_system.end_frame(); }

    fn update(&mut self, dt: std::time::Duration) {
        println!("Client update: dt={:?}", dt);
        match self.world.as_mut() {
            Some(world) => { self.engine.update(world, dt); }
            _ => {}
        }
    }

    // Event handling
    pub fn on_event(&mut self, event : Event<'_, ()>, elwt : &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow) {
        match event {
            // Emitted before any events in specific frame.
            Event::NewEvents(start_cause) => { 
                match start_cause {
                    StartCause::Init => { self.on_init(); },
                    StartCause::ResumeTimeReached{start, requested_resume} => {},
                    StartCause::WaitCancelled{start, requested_resume} => {},
                    StartCause::Poll => {},
                }
            },
            // Window related events category
            Event::WindowEvent{window_id, event} => { self.on_window_event(window_id, event, elwt, control_flow); },
            // User I/O related events category
            Event::DeviceEvent{device_id, event} => { self.on_device_event(device_id, event, elwt, control_flow); },
            // Custom user event
            Event::UserEvent(user_data) => { println!("UNKNOWN USER EVENT: {:?}", user_data); },
            // Emmited when the application gets suspended.
            Event::Suspended => { self.on_suspend(); },
            // Emmited when the application gets unsuspended.
            Event::Resumed => { self.on_resume(); },
            // Emmited after all non-rendering events got processed.
            Event::MainEventsCleared => { self.on_update(); },
            
            // Emmited when a window should be redrawn
            Event::RedrawRequested(window_id) => { self.on_redraw(window_id); },
            // Emmited after all RedrawRequested events have been processed.
            Event::RedrawEventsCleared => { self.on_draw(); },
            
            // Emitted when the event loop is being shut down.
            Event::LoopDestroyed => { self.on_close(); },
        }
    }

    fn on_window_event(&mut self, window_id: WindowId, event : WindowEvent, elwt : &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::CloseRequested => {
                let was_last = self.rendering_system.close_window(window_id);
                if was_last {
                    *control_flow = ControlFlow::Exit;
                }
            }
            WindowEvent::Resized(size) => {
                self.rendering_system.window_resized(window_id, size);
            }
            WindowEvent::Moved(position) => {},
            WindowEvent::Destroyed => {},
            WindowEvent::DroppedFile(path_buf) => {},
            WindowEvent::HoveredFile(path_buf) => {},
            WindowEvent::HoveredFileCancelled => {},
            WindowEvent::ReceivedCharacter(c) => {},
            WindowEvent::Focused(gained) => {},
            WindowEvent::KeyboardInput{device_id, input, is_synthetic} => {},
            WindowEvent::ModifiersChanged(ModifiersState) => {},
            WindowEvent::CursorMoved{device_id, position, modifiers} => {},
            WindowEvent::CursorEntered{device_id} => {},
            WindowEvent::CursorLeft{device_id} => {},
            WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {},
            WindowEvent::MouseInput { device_id, state, button, modifiers } => {},
            WindowEvent::TouchpadPressure { device_id, pressure, stage } => {},
            WindowEvent::AxisMotion { device_id, axis, value } => {},
            WindowEvent::Touch(touch) => {},
            WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {},
            WindowEvent::ThemeChanged(theme) => {},
        }
    }

    fn on_device_event(&mut self, device_id: DeviceId, event : DeviceEvent, elwt : &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow) {
        // TODO Implement raw input handling if necessary.
        match event {
            DeviceEvent::Added => {},
            DeviceEvent::Removed => {},
            DeviceEvent::MouseMotion { delta } => {},
            DeviceEvent::MouseWheel { delta } => {},
            DeviceEvent::Motion { axis, value } => {},
            DeviceEvent::Button { button, state } => {},
            DeviceEvent::Key(keyboard_input) => {},
            DeviceEvent::Text { codepoint } => {},
        }
    }
}