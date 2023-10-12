use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::window;
use gl::types::*;

use super::shaders::Program;
use super::shape::Light;
use super::shape::Shape;
use crate::matrix::Mat4;

use std::ffi::CString;

type ObjectCollection = (Vec<Shape>, &'static Program);

pub struct Scene {
    objects: Vec<ObjectCollection>,
    view: Mat4,
    light: Light,
}

impl Scene {
    pub fn new(view: Mat4, light: Light) -> Scene { 
        let objects = Vec::new();

        Scene { objects, view, light }
    }

    /// Returns the index of the vector where the shape is stored, as well as the index in that vector
    pub fn add_shape(&mut self, shape: Shape, program: &'static Program) -> (usize, usize) {
        let mut i = 0; 
        for (v, p) in &mut self.objects {
            if *p == program {
                v.push(shape);
                return (i, v.len() - 1);
            }
            i += 1;
        } 
        let v = Vec::from([shape]);
        self.objects.push((v, program));
        return (i, 0);
    }
}

type Window = ContextWrapper<PossiblyCurrent, window::Window>;

impl Scene {
    pub fn draw(&mut self, t: f32, dims: (f32, f32), gl_window: &Window) {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for object_collection in &mut self.objects {
            for shape in &mut object_collection.0 {
                shape.animate(t);
                shape.draw(&self.light, &self.view, &object_collection.1, dims); 
            }
        }

        gl_window.swap_buffers().unwrap();
    }

    pub fn draw_deferred(&mut self, t: f32, dims: (f32, f32), gl_window: &Window, prepass_prog: &Program, lighting_prog: &Program, 
        quad_vao: u32, g_buffer: u32, g_position: u32, g_normal: u32, g_color_diffuse: u32, g_color_emission: u32, g_color_specular: u32, 
        point_lighting_prog: &Program, point_lights: &[[[GLfloat; 4]; 4]]) 
    {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, g_buffer);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            for collection in &mut self.objects {
                for shape in &mut collection.0 {
                    // Our prepass shader has all the same uniforms as the normal blinn-phong
                    // And we've bound the framebuffer
                    // So we should be good to just call 'draw'
                    shape.animate(t);
                    shape.draw(&self.light, &self.view, prepass_prog, dims);
                }
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(lighting_prog.0);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, g_position);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, g_normal);
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, g_color_diffuse);
            gl::ActiveTexture(gl::TEXTURE3);
            gl::BindTexture(gl::TEXTURE_2D, g_color_emission);
            gl::ActiveTexture(gl::TEXTURE4);
            gl::BindTexture(gl::TEXTURE_2D, g_color_specular);

            let g_position_handle_name = CString::new("gPosition").unwrap();
            let g_normal_handle_name = CString::new("gNormal").unwrap();
            let g_color_diffuse_handle_name = CString::new("gColorDiffuse").unwrap();
            let g_color_emission_handle_name = CString::new("gColorEmission").unwrap();
            let g_color_specular_handle_name = CString::new("gColorSpecular").unwrap();
            let light_handle_name = CString::new("light").unwrap();

            let g_position_handle = gl::GetUniformLocation(lighting_prog.0, g_position_handle_name.as_ptr());
            let g_normal_handle = gl::GetUniformLocation(lighting_prog.0, g_normal_handle_name.as_ptr());
            let g_color_diffuse_handle = gl::GetUniformLocation(lighting_prog.0, g_color_diffuse_handle_name.as_ptr());
            let g_color_emission_handle = gl::GetUniformLocation(lighting_prog.0, g_color_emission_handle_name.as_ptr());
            let g_color_specular_handle = gl::GetUniformLocation(lighting_prog.0, g_color_specular_handle_name.as_ptr());
            let light_handle = gl::GetUniformLocation(lighting_prog.0, light_handle_name.as_ptr());

            gl::Uniform1i(g_position_handle, 0);
            gl::Uniform1i(g_normal_handle, 1);
            gl::Uniform1i(g_color_diffuse_handle, 2);
            gl::Uniform1i(g_color_emission_handle, 3);
            gl::Uniform1i(g_color_specular_handle, 4);

            let light_mat = self.light.as_matrix();
            gl::UniformMatrix4fv(light_handle, 1, gl::FALSE, &light_mat[0] as *const GLfloat);
            
            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendColor(0.0, 0.0, 0.0, 0.5);
            gl::BlendFunc(gl::CONSTANT_ALPHA, gl::ONE_MINUS_CONSTANT_ALPHA);
            gl::DepthFunc(gl::LEQUAL);

            gl::BindVertexArray(quad_vao);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

            gl::BlendFunc(gl::ONE, gl::ONE);

            //gl::UseProgram(point_lighting_prog.0);
            let light_handle = gl::GetUniformLocation(lighting_prog.0, light_handle_name.as_ptr());

            for light in point_lights {
                gl::UniformMatrix4fv(light_handle, 1, gl::FALSE, &light[0] as *const GLfloat);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
            
            gl::BindVertexArray(0);

            gl::Disable(gl::BLEND);
            gl::DepthFunc(gl::LESS);

            gl::UseProgram(0);
        }

        gl_window.swap_buffers().unwrap();
    }
}
