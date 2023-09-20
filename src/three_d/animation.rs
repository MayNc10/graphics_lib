use crate::matrix::*;

use super::shape::Transform;

// TODO: This should like. Just be a closure. Too complictacated imo.
pub trait Animation : AnimationClone {
    fn run(&mut self, t: f32, transform: &mut Transform);
}

pub trait AnimationClone {
    fn clone_box(&self) -> Box<dyn Animation>;
}

impl<T> AnimationClone for T 
where 
    T: 'static + Animation + Clone,
{
    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub enum RotationType {
    X,
    Y,
    Z,
}

#[derive(Clone)]
pub struct ConstantRotation {
    pub ty: RotationType,
    pub secs_per_loop: f32 
}

impl Animation for ConstantRotation {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        // compute angle
        let angle = (360.0 * (t / self.secs_per_loop)) * std::f32::consts::PI / 180.0;
        let rotation = match self.ty {
            RotationType::X => generate_rotate_x(angle),
            RotationType::Y => generate_rotate_y(angle),
            RotationType::Z => generate_rotate_z(angle), 
        };
        transform.set_rotation(rotation)
    }
}

#[derive(Clone)]
pub struct Rotation {
    pub ty: RotationType,
    pub angle_func: fn(f32) -> f32,
}

/// The function should output its resulting angle in degrees
impl Animation for Rotation {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        // compute angle
        let angle = (self.angle_func)(t) * std::f32::consts::PI / 180.0;
        let rotation = match self.ty {
            RotationType::X => generate_rotate_x(angle),
            RotationType::Y => generate_rotate_y(angle),
            RotationType::Z => generate_rotate_z(angle), 
        };
        transform.set_rotation(rotation)
    }
}

#[derive(Clone)]
pub struct Scaling {
    pub x_func: Option<fn(f32) -> f32>,
    pub y_func: Option<fn(f32) -> f32>,
    pub z_func: Option<fn(f32) -> f32>,
}

impl Animation for Scaling {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        let x = if let Some(f) = self.x_func { (f)(t) } else { transform.scaling_matrix.inner[0][0] };
        let y = if let Some(f) = self.y_func { (f)(t) } else { transform.scaling_matrix.inner[1][1] };
        let z = if let Some(f) = self.z_func { (f)(t) } else { transform.scaling_matrix.inner[2][2] };

        let scaling = generate_scale(&[x, y, z]);
        transform.set_scaling(scaling);
    }
}

// We don't have a good way of combining animations of the same type, so for now we just have one of each
/// This struct expects that each animation has a type corresponding to its name
/// E.G. the 'scaling' animation scales the shape
//#[derive(Clone)]
pub struct Composite {
    pub scaling: Option<Box<dyn Animation>>,
    pub rotation: Option<Box<dyn Animation>>,
    pub translation: Option<Box<dyn Animation>>,
}

impl Clone for Composite {
    fn clone(&self) -> Self {
        let new_scaling = if let Some(b) = &self.scaling {
            Some(b.clone_box())
        } else { None };
        let new_rotation = if let Some(b) = &self.rotation {
            Some(b.clone_box())
        } else { None };
        let new_translation= if let Some(b) = &self.translation {
            Some(b.clone_box())
        } else { None };

        Composite { scaling: new_scaling, rotation: new_rotation, translation: new_translation }
    }
}

impl Animation for Composite {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        if let Some(scaling) = self.scaling.as_mut() {
            scaling.run(t, transform);
        }
        if let Some(rotation) = self.rotation.as_mut() {
            rotation.run(t, transform);
        }
        if let Some(translation) = self.translation.as_mut() {
            translation.run(t, transform);
        }
    }
}

#[derive(Clone)]
pub struct Translation {
    pub x_func: Option<fn(f32) -> f32>,
    pub y_func: Option<fn(f32) -> f32>,
    pub z_func: Option<fn(f32) -> f32>,
}

impl Animation for Translation {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        let x_offset = if let Some(f) = self.x_func { (f)(t) } else { transform.translation_matrix.inner[3][0] };
        let y_offset = if let Some(f) = self.y_func { (f)(t) } else { transform.translation_matrix.inner[3][1] };
        let z_offset = if let Some(f) = self.z_func { (f)(t) } else { transform.translation_matrix.inner[3][2] };

        let translation = generate_translate(Some(x_offset), Some(y_offset), Some(z_offset));
        transform.set_translation(translation);
    }
}

