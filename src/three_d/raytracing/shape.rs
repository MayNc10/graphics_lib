use std::rc::Rc;
use std::sync::Arc;
use crate::three_d::raytracing::hit_record::HitRecord;
use crate::three_d::raytracing::interval::Interval;
use crate::three_d::raytracing::material::{EmptyMaterial, Material};
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;


pub trait RTObject: Send + Sync {
    fn ray_intersects(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct RTObjectVec {
    objects: Vec<Box<dyn RTObject>>,
}

impl RTObjectVec {
    pub fn new() -> RTObjectVec { RTObjectVec { objects: Vec::new() } }
    pub fn clear(&mut self) { self.objects.clear(); }
    pub fn add(&mut self, object: Box<dyn RTObject>) { self.objects.push(object); }
}

impl RTObject for RTObjectVec {
    fn ray_intersects(&self, r: &Ray, mut ray_t: Interval) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut final_rec = HitRecord::blank_with_mat(Arc::new(EmptyMaterial {}));

        for object in &self.objects {
            if let Some(rec) = object.ray_intersects(r, ray_t.replace_max(closest_so_far)) {
                hit_anything = true;
                closest_so_far = rec.t;
                final_rec = rec;
            }
        }

        if hit_anything { Some(final_rec) }
        else { None }
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Arc<dyn Material>) -> Sphere {
        Sphere { center, radius, mat }
    }
}

impl RTObject for Sphere {
    fn ray_intersects(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 { return None; }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root in an acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        Some(HitRecord::new(root, r, &self.center, self.radius, self.mat.clone()))
    }
}
