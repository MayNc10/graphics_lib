use rand::{random, thread_rng};
use crate::three_d::raytracing::onb::ONB;
use crate::three_d::raytracing::random_cosine_direction;
use crate::three_d::raytracing::shape::RTObject;
use crate::three_d::raytracing::vector::Vec3;

/// A trait representing the PDF of a surface or object
/// A PDF, or Probability Density Function, computes the probability that a probability distribution generates values in a certain range
pub trait PDF {
    /// Compute the chance that reflections off of the surface would produce a vector similar to the input vector
    fn value(&self, direction: Vec3) -> f32;
    /// Compute a random reflection vector, biased towards reflection that would be more likely to occur
    fn generate(&self) -> Vec3;
}

/// A struct representing the PDF of uniformly sampling over a sphere
pub struct SpherePDF {}

impl PDF for SpherePDF {
    fn value(&self, _direction: Vec3) -> f32 {
        1.0 / (4.0 * std::f32::consts::PI)
    }
    fn generate(&self) -> Vec3 {
        Vec3::random_in_unit_sphere(&mut thread_rng())
    }
}

/// A struct representing the PDF of uniformly sampling over a hemisphere, given an orthonormal basis
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    /// Create a new hemisphere PDF, given an orthonormal basis
    pub fn new(w: Vec3) -> CosinePDF {
        CosinePDF { uvw: ONB::build_from_w(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f32 {
        let cosine_theta = Vec3::dot(&direction.unit(), &self.uvw.w());
        (cosine_theta / std::f32::consts::PI).max(0.0)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local_from_vector(random_cosine_direction())
    }
}

/// A struct representing a probability density function of a general raytracing object
pub struct RTObjectPDF {
    objects: Box<dyn RTObject>,
    origin: Vec3,
}

impl RTObjectPDF {
    /// Create a new PDF of a general raytracing object, given the ray origin
    pub fn new(objects: Box<dyn RTObject>, origin: Vec3) -> RTObjectPDF {
        RTObjectPDF { objects, origin }
    }
}

impl PDF for RTObjectPDF {
    fn value(&self, direction: Vec3) -> f32 {
        self.objects.pdf_value(self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }
}

/// A struct representing a probability density function for a mixture of two different PDFs
pub struct MixturePDF {
    p1: Box<dyn PDF>,
    p2: Box<dyn PDF>
}

impl MixturePDF {
    /// Create a new mixture PDF, based on two PDFs
    pub fn new(p1: Box<dyn PDF>, p2: Box<dyn PDF>) -> MixturePDF {
        MixturePDF { p1, p2 }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f32 {
        0.5 * self.p1.value(direction) + 0.5 * self.p2.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random::<f32>() < 0.5 {
            self.p1.generate()
        } else {
            self.p2.generate()
        }
    }
}