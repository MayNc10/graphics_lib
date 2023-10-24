//! A shape with given vertex coordinates and normal vectors.
use gl::types::*;
use std::ffi::CString;
use std::ptr;

use crate::matrix::*;
use super::shaders::Program;
use super::shaders::ShaderType;
use super::{animation::Animation, buffer::*, vao::*, lights::DirectionLight};

pub mod importing;
use importing::*;

/// Defines the camera FOV.
pub const FOV: f32 = std::f32::consts::PI / 3.0;
/// Defines the far plane of the camera.
pub const ZFAR: f32 = -1024.0;
/// Defines the near plane of the camera.
pub const ZNEAR: f32 = -0.1;

//const EPSILON: f32 = 1e-3;

/// Represents a transformation, made up of a rotation matrix, a scaling matrix, and a translation matrix
/// FIXME: Should account for multiple of each type of matrix.
#[derive(Clone, Copy)]
pub struct Transform {
    /// The full transformation matrix of this 3D transform.
    pub transform_matrix: Mat4,
    /// The rotation matrix for the transform.
    pub rotation_matrix: Mat4,
    /// The scaling matrix for the transform.
    pub scaling_matrix: Mat4,
    /// The translation matrix for the transform.
    pub translation_matrix: Mat4,
}

impl Transform {
    /// Set the transform matrix.
    pub fn set_transform_matrix(&mut self, scaling: Option<Mat4>, rotation: Option<Mat4>, translation: Option<Mat4> ) {
        if rotation.is_some() {
            self.rotation_matrix = rotation.unwrap();
        }
        if scaling.is_some() {
            self.scaling_matrix = scaling.unwrap();
        }
        if translation.is_some() {
            self.translation_matrix = translation.unwrap();
        }

        self.transform_matrix = self.scaling_matrix * self.rotation_matrix * self.translation_matrix;
    }

    /// Set the scaling matrix.
    pub fn set_scaling(&mut self, scaling: Mat4) {
        self.scaling_matrix = scaling;

        self.transform_matrix = self.scaling_matrix * self.rotation_matrix * self.translation_matrix;
    }

    /// Set the rotation matrix.
    pub fn set_rotation(&mut self, rotation: Mat4) {
        self.rotation_matrix = rotation;

        self.transform_matrix = self.scaling_matrix * self.rotation_matrix * self.translation_matrix;
    }

    /// Set the translation matrix.
    pub fn set_translation(&mut self, translation: Mat4) {
        self.translation_matrix = translation;

        self.transform_matrix = self.scaling_matrix * self.rotation_matrix * self.translation_matrix;
    }
    
}

impl Default for Transform {
    fn default() -> Self {
        Transform { 
            transform_matrix: IDENTITY, 
            rotation_matrix: IDENTITY, 
            scaling_matrix: IDENTITY, 
            translation_matrix: IDENTITY 
        }
    }
}

/// A shape with given vertex coordinates and normal vectors.
pub struct Shape {
    vao: VertexArrayObject,

    positions: VertexBuffer,
    normals: NormalBuffer,
    indices: IndexBuffer,

    transform: Transform,
    animation: Option<Box<dyn Animation>>,

    shader_type: ShaderType,

    material: Material,
}

impl Shape {
    /// Bind the position and normal attributes of this shape to the given program.
    pub fn bind_attributes(&self, program: &Program) {
        let _vao_lock = self.vao.bind().unwrap().into_inner();

        let pos_name = CString::new("position").unwrap();
        let norm_name = CString::new("normal").unwrap();

        self.positions.bind_attributes(program, &pos_name);
        self.normals.bind_attributes(program, &norm_name);
    }
}

impl Shape {
    /// Run one step of animation, which updates the shape's 3D transformation based on the shape's animation.
    pub fn animate(&mut self, t: f32) {
        if let Some(animation) = self.animation.as_mut() {
            animation.run(t, &mut self.transform);
        }
    }
    /// Swap out the current animation for a different one.
    pub fn replace_animation(&mut self, animation: Box<dyn Animation>) {
        self.animation = Some(animation);
    }
    /// Sets the shape's material.
    pub fn set_material(&mut self, mat: Material) {
        self.material = mat;
    }
}

impl Shape {
    /// Sets the shape's transformation matrix.
    pub fn set_transform_matrix(&mut self, scaling: Option<Mat4>, rotation: Option<Mat4>, translation: Option<Mat4> ) {
        self.transform.set_transform_matrix(scaling, rotation, translation);
    }

