use super::shape::Light;
use super::shape::Shape;
use super::shaders;
use crate::matrix::Mat4;
use crate::matrix::Vec4;

use glium::Display;
use glium::Frame;
use glium::Program;

pub struct Scene {
    no_shading: (Vec<Shape>, Program),
    gouraud_shading: (Vec<Shape>, Program),
    blinn_phong_shading: (Vec<Shape>, Program),

    view: Mat4,
    light: Light,
}

impl Scene {
    pub fn new(view: Mat4, light: Light, display: &glium::Display) -> Scene {
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
            shaders::BLINN_PHONG_3D_FRAG_10_LIGHTS_SHADER, None
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
    pub fn draw(&mut self, frame: &mut Frame, t: f32, display: &Display) {
        // Find all the lights in the scene
        // FIXME: Get actual values for ambient, diffuse, and spectral, instead of hardcoding them
        let mut point_lights = Vec::new();
        for shape in &mut self.no_shading.0 {
            shape.animate(t); // Animate to have an updated position

            let emission = shape.get_material().emission_color;
            println!("{}, {emission:?}", shape.center.is_some());
            // If the shape doesn't have a provided center, we can't calculate one (all the data is in the buffers)
            // So we just ignore it
            if let Some(center) = shape.center && (emission[0] > 0.0 || emission[1] > 0.0 || emission[2] > 0.0) {
                // This shape has positive emmision, so it emits light!
                // To get the actual center of the light, we have to scale it by the shapes transform matrix
                // First, this means making our vector 4-long
                let center4 = Vec4::from_v3(center);
                let transformed_center = shape.get_transform_matrix().transform_matrix * center4;

                point_lights.push(Light {
                    direction: transformed_center.to_v3(),
                    ambient: [0.0; 3],
                    diffuse: [1.0; 3],
                    specular: [1.0; 3],
                });
            }
        }

        for shape in &mut self.gouraud_shading.0 {
            shape.animate(t); // Animate to have an updated position

            let emission = shape.get_material().emission_color;
            println!("{}, {emission:?}", shape.center.is_some());
            // If the shape doesn't have a provided center, we can't calculate one (all the data is in the buffers)
            // So we just ignore it
            if let Some(center) = shape.center && (emission[0] > 0.0 || emission[1] > 0.0 || emission[2] > 0.0) {
                // This shape has positive emmision, so it emits light!
                // To get the actual center of the light, we have to scale it by the shapes transform matrix
                // First, this means making our vector 4-long
                let center4 = Vec4::from_v3(center);
                let transformed_center = shape.get_transform_matrix().transform_matrix * center4;

                point_lights.push(Light {
                    direction: transformed_center.to_v3(),
                    ambient: [0.0; 3],
                    diffuse: [1.0; 3],
                    specular: [1.0; 3],
                });
            }
        }

        for shape in &mut self.blinn_phong_shading.0 {
            shape.animate(t); // Animate to have an updated position

            let emission = shape.get_material().emission_color;
            println!("{}, {emission:?}", shape.center.is_some());
            // If the shape doesn't have a provided center, we can't calculate one (all the data is in the buffers)
            // So we just ignore it
            if let Some(center) = shape.center && (emission[0] > 0.0 || emission[1] > 0.0 || emission[2] > 0.0) {
                // This shape has positive emmision, so it emits light!
                // To get the actual center of the light, we have to scale it by the shapes transform matrix
                // First, this means making our vector 4-long
                let center4 = Vec4::from_v3(center);
                let transformed_center = shape.get_transform_matrix().transform_matrix * center4;

                point_lights.push(Light {
                    direction: transformed_center.to_v3(),
                    ambient: [0.0; 3],
                    diffuse: [1.0; 3],
                    specular: [1.0; 3],
                });
            }
        }

        // Draw all the shapes
        for shape in &mut self.no_shading.0 {
            shape.draw(frame, &self.light, &self.view, &self.no_shading.1, &point_lights, display);
        }

        for shape in &mut self.gouraud_shading.0 {
            shape.draw(frame, &self.light, &self.view, &self.gouraud_shading.1, &point_lights, display);
        }

        for shape in &mut self.blinn_phong_shading.0 {
            shape.draw(frame, &self.light, &self.view, &self.blinn_phong_shading.1, &point_lights, display);
        }
    }
}