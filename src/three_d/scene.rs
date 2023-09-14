use super::shape::Shape;
use super::shaders;
use crate::matrix::Mat4;

use glium::Frame;
use glium::Program;

pub struct Scene {
    no_shading: (Vec<Shape>, Program),
    gouraud_shading: (Vec<Shape>, Program),
    blinn_phong_shading: (Vec<Shape>, Program),

    view: Mat4,
    light: [f32; 3],
}

impl Scene {
    pub fn new(view: Mat4, light: [f32; 3], display: &glium::Display) -> Scene {
        let no_shading =  glium::Program::from_source(
            display, shaders::DEFAULT_3D_SHADER, 
            shaders::DEFAULT_3D_FRAG_SHADER, None
        ).unwrap();
    
        let gouraud_shading = glium::Program::from_source(
        display, shaders::GOURAUD_3D_SHADER, 
            shaders::GOURAUD_3D_FRAG_SHADER, None
        ).unwrap();
        
        let blinn_phong_shading = glium::Program::from_source(
            display, shaders::BLINN_PHONG_3D_SHADER, 
            shaders::BLINN_PHONG_3D_FRAG_SHADER, None
        ).unwrap();

        Scene 
        {   no_shading: (Vec::new(), no_shading), 
            gouraud_shading: (Vec::new(), gouraud_shading), 
            blinn_phong_shading: (Vec::new(), blinn_phong_shading), 
            view, light
        }
    }

    /// Returns the index into the vector where the shape is located
    pub fn add_shape(&mut self, shape: Shape) -> usize {
        let add_to_vec = |v: &mut Vec<Shape>, shape: Shape| {
            v.push(shape);
            v.len() - 1
        };

        add_to_vec(match shape.shader_type {
            shaders::ShaderType::None => {
                &mut self.no_shading.0
            },
            shaders::ShaderType::Gouraud => {
                &mut self.gouraud_shading.0
            },
            shaders::ShaderType::BlinnPhong => {
                &mut self.blinn_phong_shading.0
            },
        }, shape)
    }
}

impl Scene {
    pub fn draw(&mut self, frame: &mut Frame, t: f32) {
        for shape in &mut self.no_shading.0 {
            shape.animate(t);
            shape.draw(frame, &self.light, &self.view, &self.no_shading.1);
        }

        for shape in &mut self.gouraud_shading.0 {
            shape.animate(t);
            shape.draw(frame, &self.light, &self.view, &self.gouraud_shading.1);
        }

        for shape in &mut self.blinn_phong_shading.0 {
            shape.animate(t);
            shape.draw(frame, &self.light, &self.view, &self.blinn_phong_shading.1);
        }
    }
}