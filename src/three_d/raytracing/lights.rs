use crate::three_d::raytracing::hit_record::HitRecordNoMat;
use crate::three_d::raytracing::material::Material;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;
/// A struct representing a light that emits light
pub struct DiffuseLight {
    albedo: Vec3,
}

impl DiffuseLight {
    /// Create a light, given the color that should be emitted
    pub fn new(albedo: Vec3) -> DiffuseLight { DiffuseLight { albedo } }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: Ray, _rec: HitRecordNoMat) -> Option<(Vec3, Ray, Option<f32>)> {
        None
    }
    fn emitted(&self, _ray_in: Ray, rec: HitRecordNoMat, u: f32, v: f32, p: Vec3) -> Vec3 {
        if !rec.front_face { Vec3::default() }
        else { self.albedo }
    }
}