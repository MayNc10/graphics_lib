use std::ops::{Mul, MulAssign};

pub type Matrix = [[f32; 4]; 4];

#[derive(Clone, Copy)]
pub struct Mat4 {
    pub inner: Matrix,
}

impl Mat4 {
    pub fn new(m: Matrix) -> Mat4 {
        Mat4 { inner: m }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let calc_entry = |i: usize, j: usize| {
            let mut res = 0.0;
            for n in 0..4 {
                res += self.inner[i][n] * other.inner[n][j];
            }
            res
        };

        let mut m = [
            [0.0, 0.0, 0.0, 0.0], 
            [0.0, 0.0, 0.0, 0.0], 
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0f32],
        ];
        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = calc_entry(i, j);
            }
        }
        Self { inner: m }
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// Define an identity matrix
pub const IDENTITY: Mat4 = Mat4 { inner: [
    [1.0, 0.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0, 0.0], 
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0f32],
]}; 

pub fn generate_scale(scaling: &[f32; 3]) -> Mat4 {
    Mat4 { inner: [
        [scaling[0], 0.0, 0.0, 0.0], 
        [0.0, scaling[1], 0.0, 0.0], 
        [0.0, 0.0, scaling[2], 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]}
}

pub fn generate_rotate_x(angle: f32) -> Mat4 {
    Mat4 { inner: [
        [1.0, 0.0, 0.0, 0.0], 
        [0.0, angle.cos(), angle.sin(), 0.0], 
        [0.0, -angle.sin(), angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]}
}

pub fn generate_rotate_y(angle: f32) -> Mat4 {
    Mat4 { inner: [
        [angle.cos(), 0.0, angle.sin(), 0.0], 
        [0.0, 1.0, 0.0, 0.0], 
        [-angle.sin(), 0.0, angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]}
}

pub fn generate_rotate_z(angle: f32) -> Mat4 {
    Mat4 { inner: [
        [angle.cos(), angle.sin(), 0.0, 0.0], 
        [-angle.sin(), angle.cos(), 0.0, 0.0], 
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]}
}

pub fn generate_translate(x_offset: Option<f32>, y_offset: Option<f32>, z_offset: Option<f32>) -> Mat4 {
    Mat4 { inner: [
        [1.0, 0.0, 0.0, 0.0], 
        [0.0, 1.0, 0.0, 0.0], 
        [0.0, 0.0, 1.0, 0.0],
        [x_offset.unwrap_or(0.0), y_offset.unwrap_or(0.0), z_offset.unwrap_or(0.0), 1.0f32],
    ]}
}


pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> Mat4 {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    Mat4 { inner: [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ] }
}

