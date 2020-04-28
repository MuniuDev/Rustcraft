use super::map;

#[derive(Clone)]
pub struct World {
    maps: Vec<map::Map>
}

impl World {
    pub fn new() -> Self { return World{maps:vec![map::Map::new()]}; }
    pub fn load(path: &str) -> Self { return World::new(); }
    pub fn save(path: &str) {}
}