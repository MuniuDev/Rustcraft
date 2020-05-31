pub struct ServerApp {
    is_running : bool
}

impl ServerApp {
    pub fn new() -> Self {
        return ServerApp{
            is_running: true
        };
    }

    pub fn is_running(&self) -> bool {
        return self.is_running;
    }

    fn update(&mut self, dt: std::time::Duration) {
        println!("Server tick: dt={:?}", dt);
        let ms16 = std::time::Duration::from_millis(16);
        if dt < ms16 {
            std::thread::sleep(ms16 - dt);
        }
    }
}