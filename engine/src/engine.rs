use crate::model::world::World;


pub struct Engine {

}

impl Engine {
    pub fn new() -> Self { return Engine{}; }

    pub fn update(&mut self, world: &mut World, dt: std::time::Duration) {

    }
}