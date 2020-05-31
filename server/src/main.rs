mod server;

use std::time::{Duration, Instant};

use rustcraft_engine::app::App;

use server::ServerApp;


fn main() {
    println!("{:?}", rustcraft_engine::Features::enabled());

    let mut app = ServerApp::new();

    let mut last_tick_instant = Instant::now();
    while app.is_running() {
        let now_instant = Instant::now();
        let dt = now_instant - last_tick_instant;
        app.update(dt);
        last_tick_instant = now_instant;
    }
}