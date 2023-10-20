//! A trait representing a time-dependent animation, along with a few implementations of simple use cases.

use crate::matrix::*;

use super::shape::Transform;

/// An Animation is any struct that can define a transformation on an object.
/// This could be rotating the object, or sliding the object for some time, etc.
pub trait Animation : AnimationClone {
    /// Runs the animation, given the transform to mutate and the current timestep
    fn run(&mut self, t: f32, transform: &mut Transform);
}

unsafe trait RotationAnimation : Animation {}
unsafe trait ScalingAnimation : Animation {}
unsafe trait TranslationAnimation : Animation {}

#[doc(hidden)]
pub trait AnimationClone {
    fn clone_box(&self) -> Box<dyn Animation>;
}

#[doc(hidden)]
impl<T> AnimationClone for T 
where 
    T: 'static + Animation + Clone,
{
    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum RotationType {
    X,
    Y,
    Z,
}

/// This represents a constant rotation with some given period along a specified axis.
#[derive(Clone)]
pub struct ConstantRotation {
    /// The axis of rotation
    pub ty: RotationType,
    /// The period of rotation
    pub secs_per_loop: f32 
}

#[doc(hidden)]
impl Animation for ConstantRotation {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        // compute ang;e
        let angle = (360.0 * (t / self.secs_per_loop)) * std::f32::consts::PI / 180.0;
        let rotation = match self.ty {
            RotationType::X => generate_rotate_x(angle),
            RotationType::Y => generate_rotate_y(angle),
            RotationType::Z => generate_rotate_z(angle),
        };
        transform.set_rotation(rotation)
    }
}

#[doc(hidden)]
unsafe impl RotationAnimation for ConstantRotation {}

/// This represents a rotation along a specified axis, with the angle depending on some closure.
#[derive(Clone)]
pub struct Rotation {
    /// The axis of rotation.
    pub ty: RotationType,
    /// The function determining the current angle. This should output its result in degrees
    pub angle_func: fn(f32) -> f32,
}

#[doc(hidden)]
// The function should output its resulting angle in degrees
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

#[doc(hidden)]
unsafe impl RotationAnimation for Rotation {}

/// This represents scaling in all three dimensions, according to specified closures.
///
/// If `None` is given instead of a function, the value will stay the same was what it was the previous timestep.
#[derive(Clone)]
pub struct Scaling {
    /// The scaling in the x-dimension
    pub x_func: Option<fn(f32) -> f32>,
    /// The scaling in the y-dimension
    pub y_func: Option<fn(f32) -> f32>,
    /// The scaling in the z-dimension
    pub z_func: Option<fn(f32) -> f32>,
}

#[doc(hidden)]
impl Animation for Scaling {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        let x = if let Some(f) = self.x_func { (f)(t) } else { transform.scaling_matrix.inner[0][0] };
        let y = if let Some(f) = self.y_func { (f)(t) } else { transform.scaling_matrix.inner[1][1] };
        let z = if let Some(f) = self.z_func { (f)(t) } else { transform.scaling_matrix.inner[2][2] };

        let scaling = generate_scale(&[x, y, z]);
        transform.set_scaling(scaling);
    }
}

unsafe impl ScalingAnimation for Scaling {}

/// This represents a translation in along all three axes, specified by closures.
///
/// If a given function is marked as 'None', then nothing will be done along that axis.
#[derive(Clone)]
pub struct Translation {
    /// The function determining translation along the x-axis
    pub x_func: Option<fn(f32) -> f32>,
    /// The function determining translation along the y-axis
    pub y_func: Option<fn(f32) -> f32>,
    /// The function determining translation along the z-axis
    pub z_func: Option<fn(f32) -> f32>,
}

#[doc(hidden)]
impl Animation for Translation {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        let x_offset = if let Some(f) = self.x_func { (f)(t) } else { transform.translation_matrix.inner[3][0] };
        let y_offset = if let Some(f) = self.y_func { (f)(t) } else { transform.translation_matrix.inner[3][1] };
        let z_offset = if let Some(f) = self.z_func { (f)(t) } else { transform.translation_matrix.inner[3][2] };

        let translation = generate_translate(Some(x_offset), Some(y_offset), Some(z_offset));
        transform.set_translation(translation);
    }
}

unsafe impl TranslationAnimation for Translation {}

/// This struct represents a combination of animations.
///
/// The `scaling` animations should only scale, the `rotation` animations should only rotate, and the `translation` animations should only translate.
/// This ensures that the animations are applied in the proper order.
pub struct Composite {
    /// A list of the scaling animations to be applied
    pub scaling: Vec<Box<dyn RotationAnimation>>,
    /// A list of the rotation animations to be applied
    pub rotation: Vec<Box<dyn ScalingAnimation>>,
    /// A list of the translation animations to be applied
    pub translation: Vec<Box<dyn TranslationAnimation>>,
}

#[doc(hidden)]
impl Clone for Composite {
    fn clone(&self) -> Self {
        let mut new_scaling= Vec::with_capacity(self.scaling.len());
        for anim in &self.scaling {
            // TODO: This *should* be fine because RotationAnimation is an empty trait, but we should check this
            new_scaling.push(unsafe {
                std::mem::transmute(anim.clone_box())
            });
        }
        let mut new_rotation= Vec::with_capacity(self.rotation.len());
        for anim in &self.rotation {
            // TODO: See above
            new_scaling.push(unsafe {
                std::mem::transmute(anim.clone_box())
            });
        }
        let mut new_translation= Vec::with_capacity(self.translation.len());
        for anim in &self.translation {
            // TODO: See above

            new_scaling.push(unsafe {
                std::mem::transmute(anim.clone_box())
            });
        }

        Composite { scaling: new_scaling, rotation: new_rotation, translation: new_translation }
    }
}

#[doc(hidden)]
impl Animation for Composite {
    fn run(&mut self, t: f32, transform: &mut Transform) {
        for anim in &mut self.scaling {
            anim.run(t, transform);
        }

        for anim in &mut self.rotation {
            anim.run(t, transform);
        }

        for anim in &mut self.translation {
            anim.run(t, transform);
        }
    }
}

