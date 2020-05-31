use nalgebra::{Vector2, Vector3, Scalar};

pub fn vec2_to_vec3<T: Scalar + Copy>(v2: Vector2<T>, z: T) -> Vector3<T> {
    return Vector3::new(v2.x, v2.y, z);
}

pub fn vec3_to_vec2<T: Scalar + Copy>(v3: Vector3<T>) -> Vector2<T> {
    return Vector2::new(v3.x, v3.y);
}