use rand::thread_rng;
use crate::three_d::raytracing::hit_record::{HitRecord, HitRecordNoMat};
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray)>;
}

pub struct EmptyMaterial {}
impl Material for EmptyMaterial {
    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray)> {
        panic!("Attempted to scatter a ray with an empty material");
    }
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere(&mut thread_rng()).to_unit();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}