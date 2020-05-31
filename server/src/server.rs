use rustcraft_engine::engine::Engine;
use rustcraft_engine::model::World;

pub struct ServerApp {
    is_running : bool,
    engine: Engine,
    world: Option<World>,
}

impl ServerApp {
    pub fn new() -> Self {
        let engine = Engine::new();
        let world = World::new();

        return ServerApp{
            is_running: true,
            engine,
            world: None,
        };
    }

    pub fn is_running(&self) -> bool {
        return self.is_running;
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        println!("Server update: dt={:?}", dt);
        //self.engine.update(&mut self.world, dt);
        std::thread::sleep_ms(1000);

    }
}