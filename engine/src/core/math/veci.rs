use super::vecf;

pub struct Veci {
    x: i32,
    y: i32,
    z: i32,
}

impl Veci {
    pub fn zero() -> Self { return Veci { x:0, y:0, z:0 } }
    pub fn new(_x: i32, _y: i32, _z: i32) -> Self { return Veci { x:_x, y:_y, z:_z }; }

    pub fn to_vecf(&self) -> vecf::Vecf {
        return vecf::Vecf::new(self.x as f32, self.y as f32, self.z as f32);
    }
}