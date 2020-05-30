use crate::app::App;

pub struct ServerApp {
}

impl ServerApp {
    pub fn new() -> Self {
        return ServerApp{
        };
    }
}

impl App for ServerApp {
    fn onTick(&mut self, dt: std::time::Duration) {
        println!("Server tick: dt={:?}", dt);
    }
}