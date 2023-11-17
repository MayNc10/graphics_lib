use rand::{random, Rng, thread_rng};
use crate::three_d::raytracing::aabb::AABB;
use crate::three_d::raytracing::hit_record::{HitRecord, HitRecordNoMat};
use crate::three_d::raytracing::onb::ONB;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::shape::RTObject;
use crate::three_d::raytracing::vector::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray)>;
    fn emitted(&self) -> Vec3 { Vec3::default() }
    fn scattering_pdf(&self, ray_in: Ray, rec: &HitRecord, scattered: Ray) -> f32 { 0.0 }
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
        let uvw = ONB::build_from_w(rec.normal);

        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere(&mut thread_rng()).to_unit();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }

    fn scattering_pdf(&self, ray_in: Ray, rec: &HitRecord, scattered: Ray) -> f32 {
        let cos_theta = Vec3::dot(&rec.normal, &scattered.direction().unit());
        if cos_theta < 0.0 { 0.0 } else { cos_theta / std::f32::consts::PI }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal { Metal { albedo, fuzz } }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(&ray_in.direction().to_unit(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere(&mut thread_rng()).unit() * self.fuzz);
        if Vec3::dot(&scattered.direction(), &rec.normal) <= 0.0 { None }
        else { Some((self.albedo, scattered)) }
    }
}

pub struct Dielectric {
    refrac_index: f32,
}

impl Dielectric {
    pub fn new(refrac_index: f32) -> Dielectric { Dielectric { refrac_index } }

    // Use Schlick approximation
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {

    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new([1.0; 3]);
        let refrac_ratio = if rec.front_face { 1.0 / self.refrac_index } else { self.refrac_index };

        let unit_direction = ray_in.direction().unit();
        let cos_theta = Vec3::dot(&(unit_direction * -1.0), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if refrac_ratio * sin_theta > 1.0
        || Dielectric::reflectance(cos_theta, refrac_ratio) > random()
        {
            // cannot refract
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            // must refract
            Vec3::refract(&unit_direction, &rec.normal, refrac_ratio)
        };

        Some((attenuation, Ray::new(rec.p, direction)))
    }

}