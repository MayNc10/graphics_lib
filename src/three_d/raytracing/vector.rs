use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    data: [f32; 3]
}

impl Vec3 {
    pub fn new(data: [f32; 3]) -> Vec3 { Vec3 { data } }
    pub fn data(&self) -> [f32; 3] { self.data }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { data: [self.data[0] + rhs.data[0], self.data[1] + rhs.data[1], self.data[2] + rhs.data[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { data: [self.data[0] - rhs.data[0], self.data[1] - rhs.data[1], self.data[2] - rhs.data[2]] }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 { data: [self.data[0] * rhs.data[0], self.data[1] * rhs.data[1], self.data[2] * rhs.data[2]] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 { data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs] }
    }
}

impl Mul<i32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f32;
        Vec3 { data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs] }
    }
}

impl Div<i32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f32;
        Vec3 { data: [self.data[0] / rhs, self.data[1] / rhs, self.data[2] / rhs] }
    }
}