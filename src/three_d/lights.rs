//! A module with structs representing different kinds of lights.
use crate::matrix::Matrix;

/// A light that points in a specified direction.
#[derive(Clone, Copy)]
pub struct DirectionLight {
    /// The direction of the light.
    pub direction: [f32; 3],

    /// The ambient color of the light.
    pub ambient: [f32; 3],
    /// The diffuse color of the light.
    pub diffuse: [f32; 3],
    /// The specular color of the light.
    pub specular: [f32; 3],
} 

impl DirectionLight {
    /// Convert the light to a 4x4 matrix.
    pub fn as_matrix(&self) -> Matrix {
        [[self.direction[0], self.direction[1], self.direction[2], 0.0], 
        [self.ambient[0], self.ambient[1], self.ambient[2], 0.0],  
        [self.diffuse[0], self.diffuse[1], self.diffuse[2], 0.0], 
        [self.specular[0], self.specular[1], self.specular[2], 0.0]]
    }
}

/// A light that emanates from a specific point.
#[derive(Clone, Copy)]
pub struct PointLight {
    /// The position of the light.
    pub position: [f32; 3],

    /// The ambient color of the light.
    pub ambient: [f32; 3],
    /// The diffuse color of the light.
    pub diffuse: [f32; 3],
    /// The specular color of the light.
    pub specular: [f32; 3],

    /// The constant attenuation of the light.
    pub constant: f32,
    /// The linear attenuation of the light.
    pub linear: f32,
    /// The quadratic attenuation of the light.
    pub quadratic: f32,
} 

impl PointLight {
    /// Convert the light to a 4x4 matrix.
    pub fn as_matrix(&self) -> Matrix {
        [[self.position[0], self.position[1], self.position[2], 0.0], 
        [self.ambient[0], self.ambient[1], self.ambient[2], self.constant],  
        [self.diffuse[0], self.diffuse[1], self.diffuse[2], self.linear], 
        [self.specular[0], self.specular[1], self.specular[2], self.quadratic]]
    }
}