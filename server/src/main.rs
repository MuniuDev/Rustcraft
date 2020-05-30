mod server;

use std::time::{Duration, Instant};

use rustcraft_engine::app::App;

use server::ServerApp;


fn main() {
    let mut app = ServerApp::new();

    let mut last_tick_instant = Instant::now();
    while app.is_running() {
        let now_instant = Instant::now();
        let dt = now_instant - last_tick_instant;
        app.on_tick(dt);
        last_tick_instant = now_instant;
    }
}