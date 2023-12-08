use rand::{random, Rng, thread_rng};
use crate::three_d::raytracing::aabb::AABB;
use crate::three_d::raytracing::hit_record::{HitRecord, HitRecordNoMat};
use crate::three_d::raytracing::onb::ONB;
use crate::three_d::raytracing::random_cosine_direction;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::shape::RTObject;
use crate::three_d::raytracing::vector::Vec3;

/// A trait representing a generic material
pub trait Material: Send + Sync {
    /// Compute how a ray would scatter off of the material, given information about the ray intersection
    /// If the ray scatters, return information about the color of the scattered ray, the scattering direction, and the {DF (if supported)
    /// If the ray doesn't, return None
    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray, Option<f32>)>;
    /// Compute the emitted light of a material
    /// /// This method is optional
    fn emitted(&self, _ray_in: Ray, _rec: HitRecordNoMat, _u: f32, _v: f32, _p: Vec3) -> Vec3 { Vec3::default() }
    /// Compute the PDF of the material
    /// This method is optional
    fn scattering_pdf(&self, _ray_in: Ray, _rec: &HitRecord, _scattered: Ray) -> f32 { 0.0 }
}

/// A struct representing an empty material
/// Although this struct is visible, it can't be constructed, and is only for internal use
pub struct EmptyMaterial {}
impl Material for EmptyMaterial {
    fn scatter(&self, _ray_in: Ray, _rec: HitRecordNoMat) -> Option<(Vec3, Ray, Option<f32>)> {
        panic!("Attempted to scatter a ray with an empty material");
    }
}

/// A struct representing a normal object
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
     /// Create a new Lambertian, given the material's color
    pub fn new(albedo: Vec3) -> Lambertian { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray, Option<f32>)> {
        let uvw = ONB::build_from_w(rec.normal);

        let mut scatter_direction = uvw.local_from_vector(random_cosine_direction());

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction), Some(Vec3::dot(&uvw.w(), &scatter_direction) / std::f32::consts::PI)))
        // Give back the PDF!
    }

    fn scattering_pdf(&self, _ray_in: Ray, rec: &HitRecord, scattered: Ray) -> f32 {
        let cos_theta = Vec3::dot(&rec.normal, &scattered.direction().unit());
        if cos_theta < 0.0 { 0.0 } else { cos_theta / std::f32::consts::PI }
    }
}

/// A struct representing a metal material
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    /// Create a new metal material, given the metal's color and material's fuzziness
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal { Metal { albedo, fuzz } }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray, Option<f32>)> {
        let reflected = Vec3::reflect(&ray_in.direction().unit(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere(&mut thread_rng()).unit() * self.fuzz);
        if Vec3::dot(&scattered.direction(), &rec.normal) <= 0.0 { None }
        else { Some((self.albedo, scattered, None)) }
    }
}

/// A struct representing Dielectric materials
/// These are materials such as glass or water
pub struct Dielectric {
    refrac_index: f32,
    glass_color: Vec3,
}

impl Dielectric {
    /// Create a new Dielectric material, given an index of reflection
    pub fn new(refrac_index: f32) -> Dielectric { Dielectric { refrac_index, glass_color: [1.0; 3].into() } }
    pub fn new_colored(refrac_index: f32, color: Vec3) -> Dielectric { Dielectric { refrac_index, glass_color: color } }

    // Use Schlick approximation
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {

    fn scatter(&self, ray_in: Ray, rec: HitRecordNoMat) -> Option<(Vec3, Ray, Option<f32>)> {
        let attenuation = self.glass_color;
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

        Some((attenuation, Ray::new(rec.p, direction), None))
    }

}