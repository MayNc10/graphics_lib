use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { *outward_normal * -1.0 };
    }
    pub fn new(root: f32, r: &Ray, center: &Vec3, radius: f32) -> HitRecord {
        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - *center) / radius;
        rec.set_face_normal(r, &outward_normal);

        rec
    }
}

pub trait RTObject {
    fn ray_intersects(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl RTObject for Sphere {
    fn ray_intersects(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 { return None; }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root in an acceptable range
        let root = (-half_b - sqrt_d) / a;
        if root <= tmin || tmax <= root {
            let root = (-half_b + sqrt_d) / a;
            if root <= tmin || tmax <= root {
                return None;
            }
        }

        Some(HitRecord::new(root, r, &self.center, self.radius))
    }
}