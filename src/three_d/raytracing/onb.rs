use std::ops::{Index, IndexMut};
use crate::three_d::raytracing::vector::Vec3;

#[derive(Default)]
pub struct ONB {
    axes: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axes[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axes[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axes[2]
    }

    pub fn local(&self, a: f32, b: f32, c: f32) -> Vec3 {
        self.u() * a + self.v() * b + self.w() * c
    }
    pub fn local_from_vector(&self, a: Vec3) -> Vec3 {
        self.u() * a.x() + self.v() * a.y() + self.w() * a.z()
    }
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