use super::map;

#[derive(Clone)]
pub struct World {
    map: Box<map::Map>
}

impl World {
    pub fn new() -> Self { return World{map: map::Map::new()}; }
    pub fn load(path: &str) -> Self { return World::new(); }
    pub fn save(path: &str) {}
}