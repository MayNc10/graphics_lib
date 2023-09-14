use glium::{self, implement_vertex, Surface, Frame, uniform, uniforms::{UniformsStorage, EmptyUniforms}, Display};
use image;

use crate::matrix::*;
use super::triangulate::*;

// This is a vertex that defines its own color
#[derive(Copy, Clone)]
pub struct ColorVertex {
    pub position: [f32; 2],
    pub color: [f32; 3]
}

implement_vertex!(ColorVertex, position, color);

// This is a vertex that defines its own texture coordinates
#[derive(Copy, Clone)]
pub struct TextureVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2]
}

implement_vertex!(TextureVertex, position, tex_coords);

// Our general vertex container type
// Shape should be able to use either type of vertex
// TODO: Rename since this holds more than Vectex info now
enum VertexContainer {
    Color(glium::VertexBuffer<ColorVertex>),
    Texture(glium::VertexBuffer<TextureVertex>, glium::texture::SrgbTexture2d),
}

pub struct Shape {
    // These verticles are absolute for now, but we should make them relative to the Shape's position
    vertices: VertexContainer, // the vertices shouldn't change, so we use a Box<[]> for performance
    // TODO: Add rotation stuff
    transform_matrix: Mat4,
}

impl Shape {
    pub fn new_convex_color(poly_vertices: &[ColorVertex], display: &Display, transform: Option<&Mat4>) -> Shape {
        // We assume that the given shape is convex

        let shape = triangulate_simple_polygon_color(poly_vertices);

        Shape {vertices: VertexContainer::Color(glium::VertexBuffer::new(display, &shape).unwrap()), 
            transform_matrix: *transform.unwrap_or(&IDENTITY)}
    }

    pub fn new_convex_texture(poly_vertices: &[TextureVertex], display: &Display, 
            bytes: &[u8], format: image::ImageFormat, transform: Option<&Mat4>) -> Shape {
        // We assume that the given shape is convex
        let shape = triangulate_simple_polygon_texture(poly_vertices);

        let image = image::load(std::io::Cursor::new(bytes), 
            format).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        Shape {vertices: VertexContainer::Texture(glium::VertexBuffer::new(display, &shape).unwrap(), texture), 
            transform_matrix: *transform.unwrap_or(&IDENTITY)}
    }

    /// This is unsafe because the user has to guarantee that the shape is actually convex
    pub unsafe fn new_convex_unchecked_color(poly_vertices: &[ColorVertex], display: &Display, transform: Option<&Mat4>) -> Shape {
        // We assume that the given shape is convex

        let shape = triangulate_convex_color(poly_vertices);

        Shape {vertices: VertexContainer::Color(glium::VertexBuffer::new(display, &shape).unwrap()), 
            transform_matrix: *transform.unwrap_or(&IDENTITY)}
    }

    /// This is unsafe because the user has to guarantee that the shape is actually convex
    pub unsafe fn new_convex_unchecked_texture(poly_vertices: &[TextureVertex], display: &Display, 
            bytes: &[u8], format: image::ImageFormat, transform: Option<&Mat4>) -> Shape {
        // We assume that the given shape is convex
        let shape = triangulate_convex_texture(poly_vertices);

        let image = image::load(std::io::Cursor::new(bytes), 
            format).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        Shape {vertices: VertexContainer::Texture(glium::VertexBuffer::new(display, &shape).unwrap(), texture), 
            transform_matrix: *transform.unwrap_or(&IDENTITY)}
    }
}

impl Shape {
    pub fn set_transform_matrix(&mut self, mat: Mat4) {
        self.transform_matrix = mat;
    }
    pub fn get_transform(&mut self) -> &mut Mat4 {
        &mut self.transform_matrix
    }
}

// Create an OpenGL vextex shader for a color vertex
// This shader takes a position, a matrix, and outputs an attribute
// The attribute is set to the position, and will be used later
// In the code, a special variable called "gl_Position" (which I assume is the position of the vertex), is set to a vec4
// A vec4 is x, y, z, w
// x and y are position
// z is depth and set to 0 bc we are just doing two
// TODO: What is w used for? some scaling thing
const DEFAULT_COLOR_2D_SHADER: &str = r#"
    #version 140

    in vec2 position;
    in vec3 color;
    out vec3 vertex_color;

    uniform mat4 matrix;

    void main() {
        vertex_color = color;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    } 
"#;

// Create an OpenGL fragment shader for a color vertex
// This determines the color (and other aspects) of a fragment (which is bacially just a pixel)
const DEFAULT_COLOR_2D_FRAG_SHADER: &str = r#"
    #version 140

    in vec3 vertex_color;
    out vec4 color;

    void main() {
        color = vec4(vertex_color, 1.0);  
    }
"#;

// Create the vertex and fragment shaders but for a texture
const DEFAULT_TEXTURE_2D_SHADER: &str = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;


    uniform mat4 matrix;

    void main() {
        v_tex_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    } 
"#;

const DEFAULT_TEXTURE_2D_FRAG_SHADER: &str = r#"
    #version 140

    in vec2 v_tex_coords;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texture(tex, v_tex_coords);
    }
"#;

impl Shape {
    pub fn draw(&self, frame: &mut Frame, display: &Display) {

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        match &self.vertices {
            VertexContainer::Color(ref vb) => {
                // Create a glium program out of the default color shaders
                let program = glium::Program::
                from_source(display, DEFAULT_COLOR_2D_SHADER, 
                        DEFAULT_COLOR_2D_FRAG_SHADER, None).unwrap();

                let uniforms = uniform! {matrix: self.transform_matrix.inner};

                frame.draw(vb, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
            },
            VertexContainer::Texture(ref vb, ref texture) => {
                // Create a glium program out of the default texture shaders
                let program = glium::Program::
                from_source(display, DEFAULT_TEXTURE_2D_SHADER, 
                        DEFAULT_TEXTURE_2D_FRAG_SHADER, None).unwrap();

                let uniforms = uniform! {matrix: self.transform_matrix.inner, tex: texture};

                frame.draw(vb, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
            }
        }
    }
}

