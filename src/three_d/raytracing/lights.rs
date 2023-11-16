use crate::three_d::raytracing::hit_record::HitRecordNoMat;
use crate::three_d::raytracing::material::Material;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;

pub struct DiffuseLight {
    albedo: Vec3,
}

impl DiffuseLight {
    pub fn new(albedo: Vec3) -> DiffuseLight { DiffuseLight { albedo } }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray)> {
        None
    }
    fn emitted(&self) -> Vec3 {
        self.albedo
    }
}