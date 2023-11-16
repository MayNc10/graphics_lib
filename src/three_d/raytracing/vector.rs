use std::ffi::OsString;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Range, RangeInclusive, Sub};
use clap::Parser;
use rand::distributions::uniform::SampleRange;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::three_d::raytracing::aabb::AABB;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub data: [f32; 3]
}

const EPSILON: f32 = 1e-8;

impl Vec3 {
    pub fn new(data: [f32; 3]) -> Vec3 { Vec3 { data } }
    pub fn data(&self) -> [f32; 3] { self.data }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.data[0].powi(2) + self.data[1].powi(2) + self.data[2].powi(2))
    }
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
    pub fn to_unit(self) -> Vec3 { self / self.length() }

    pub fn x(&self) -> f32 { self.data[0] }
    pub fn y(&self) -> f32 { self.data[1] }
    pub fn z(&self) -> f32 { self.data[2] }

    pub fn for_each(&mut self, func: &fn(f32) -> f32) {
        self.data[0] = func(self.data[0]);
        self.data[1] = func(self.data[1]);
        self.data[2] = func(self.data[2]);
    }

    pub fn length_squared(&self) -> f32 {
        Vec3::dot(self, self)
    }

    pub fn near_zero(&self) -> bool {
        self.data[0].abs() < EPSILON && self.data[1].abs() < EPSILON && self.data[2].abs() < EPSILON
    }
}

impl Vec3 {
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new([
            v1.y() * v2.z() - v1.z() * v2.y(),
            v1.z() * v2.x() - v1.x() * v2.z(),
            v1.x() * v2.y() - v1.y() * v2.x(),
        ])
    }
    pub fn random(rng: &mut ThreadRng) -> Vec3 {
        Vec3 { data: rng.gen() }
    }

    pub fn random_in_range<R>(rng: &mut ThreadRng, range: R) -> Vec3
    where
        R: SampleRange<f32> + Clone
    {
        Vec3 { data: [rng.gen_range(range.clone()), rng.gen_range(range.clone()), rng.gen_range(range)] }
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let v = Vec3::random_in_range(rng, -1.0..=1.0);
            if v.length_squared() < 1.0 { return v }
        }
    }

    pub fn random_on_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Vec3 {
        let v = Vec3::random_in_unit_sphere(rng).to_unit();
        if Vec3::dot(&v, normal) > 0.0 { v } else { v * -1 }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * Vec3::dot(v, n) * 2
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = Vec3::dot(&(*uv * -1.0), n).min(1.0);
        let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { data: [self.data[0] + rhs.data[0], self.data[1] + rhs.data[1], self.data[2] + rhs.data[2]] }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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

impl DivAssign<i32> for Vec3 {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new([0.0; 3])
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Vec3::default(), |a, b| a + b)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Vec3::new(value)
    }
}

impl Add<AABB> for Vec3 {
    type Output = AABB;
    fn add(self, rhs: AABB) -> Self::Output {
        AABB::new(self.x() + rhs.x, self.y() + rhs.y, self.z() + rhs.z)
    }
}