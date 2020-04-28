use crate::config;
use crate::io;
use crate::system;
use crate::util;
use crate::model;
use crate::core;

struct Engine {

}

impl Engine {
    pub fn new() -> Self { return Engine{}; }

    pub fn selfhost(&mut self, world: model::world::World) { }

    pub fn connectClient(&mut self,) {}

    pub fn clientUpdate(&mut self, dt: f32) {}
    pub fn serverUpdate(&mut self, dt: f32) {}
}