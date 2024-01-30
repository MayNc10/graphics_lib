//! A Scene represents a set of lights, shapes, and a specific rendering program.

const UP_INCREMENT: f32 = 1.0;
const DOWN_INCREMENT: f32 = 1.0;
const LEFT_INCREMENT: f32 = 1.0;
const RIGHT_INCREMENT: f32 = 1.0;
const FORWARD_INCREMENT: f32 = 1.0;
const BACKWARD_INCREMENT: f32 = 1.0;
const LOOK_UP_INCREMENT: f32 = 1.0;
const LOOK_DOWN_INCREMENT: f32 = 1.0;
const LOOK_LEFT_INCREMENT: f32 = 1.0;
const LOOK_RIGHT_INCREMENT: f32 = 1.0;

use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::window;
use gl::types::*;

use super::buffer::FrameBuffer;
use super::buffer::VertexBuffer;
use super::shaders::Program;
use super::lights::{DirectionLight, PointLight};
use super::shape::Shape;
use super::vao::VertexArrayObject;
use crate::matrix::{Mat4, view_matrix};

use once_cell::sync::OnceCell;
use std::ffi::CString;

thread_local! {
    static DEFERRED_QUAD_VO: OnceCell<(VertexArrayObject, VertexBuffer)> = OnceCell::new();
}

/// If using deferred rendering, this function must be called before rendering.
pub fn init_deferred_quad() -> bool {
    return DEFERRED_QUAD_VO.with(|quad_vo | {
        if quad_vo.get().is_some() {
            return false;
        }
        else {
            let quad_vertices = [
                    // positions        // texture Coords
                    [-1.0,  1.0, 0.0],
                    [-1.0, -1.0, 0.0],
                    [1.0,  1.0, 0.0], 
                    [1.0, -1.0, 0.0_f32],
            ];
            let (quad_vao, vao_lock) = VertexArrayObject::new().unwrap().into_inner();
            let quad_vbo = VertexBuffer::new(&quad_vertices, &vao_lock);
            quad_vbo.bind_attributes_index(0);
            quad_vo.set((quad_vao, quad_vbo)).expect("Deferred Quad was somehow already set");
        }
        true
    });
}

/// A Scene represents a set of lights, shapes, and a specific rendering program.
pub struct Scene {
    shapes: Vec<Shape>,
    program: &'static Program,
    position: [f32; 3],
    direction: [f32; 3],
    view: Mat4,
    direction_lights: Vec<DirectionLight>,
    point_lights: Vec<PointLight>,
}

impl Scene {
    /// Create a new scene given a view matrix, a program to use, a set of direction lights, and a set of point lights.
    pub fn new(position: [f32; 3], program: &'static Program, direction_lights: Vec<DirectionLight>, point_lights: Vec<PointLight>, direction: [f32; 3], up: &[f32; 3]) -> Scene {
        let shapes = Vec::new();

        Scene { shapes, program, position, direction, view: view_matrix(&position, &direction, up), direction_lights, point_lights }
    }

    /// Returns the index of the shape vector where the shape is stored.
    // FIXME: Make this actually usable
    pub fn add_shape(&mut self, shape: Shape) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }
}

