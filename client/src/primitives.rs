use rustcraft_engine::core::*;


pub fn generate_box(scale: FpScalar) -> Vec<na::Vector3::<FpScalar>> {
    return vec![
        na::Vector3::<FpScalar>::new(-0.5, -0.5, 0.0) * scale,
        na::Vector3::<FpScalar>::new(0.5, 0.5, 0.0) * scale,
        na::Vector3::<FpScalar>::new(0.5, -0.5, 0.0) * scale,
        na::Vector3::<FpScalar>::new(-0.5, -0.5, 0.0) * scale,
        na::Vector3::<FpScalar>::new(-0.5, 0.5, 0.0) * scale,
        na::Vector3::<FpScalar>::new(0.5, 0.5, 0.0) * scale,
    ];
}