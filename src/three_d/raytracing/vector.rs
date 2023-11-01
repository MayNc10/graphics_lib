use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    data: [f32; 3]
}

impl Vec3 {
    pub fn new(data: [f32; 3]) -> Vec3 { Vec3 { data } }
    pub fn data(&self) -> [f32; 3] { self.data }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.data[0].powi(2) + self.data[1].powi(2) + self.data[2].powi(2))
    }
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn x(&self) -> f32 { self.data[0] }
    pub fn y(&self) -> f32 { self.data[1] }
    pub fn z(&self) -> f32 { self.data[2] }

    pub fn length_squared(&self) -> f32 {
        Vec3::dot(self, self)
    }
}

impl Vec3 {
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }
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

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 { data: [self.data[0] / rhs, self.data[1] / rhs, self.data[2] / rhs] }
    }
}

impl Div<i32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f32;
        Vec3 { data: [self.data[0] / rhs, self.data[1] / rhs, self.data[2] / rhs] }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new([0.0; 3])
    }
}