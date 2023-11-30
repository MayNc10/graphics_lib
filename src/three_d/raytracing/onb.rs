use std::ops::{Index, IndexMut};
use crate::three_d::raytracing::vector::Vec3;

/// A struct representing an orthonormal basis
/// An orthonormal basis is a collection of three vectors that are are perpendicular to each other
#[derive(Default)]
pub struct ONB {
    axes: [Vec3; 3],
}

impl ONB {
    /// Get the first vector
    pub fn u(&self) -> Vec3 {
        self.axes[0]
    }
    /// Get the second vector
    pub fn v(&self) -> Vec3 {
        self.axes[1]
    }
    /// Get the third vector
    pub fn w(&self) -> Vec3 {
        self.axes[2]
    }

    /// Transform the input vector (as component values) to be relative to the ONB
    pub fn local(&self, a: f32, b: f32, c: f32) -> Vec3 {
        self.u() * a + self.v() * b + self.w() * c
    }
    /// Transform the input vector to be relative to the ONB
    pub fn local_from_vector(&self, a: Vec3) -> Vec3 {
        self.u() * a.x() + self.v() * a.y() + self.w() * a.z()
    }
    /// Create an ONB from a given vector
    pub fn build_from_w(w: Vec3) -> ONB {
        let unit_w = w.unit();
        let a = if unit_w.x().abs() > 0.9 { [0.0, 1.0, 0.0].into() } else { [1.0, 0.0, 0.0].into() };
        let v = Vec3::cross(&unit_w, &a).unit();
        let u = Vec3::cross(&unit_w, &v);
        ONB { axes: [u, v, unit_w] }
    }
}

impl Index<usize> for ONB {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.axes[index]
    }
}

impl IndexMut<usize> for ONB {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.axes[index]
    }
}