use std::hash::Hash;

use glium::{self, implement_vertex, Surface, Frame, uniform, uniforms::{UniformsStorage, EmptyUniforms}, Display, VertexBuffer, IndexBuffer};
use image;

use crate::matrix::*;
use super::{shaders, animation::Animation};

pub mod importing;

const FOV: f32 = std::f32::consts::PI / 3.0;
const ZFAR: f32 = 1024.0;
const ZNEAR: f32 = 0.1;

//const EPSILON: f32 = 1e-3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Vertex, position);
implement_vertex!(Normal, normal);

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
    positions : VertexBuffer<Vertex>,
    normals: VertexBuffer<Normal>,
    indices: IndexBuffer<u16>,

    transform: Transform,
    animation: Option<Box<dyn Animation>>,

    pub shader_type: shaders::ShaderType,
    bface_culling: glium::draw_parameters::BackfaceCullingMode,

}

impl Shape {
    pub fn new(positions: VertexBuffer<Vertex>, normals: VertexBuffer<Normal>, indices: IndexBuffer<u16>, 
            shader_type: shaders::ShaderType, transform: Option<Transform>, animation: Option<Box<dyn Animation>>, bface_culling: bool) -> Shape {

        let bface_culling = if bface_culling {
            glium::draw_parameters::BackfaceCullingMode::CullClockwise
        } else {
            glium::draw_parameters::BackfaceCullingMode::CullingDisabled
        };

        Shape { positions , normals, indices, transform: transform.unwrap_or_default(), animation, shader_type, bface_culling}
    }
}

impl Shape {
    pub fn animate(&mut self, t: f32) {
        if let Some(animation) = self.animation.as_mut() {
            animation.run(t, &mut self.transform);
        }
    }
    pub fn replace_animation(&mut self, animation: Box<dyn Animation>) {
        self.animation = Some(animation);
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


impl Shape {
    pub fn draw(&self, frame: &mut Frame, light: &[f32; 3], view: &Mat4, program: &glium::Program) {
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess, // When should we use pixel values? IfLess (than the value already there)
                write: true, // Write the new pixel depths to the depth buffer
                .. Default::default()
            },
            backface_culling: self.bface_culling,
            .. Default::default()
        };

        // perspective matrix        
        let perspective = {
            let (width, height) = frame.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let f = 1.0 / (FOV / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (ZFAR+ZNEAR)/(ZFAR-ZNEAR)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*ZFAR*ZNEAR)/(ZFAR-ZNEAR),   0.0],
            ]
        };

        let uniforms = uniform! {
                model: self.transform.transform_matrix.inner, view: view.inner, perspective: perspective, u_light: *light};

        frame.draw((&self.positions , &self.normals), &self.indices, program, &uniforms,
            &params).unwrap();
    }
}
