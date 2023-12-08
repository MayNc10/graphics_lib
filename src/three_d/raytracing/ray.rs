use super::vector::Vec3;

/// A struct to represent a Ray
/// A Ray has an origin point and a direction that it points in
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    /// Create a new ray, given the ray's origin and direction
    pub fn new(origin: Vec3, direction: Vec3) -> Ray { Ray { origin, direction } }
    /// Get the origin of the ray
    pub fn origin(&self) -> Vec3 { self.origin }
    /// Get the direction of the ray
    pub fn direction(&self) -> Vec3 { self.direction }
    /// Compute the endpoint given some progression t along the ray
    /// Increasing the t value by one will move the endpoint by the magnitude of the direction vector, in the direction of that vector
    pub fn at(&self, t: f32) -> Vec3 { self.origin + self.direction * t }
}