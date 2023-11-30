use std::rc::Rc;
use std::sync::Arc;
use crate::three_d::raytracing::material::Material;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;

/// A struct representing the result of a hit
pub struct HitRecord {
    /// The point that was hit at (p = r0 + r*t)
    pub p: Vec3,
    /// The normal vector to the hit
    pub normal: Vec3,
    /// The material that was hit
    pub mat: Arc<dyn Material>,
    /// The 't' at which the ray hit (using the ray form r0 + r*t)
    pub t: f32,
    /// Whether the ray hit the front or back face of the object
    pub front_face: bool,
}

impl HitRecord {
    /// Determine in which direction the normal points, and whether the ray hit the front of a face or the back
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { *outward_normal * -1.0 };
    }
    /// Create a default material, except that it has the given material
    pub fn blank_with_mat(mat: Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            mat,
            t: 0.0,
            front_face: false,
        }
    }
    /// Create a new material, given information about the circle it hit
    /// This method specifically focuses on circles, but this class is general purpose
    pub fn new(root: f32, r: &Ray, center: &Vec3, radius: f32, mat: Arc<dyn Material>) -> HitRecord {
        let mut rec = HitRecord::blank_with_mat(mat);
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - *center) / radius;
        rec.set_face_normal(r, &outward_normal);

        rec
    }
    /// Return the hit record without the material
    pub fn self_without_mat(&self) -> HitRecordNoMat {
        HitRecordNoMat {
            p: self.p,
            normal: self.normal,
            t: self.t,
            front_face: self.front_face,
        }
    }
}

/// This struct represents a Hit Record, without the material information
/// This struct exists so that the material within a Hit Record can use data from the rest of the struct
pub struct  HitRecordNoMat {
    /// The point that was hit at (p = r0 + r*t)
    pub p: Vec3,
    /// The normal vector to the hit
    pub normal: Vec3,
    // This could be a reference or an Rc, but this makes it easier
    /// The 't' at which the ray hit (using the ray form r0 + r*t)
    pub t: f32,
    /// Whether the ray hit the front or back face of the object
    pub front_face: bool,
}