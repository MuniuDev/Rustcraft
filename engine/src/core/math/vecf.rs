use super::veci;

pub struct Vecf {
    x: f32,
    y: f32,
    z: f32,
}

impl Vecf {
    pub fn zero() -> Self { return Vecf { x:0.0, y:0.0, z:0.0 } }
    pub fn new(_x: f32, _y: f32, _z: f32) -> Self { return Vecf { x:_x, y:_y, z:_z }; }

    pub fn to_veci(&self) -> veci::Veci {
        return veci::Veci::new(self.x as i32, self.y as i32, self.z as i32);
    }
}