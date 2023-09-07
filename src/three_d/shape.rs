use glium::{self, implement_vertex, Surface, Frame, uniform, uniforms::{UniformsStorage, EmptyUniforms}, Display, VertexBuffer, IndexBuffer};
use image;

use crate::matrix::*;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Vertex, position);
implement_vertex!(Normal, normal);

pub struct Shape {
    positions : VertexBuffer<Vertex>,
    normals: VertexBuffer<Normal>,
    indices: IndexBuffer<u16>,

    transform_matrix: Matrix,
}

impl Shape {
    pub fn new(positions: VertexBuffer<Vertex>, normals: VertexBuffer<Normal>, indices: IndexBuffer<u16>, 
        transform: Option<Matrix>) -> Shape {
        Shape { positions , normals, indices, transform_matrix: transform.unwrap_or(IDENTITY)}
    }
}

impl Shape {
    pub fn set_transform_matrix(&mut self, mat: Matrix) {
        self.transform_matrix = mat;
    }
    pub fn get_transform(&mut self) -> &mut Matrix {
        &mut self.transform_matrix
    }
}

// Create an OpenGL vextex shader for a vertex
const DEFAULT_3D_SHADER: &str = r#"
    #version 140

    in vec3 position;
    in vec3 normal;

    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(position, 1.0);
    }
"#;

// Create an OpenGL fragment shader for a color vertex
// This determines the color (and other aspects) of a fragment (which is bacially just a pixel)
const DEFAULT_3D_FRAG_SHADER: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

impl Shape {
    pub fn draw(&self, frame: &mut Frame, display: &Display) {
        let program = glium::Program::from_source(
            display, DEFAULT_3D_SHADER, 
            DEFAULT_3D_FRAG_SHADER, None).unwrap(); 

        let uniforms = uniform! {matrix: self.transform_matrix};

        frame.draw((&self.positions , &self.normals), &self.indices, &program, &uniforms,
            &Default::default()).unwrap();
    }
}