impl Scene {
    pub fn move_up(&mut self, dt: f32, up: &[f32; 3]) {
        self.position[1] += UP_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn move_down(&mut self, dt: f32, up: &[f32; 3]) {
        self.position[1] -= DOWN_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn move_right(&mut self, dt: f32, up: &[f32; 3]) {
        self.position[0] += RIGHT_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn move_left(&mut self, dt: f32, up: &[f32; 3]) {
        self.position[0] -= LEFT_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn move_forward(&mut self, dt: f32, up: &[f32; 3]) {
        self.position[2] -= FORWARD_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn move_backward(&mut self, dt: f32, up: &[f32; 3]) {
        self.position[2] += BACKWARD_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }

    pub fn look_up(&mut self, dt: f32, up: &[f32; 3]) {
        self.direction[1] += LOOK_UP_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn look_down(&mut self, dt: f32, up: &[f32; 3]) {
        self.direction[1] -= LOOK_DOWN_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn look_right(&mut self, dt: f32, up: &[f32; 3]) {
        self.direction[0] -= LOOK_RIGHT_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn look_left(&mut self, dt: f32, up: &[f32; 3]) {
        self.direction[0] += LOOK_LEFT_INCREMENT * dt;
        self.view = view_matrix(&self.position, &self.direction, up);
    }
    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

type Window = ContextWrapper<PossiblyCurrent, window::Window>;

impl Scene {
    /// Render the scene to the screen, given the window, the window dimensions, and the current time.
    ///
    /// Note: This function only applied the first direction light and no point lights.
    pub fn draw(&mut self, t: f32, dims: (f32, f32), gl_window: &Window) {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for shape in &mut self.shapes {
            shape.animate(t);
            shape.draw(&self.direction_lights[0], &self.view, self.program, dims);
        }


        gl_window.swap_buffers().unwrap();
    }

    /// Draw using deferred rendering.
    /// Deferred rendering allows for the use of an arbitrary number of lights.
    ///
    /// The `prepass_prog` 
    pub fn draw_deferred(&mut self, t: f32, dims: (f32, f32), gl_window: &Window, prepass_prog: &Program, lighting_prog: &Program,
        point_lighting_prog: &Program, emission_prog: &Program, frame_buffer: &FrameBuffer)
    {
        DEFERRED_QUAD_VO.with(|quad_vo | { 
            let quad = quad_vo.get().expect("Deffered Quad was not initialized");
            let quad_vao = &quad.0;
            unsafe {
                prepass_prog.use_progam();
    
                frame_buffer.bind();
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::ClearColor(0.0, 0.0, 0.0, 0.0);

                for shape in &mut self.shapes {
                    // Our prepass shader has all the same uniforms as the normal blinn-phong
                    // And we've bound the framebuffer
                    // So we should be good to just call 'draw'
                    shape.animate(t);
                    // The direction light is just a filler value, our program doesn't use it
                    shape.draw(&self.direction_lights[0], &self.view, prepass_prog, dims);
                }

                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);

                lighting_prog.use_progam();
                frame_buffer.bind_textures();
    
                let g_position_handle_name = CString::new("gPosition").unwrap();
                let g_normal_handle_name = CString::new("gNormal").unwrap();
                let g_color_diffuse_handle_name = CString::new("gColorDiffuse").unwrap();
                let g_color_emission_handle_name = CString::new("gColorEmission").unwrap();
                let g_color_specular_handle_name = CString::new("gColorSpecular").unwrap();
    
                let fb_names = [g_position_handle_name, g_normal_handle_name, g_color_diffuse_handle_name, 
                    g_color_emission_handle_name, g_color_specular_handle_name];

                emission_prog.use_progam();
                frame_buffer.add_uniforms(&fb_names, &emission_prog);
    
                let _vao_lock = quad_vao.bind().unwrap();

                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
    
                // Only enable blending first direction light calculation bc we don't want to blend with the background
                gl::Enable(gl::BLEND);
                gl::BlendEquation(gl::FUNC_ADD);
                gl::BlendFunc(gl::ONE, gl::ONE);
                gl::DepthFunc(gl::LEQUAL);

                let light_handle_name = CString::new("light").unwrap();

                lighting_prog.use_progam();
                frame_buffer.add_uniforms(&fb_names, &lighting_prog);

                let light_handle = gl::GetUniformLocation(lighting_prog.0, light_handle_name.as_ptr());

                for light in &self.direction_lights {
                    gl::UniformMatrix4fv(light_handle, 1, gl::FALSE, &light.as_matrix()[0] as *const GLfloat);
                    gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
                }

                point_lighting_prog.use_progam();
                frame_buffer.add_uniforms(&fb_names, &point_lighting_prog);
    
                let light_handle = gl::GetUniformLocation(point_lighting_prog.0, light_handle_name.as_ptr());
    
                for light in &self.point_lights {
                    gl::UniformMatrix4fv(light_handle, 1, gl::FALSE, &light.as_matrix()[0] as *const GLfloat);
                    gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
                }
                
                gl::BindVertexArray(0);
    
                gl::Disable(gl::BLEND);
                gl::DepthFunc(gl::LESS);
    
                gl::UseProgram(0);
            }
        });

        gl_window.swap_buffers().unwrap();
    }
}
