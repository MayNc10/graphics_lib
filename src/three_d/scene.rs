use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::window;

use super::shaders::Program;
use super::shape::Light;
use super::shape::Shape;
use crate::matrix::Mat4;

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
}