    /// Sets the shape's scaling matrix.
    pub fn set_scaling(&mut self, scaling: Mat4) {
        self.transform.set_scaling(scaling)
    }

    /// Sets the shape's rotation matrix.
    pub fn set_rotation(&mut self, rotation: Mat4) {
        self.transform.set_rotation(rotation)
    }

    /// Sets the shape's translation matrix.
    pub fn set_translation(&mut self, translation: Mat4) {
        self.transform.set_translation(translation)
    }
}

impl Shape {
    /// Draws the shape to the set framebuffer, given the light source, the view matrix, the program to use, and the screen dimensions.
    pub fn draw(&self, light: &DirectionLight, view: &Mat4, program: &Program, dims: (f32, f32)) {
        // perspective matrix        
        let perspective = {
            let (width, height) = dims;
            let aspect_ratio = height / width;

            let f = 1.0 / (FOV / 2.0).tan();

            
            [
                [f  *  aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  -(ZFAR+ZNEAR)/(ZFAR-ZNEAR)    ,   -1.0],
                [         0.0         ,    0.0, (2.0*ZFAR*ZNEAR)/(ZFAR-ZNEAR),   0.0],
            ]
            
        };
        unsafe {
            let _vao_lock = self.vao.bind().unwrap();
            let perspective_name = CString::new("perspective").unwrap();
            let view_name = CString::new("view").unwrap();
            let model_name = CString::new("model").unwrap();
            
            let perspective_handle = gl::GetUniformLocation(program.0, perspective_name.as_ptr());
            let view_handle = gl::GetUniformLocation(program.0, view_name.as_ptr());
            let model_handle = gl::GetUniformLocation(program.0, model_name.as_ptr());
            // Bind matrix data to uniforms
            gl::UniformMatrix4fv(perspective_handle, 1, gl::FALSE, perspective.as_ptr() as *const GLfloat);
            gl::UniformMatrix4fv(view_handle, 1, gl::FALSE, view.inner.as_ptr() as *const GLfloat);
            gl::UniformMatrix4fv(model_handle, 1, gl::FALSE, 
                self.transform.transform_matrix.inner.as_ptr() as *const GLfloat);

            let light_name = CString::new("u_light").unwrap();

            if self.shader_type == ShaderType::Gouraud {
                let light_handle = gl::GetUniformLocation(program.0, light_name.as_ptr());
                let light = light.direction;
                gl::Uniform3fv(light_handle, 1, light.as_ptr() as *const GLfloat);

            }
            else if self.shader_type == ShaderType::BlinnPhong {
                let light_handle = gl::GetUniformLocation(program.0, light_name.as_ptr());
                gl::UniformMatrix4fv(light_handle, 1, gl::FALSE, light.as_matrix().as_ptr() as *const GLfloat);
                
                let ambient_color_name = CString::new("ambient_color").unwrap();
                let diffuse_color_name = CString::new("diffuse_color").unwrap();
                let emission_color_name = CString::new("emission_color").unwrap();
                let specular_color_name = CString::new("specular_color").unwrap();
                let specular_exp_name = CString::new("specular_exp").unwrap();

                // Add material uniforms
                let ambient_color_handle = gl::GetUniformLocation(program.0, ambient_color_name.as_ptr());
                let diffuse_color_handle = gl::GetUniformLocation(program.0, diffuse_color_name.as_ptr());
                let emission_color_handle = gl::GetUniformLocation(program.0, emission_color_name.as_ptr());
                let specular_color_handle = gl::GetUniformLocation(program.0, specular_color_name.as_ptr());
                let specular_exp_handle = gl::GetUniformLocation(program.0, specular_exp_name.as_ptr());

                gl::Uniform3fv(ambient_color_handle, 1, self.material.ambient_color.as_ptr() as *const GLfloat);
                gl::Uniform3fv(diffuse_color_handle, 1, self.material.diffuse_color.as_ptr() as *const GLfloat);
                gl::Uniform3fv(emission_color_handle, 1, self.material.emission_color.as_ptr() as *const GLfloat);
                gl::Uniform3fv(specular_color_handle, 1, self.material.specular_color.as_ptr() as *const GLfloat);

                gl::Uniform1f(specular_exp_handle, self.material.specular_exp);
            }

            gl::DrawElements(
                gl::TRIANGLES,      
                self.indices.num_indices as GLint,    
                gl::UNSIGNED_INT,   
                ptr::null()           
            );

            gl::BindVertexArray(0);
        }
        
    }
}