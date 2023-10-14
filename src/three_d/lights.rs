use crate::matrix::Matrix;

#[derive(Clone, Copy)]
pub struct DirectionLight {
    pub direction: [f32; 3],

    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
} 

impl DirectionLight {
    pub fn as_matrix(&self) -> Matrix {
        [[self.direction[0], self.direction[1], self.direction[2], 0.0], 
        [self.ambient[0], self.ambient[1], self.ambient[2], 0.0],  
        [self.diffuse[0], self.diffuse[1], self.diffuse[2], 0.0], 
        [self.specular[0], self.specular[1], self.specular[2], 0.0]]
    }
}

#[derive(Clone, Copy)]
pub struct PointLight {
    pub position: [f32; 3],

    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
} 

impl PointLight {
    pub fn as_matrix(&self) -> Matrix {
        [[self.position[0], self.position[1], self.position[2], 0.0], 
        [self.ambient[0], self.ambient[1], self.ambient[2], self.constant],  
        [self.diffuse[0], self.diffuse[1], self.diffuse[2], self.linear], 
        [self.specular[0], self.specular[1], self.specular[2], self.quadratic]]
    }
}