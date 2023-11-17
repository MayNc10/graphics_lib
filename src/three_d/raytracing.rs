use rand::{Rng, thread_rng};
use crate::three_d::raytracing::vector::Vec3;

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hit_record;
pub mod interval;
pub mod lights;
pub mod material;
pub mod opengl;
pub mod onb;
pub mod ray;
pub mod shape;
pub mod vector;

pub fn random_cosine_direction() -> Vec3 {
    let mut rng = thread_rng();
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * std::f32::consts::PI * r1;
    let (y, x) = phi.sin_cos();

    [x * r2.sqrt(), y * r2.sqrt(), z].into()
}
