use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use image;

use crate::matrix::*;
use super::{shaders, animation::Animation, buffer::*, VAO::*};

pub mod importing;
use importing::*;

pub const FOV: f32 = std::f32::consts::PI / 3.0;
pub const ZFAR: f32 = 1024.0;
pub const ZNEAR: f32 = 0.1;

//const EPSILON: f32 = 1e-3;

#[derive(Clone, Copy)]
pub struct Transform {
    pub transform_matrix: Mat4,
    pub rotation_matrix: Mat4,
    pub scaling_matrix: Mat4,
    pub translation_matrix: Mat4,
}

impl Transform {
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

    pub fn set_scaling(&mut self, scaling: Mat4) {
        self.scaling_matrix = scaling;

        self.transform_matrix = self.scaling_matrix * self.rotation_matrix * self.translation_matrix;
    }

    pub fn set_rotation(&mut self, rotation: Mat4) {
        self.rotation_matrix = rotation;

        self.transform_matrix = self.scaling_matrix * self.rotation_matrix * self.translation_matrix;
    }

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

pub struct Shape {
    //vao: VertexArrayObject,

    pub positions: VertexBuffer,
    pub normals: NormalBuffer,
    pub indices: IndexBuffer,

    pub transform: Transform,
    animation: Option<Box<dyn Animation>>,

    pub shader_type: shaders::ShaderType,
    //bface_culling: glium::draw_parameters::BackfaceCullingMode,

    material: Material,

}

/* 
impl Shape {
    pub fn new(positions: VertexBuffer<Vertex>, normals: VertexBuffer<Normal>, indices: IndexBuffer<u16>, 
            shader_type: shaders::ShaderType, transform: Option<Transform>, animation: Option<Box<dyn Animation>>, bface_culling: bool, 
            material: Material) -> Shape {

        let bface_culling = if bface_culling {
            glium::draw_parameters::BackfaceCullingMode::CullClockwise
        } else {
            glium::draw_parameters::BackfaceCullingMode::CullingDisabled
        };

        Shape { positions , normals, indices, transform: transform.unwrap_or_default(), animation, shader_type, bface_culling, material}
    }
}
*/

impl Shape {
    pub fn animate(&mut self, t: f32) {
        if let Some(animation) = self.animation.as_mut() {
            animation.run(t, &mut self.transform);
        }
    }
    pub fn replace_animation(&mut self, animation: Box<dyn Animation>) {
        self.animation = Some(animation);
    }
    pub fn set_material(&mut self, mat: Material) {
        self.material = mat;
    }
}

impl Shape {
    pub fn set_transform_matrix(&mut self, scaling: Option<Mat4>, rotation: Option<Mat4>, translation: Option<Mat4> ) {
        self.transform.set_transform_matrix(scaling, rotation, translation);
    }

    pub fn set_scaling(&mut self, scaling: Mat4) {
        self.transform.set_scaling(scaling)
    }

    pub fn set_rotation(&mut self, rotation: Mat4) {
        self.transform.set_rotation(rotation)
    }

    pub fn set_translation(&mut self, translation: Mat4) {
        self.transform.set_translation(translation)
    }
}

pub struct Light {
    pub direction: [f32; 3],

    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
} 

impl Light {
    pub fn as_matrix(&self) -> Matrix {
        [[self.direction[0], self.direction[1], self.direction[2], 0.0], 
        [self.ambient[0], self.ambient[1], self.ambient[2], 0.0],  
        [self.diffuse[0], self.diffuse[1], self.diffuse[2], 0.0], 
        [self.specular[0], self.specular[1], self.specular[2], 0.0]]
    }
}

impl Shape {
    pub fn draw(&self, light: &Light, view: &Mat4, program: &shaders::Program, dims: (f32, f32)) {
        // perspective matrix        
        let perspective = {
            let (width, height) = dims;
            let aspect_ratio = height as f32 / width as f32;

            let f = 1.0 / (FOV / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (ZFAR+ZNEAR)/(ZFAR-ZNEAR)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*ZFAR*ZNEAR)/(ZFAR-ZNEAR),   0.0],
            ]
        };
        unsafe {
            // Clear the screen
            gl::ClearColor(0.0, 0.6, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            /* 
            // Bind program
            gl::UseProgram(program.0); 
            gl::BindFragDataLocation(program.0, 0, CString::new("color").unwrap().as_ptr());

            // Specify the layout of the vertex data
            let pos_attr = gl::GetAttribLocation(program.0, CString::new("position").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                mem::size_of::<Vertex>() as GLint,
                ptr::null()
            ); */
            

            // FIXME: Assumes just using no shader
            // Get uniform handles
            //let perspective_handle = gl::GetUniformLocation(program.0, CString::new("perspective").unwrap().as_ptr());
            //let view_handle = gl::GetUniformLocation(program.0, CString::new("view").unwrap().as_ptr());
            //let model_handle = gl::GetUniformLocation(program.0, CString::new("model").unwrap().as_ptr());

            // Bind matrix data to uniforms
            //gl::UniformMatrix4fv(perspective_handle, 1, gl::FALSE, perspective.as_ptr() as *const GLfloat);
            //gl::UniformMatrix4fv(view_handle, 1, gl::FALSE, view.inner.as_ptr() as *const GLfloat);
            //gl::UniformMatrix4fv(model_handle, 1, gl::FALSE, 
            //    self.transform.transform_matrix.inner.as_ptr() as *const GLfloat);
            
            // Make sure we have the right VAO loaded
            //gl::BindVertexArray(*self.vao.id());

            //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *self.indices.get_id());

            gl::DrawElements(
                gl::TRIANGLES,      
                self.indices.num_indices as i32,    
                gl::UNSIGNED_INT,   
                ptr::null()     
            );

            // FIXME: This should happen as the drop function of some struct (from where we bind the VAO)
            //gl::BindVertexArray(0);

            //gl::DisableVertexAttribArray(pos_attr as GLuint);
        }
        

        
    }
}

/*if self.shader_type != shaders::ShaderType::BlinnPhong {
            let uniforms = uniform! {
                model: self.transform.transform_matrix.inner, view: view.inner, perspective: perspective, u_light: light.direction};

            frame.draw((&self.positions , &self.normals), &self.indices, program, &uniforms,
            &params).unwrap();
        } else {
            let uniforms = uniform! {
                model: self.transform.transform_matrix.inner, view: view.inner, perspective: perspective, u_light: light.as_matrix(), 
                    ambient_color: self.material.ambient_color, 
                    diffuse_color: self.material.diffuse_color, 
                    emission_color: self.material.emission_color,
                    specular_color: self.material.specular_color, 
                    specular_exp: self.material.specular_exp};

            frame.draw((&self.positions , &self.normals), &self.indices, program, &uniforms,
            &params).unwrap();
        } */