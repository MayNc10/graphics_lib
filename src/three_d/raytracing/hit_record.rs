use std::rc::Rc;
use std::sync::Arc;
use crate::three_d::raytracing::material::Material;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    // This could be a reference or an Rc, but this makes it easier
    pub mat: Arc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { *outward_normal * -1.0 };
    }
    pub fn blank_with_mat(mat: Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            mat,
            t: 0.0,
            front_face: false,
        }
    }

    pub fn new(root: f32, r: &Ray, center: &Vec3, radius: f32, mat: Arc<dyn Material>) -> HitRecord {
        let mut rec = HitRecord::blank_with_mat(mat);
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - *center) / radius;
        rec.set_face_normal(r, &outward_normal);

        rec
    }
    pub fn self_without_mat(&self) -> HitRecordNoMat {
        HitRecordNoMat {
            p: self.p,
            normal: self.normal,
            t: self.t,
            front_face: self.front_face,
        }
    }
}

pub struct  HitRecordNoMat {
    pub p: Vec3,
    pub normal: Vec3,
    // This could be a reference or an Rc, but this makes it easier
    pub t: f32,
    pub front_face: bool,
}