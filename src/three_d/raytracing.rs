//! This module provides support for a basic raytracing engine.
//!
//! The engine provides raytraced spheres and boxes, materials like metal and glass, and a camera to render them with.
//! 
//! It also provides traits for general materials and raytracing objects 

use rand::{Rng, thread_rng};
use crate::three_d::raytracing::vector::Vec3;

/// An axis-aligned bounding box
pub mod aabb;
/// A Bounding Volume Hierarchy
pub mod bvh;
/// A camera to render raytracing scenes
pub mod camera;
/// A record of information about a ray intersection
pub mod hit_record;
/// An interval along the number line
pub mod interval;
/// A material for objects that emit light
pub mod lights;
/// A raytracing material
pub mod material;
/// A Probability Density Function
pub mod pdf;
/// Renders the pixel data
pub mod opengl;
/// An othornormal basis, used for transforming vectors relative to prespectives
pub mod onb;
/// A ray, with an origin and direction
pub mod ray;
/// A trait representing a raytracing object
pub mod shape;
/// A 3-dimensional vector
pub mod vector;

/// Generate a random vector in a random direction, without having to reject rays
pub fn random_cosine_direction() -> Vec3 {
    let mut rng = thread_rng();
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * std::f32::consts::PI * r1;
    let (y, x) = phi.sin_cos();

    [x * r2.sqrt(), y * r2.sqrt(), z].into()
}
