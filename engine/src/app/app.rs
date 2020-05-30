
pub trait App {
    fn on_tick(&mut self, dt: std::time::Duration);
}