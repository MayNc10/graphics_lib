use std::ops::Add;
use std::rc::Rc;
use std::sync::Arc;
use rand::{random, Rng, thread_rng};
use crate::three_d::raytracing::aabb::AABB;
use crate::three_d::raytracing::hit_record::HitRecord;
use crate::three_d::raytracing::interval::Interval;
use crate::three_d::raytracing::material::{EmptyMaterial, Material};
use crate::three_d::raytracing::onb::ONB;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;


/// A trait representing a raytracing object
/// A raytracing object is a 3-dimensional object that provides methods for computing ray bounces and lighting
pub trait RTObject: Send + Sync {
    /// Compute whether the given ray in the specified interval intersects with the object
    /// If the ray intersects the object, this method should return a HitRecord with information about the intersection
    /// Otherwise, this value should return None
    fn ray_intersects(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
    /// Compute a bounding box that contains the object
    fn bounding_box(&self) -> AABB;
    /// Clone the object into a Box
    fn clone_dyn(&self) -> Box<dyn RTObject>;
    /// Compute the probability that a ray starting from a certain point with a certain direction would be reflected by this object
    /// This method is not required
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 { 0.0 }
    /// Compute a random ray bouncing off the object, given the origin of the vector
    /// This should take into account the geometry of the object to match the real probabilities of reflecting in different directions
    /// This method is not required
    fn random(&self, o: Vec3) -> Vec3 {
        [1.0, 0.0, 0.0].into()
    }
}

impl Clone for Box<dyn RTObject> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

/// A struct representing a list of raytracing objects
#[derive(Clone)]
pub struct RTObjectVec {
    objects: Vec<Box<dyn RTObject>>,
    bbox: AABB
}

impl RTObjectVec {
    /// Create a new, empty object vector
    pub fn new() -> RTObjectVec { RTObjectVec { objects: Vec::new(), bbox: AABB::empty() } }
    /// Create a new object vector, given a vector of objects
    pub fn new_from_vec(objects: Vec<Box<dyn RTObject>>) -> RTObjectVec {
        let mut rt = RTObjectVec { objects: Vec::new(), bbox: AABB::empty() };
        for object in objects {
            rt.bbox = AABB::new_from_boxes(rt.bbox, object.bounding_box());
            rt.objects.push(object);
        }
        rt
    }
    /// Clear the vector
    pub fn clear(&mut self) { self.objects.clear(); self.bbox = AABB::empty(); }
    /// Add a new object to the vector
    pub fn add(&mut self, object: Box<dyn RTObject>) {
        self.bbox = AABB::new_from_boxes(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
    /// Get the list of objects contained in the vector
    pub fn objects(&self) -> &Vec<Box<dyn RTObject>> { &self.objects }
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

    fn bounding_box(&self) -> AABB { self.bbox }
    fn clone_dyn(&self) -> Box<dyn RTObject> {
        Box::new(self.clone())
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        let weight = 1.0 / (self.objects.len() as f32);
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }

        sum
    }
    fn random(&self, o: Vec3) -> Vec3 {
        if self.objects.len() == 0 { Vec3::default() }
        else { self.objects[thread_rng().gen_range(0..self.objects.len())].random(o) }
    }
}

/// A struct represeting a sphere
#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    /// Create a new sphere, given a center point, a radius, and a material
    pub fn new(center: Vec3, radius: f32, mat: Arc<dyn Material>) -> Sphere {
        let r_vec = Vec3::new([radius; 3]);
        Sphere { center, radius, mat, bbox: AABB::new_from_points(center - r_vec, center + r_vec) }
    }

    fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
        let r1 = random::<f32>();
        let r2 = random::<f32>();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * std::f32::consts::PI * r1;
        let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
        let y = phi.sin() * (1.0 - z.powi(2)).sqrt();

        [x, y, z].into()
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

    fn bounding_box(&self) -> AABB { self.bbox }
    fn clone_dyn(&self) -> Box<dyn RTObject> {
        Box::new(self.clone())
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f32 {
        let hit_rec = self.ray_intersects(&Ray::new(o, v), Interval::new(0.001, f32::INFINITY));
        if hit_rec.is_none() { return 0.0 }

        let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - o).length_squared()).sqrt();
        let solid_angle = 2.0 * std::f32::consts::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let uvw = ONB::build_from_w(direction);
        uvw.local_from_vector(Sphere::random_to_sphere(self.radius, distance_squared))
    }
}

/// A struct representing a quadrilateral in 3-dimensional space
#[derive(Clone)]
pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    d: f32,
    w: Vec3,
    area: f32,

    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Quad {
    /// Create a new quadrilateral, given one corner q, two translations, u and v, and a material
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Quad {
        let n = Vec3::cross(&u, &v);
        Quad { q, u, v, normal: n.unit(), d: Vec3::dot(&n.unit(), &q), w: n / n.length_squared(), area: n.length(), mat, bbox: Quad::get_bbox(q, u, v) }
    }

    fn get_bbox(q: Vec3, u: Vec3, v: Vec3) -> AABB {
        AABB::new_from_points(q, q + u + v)
    }
    fn is_interior(a: f32, b: f32) -> bool {
        !(a < 0.0 || 1.0 < a || b < 0.0 || 1.0 < b)
    }
}

impl RTObject for Quad {
    fn ray_intersects(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = Vec3::dot(&self.normal, &r.direction());

        // No hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 { return None; }

        let t = (self.d - Vec3::dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) { return None; }

        // Determine if the hit point lies within the planar shape using its plane coordinates
        let intersection = r.at(t);
        let planar_hitpt_vec = intersection - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vec, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vec));
        if !Quad::is_interior(alpha, beta) { return None; }

        let mut rec = HitRecord::blank_with_mat(self.mat.clone());
        rec.t = t;
        rec.p = intersection;
        rec.set_face_normal(&r, &self.normal);
        Some(rec)
    }
    fn bounding_box(&self) -> AABB { self.bbox }
    fn clone_dyn(&self) -> Box<dyn RTObject> { Box::new(self.clone()) }
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f32 {
        let rec = self.ray_intersects(&Ray::new(origin, v), Interval::new(0.001, f32::INFINITY));
        if rec.is_none() { return 0.0; }
        let rec = rec.unwrap();

        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = (Vec3::dot(&v, &rec.normal) / v.length()).abs();

        distance_squared / (cosine * self.area)
    }
    fn random(&self, origin: Vec3) -> Vec3 {
        let p = self.q + self.u * random::<f32>() + self.v * random::<f32>();
        p - origin
    }
}

/// Create a 3-dimension box, given two points of opposite corners
pub fn make_box(a: Vec3, b: Vec3, mat: Arc<dyn Material>) -> Box<dyn RTObject> {
    let mut sides = RTObjectVec::new();

    let min: Vec3 = [a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z())].into();
    let max: Vec3 = [a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z())].into();

    let dx = [max.x() - min.x(), 0.0, 0.0].into();
    let dy = [0.0, max.y() - min.y(), 0.0].into();
    let dz = [0.0, 0.0, max.z() - min.z()].into();

    sides.add(Box::new(Quad::new([min.x(), min.y(), max.z()].into(), dx, dy, mat.clone()))); // front;
    sides.add(Box::new(Quad::new([max.x(), min.y(), max.z()].into(), dz * -1.0, dy, mat.clone()))); // right;
    sides.add(Box::new(Quad::new([max.x(), min.y(), min.z()].into(), dx * -1.0, dy, mat.clone()))); // back;
    sides.add(Box::new(Quad::new([min.x(), min.y(), min.z()].into(), dz, dy, mat.clone()))); // left;
    sides.add(Box::new(Quad::new([min.x(), max.y(), max.z()].into(), dx, dz * -1.0, mat.clone()))); // top;
    sides.add(Box::new(Quad::new([min.x(), min.y(), min.z()].into(), dx, dz, mat.clone()))); // bottom;

    Box::new(sides)
}

/// A struct representing a translation of an object
/// The struct consumes a base object and computes ray bounces as if the base object had been translated a certain amount
#[derive(Clone)]
pub struct Translate {
    object: Box<dyn RTObject>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    /// Create a new translation, given a base object and an offset vector
    pub fn new(object: Box<dyn RTObject>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;
        Translate { object, offset, bbox }
    }
}

impl RTObject for Translate {
    fn ray_intersects(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(r.origin() - self.offset, r.direction());
        let mut rec = self.object.ray_intersects(&offset_r, ray_t);
        if rec.is_none() { None }
        else { rec.as_mut().unwrap().p += self.offset; rec }
    }
    fn bounding_box(&self) -> AABB { self.bbox }
    fn clone_dyn(&self) -> Box<dyn RTObject> { Box::new(self.clone()) }
}

/// A struct representing an object that has been rotated around its y axis
/// This struct consumes a base object and calculates ray bounces for it as if it were rotated a certain amount around its y axis
#[derive(Clone)]
pub struct RotateY {
    object: Box<dyn RTObject>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: AABB,
}

impl RotateY {
    /// Create a new Y rotation, given the base option and the angle (in degrees)
    pub fn new(object: Box<dyn RTObject>, angle: f32) -> RotateY {
        let radians = angle * std::f32::consts::PI / 180.0;
        let (sin_theta, cos_theta) = radians.sin_cos();
        let bbox = object.bounding_box();

        let mut min = Vec3::new([f32::INFINITY; 3]);
        let mut max = Vec3::new([-f32::INFINITY; 3]);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.x.max + (1.0 - i as f32) * bbox.x.min;
                    let y = j as f32 * bbox.y.max + (1.0 - j as f32 ) * bbox.y.min;
                    let z = k as f32 * bbox.z.max + (1.0 - k as f32) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = [newx, y, newz];
                    for c in 0..3 {
                        min.data[c] = min.data[c].min(tester[c]);
                        max.data[c] = max.data[c].max(tester[c]);
                    }
                }
            }
        }

        RotateY { object, sin_theta, cos_theta, bbox: AABB::new_from_points(min, max) }
    }
}

impl RTObject for RotateY {
    fn ray_intersects(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Change from world to object space
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.data[0] = self.cos_theta * r.origin().data[0] - self.sin_theta * r.origin().data[2];
        origin.data[2] = self.sin_theta * r.origin().data[0] + self.cos_theta * r.origin().data[2];

        direction.data[0] = self.cos_theta * r.direction().data[0] - self.sin_theta * r.direction().data[2];
        direction.data[2] = self.sin_theta * r.direction().data[0] + self.cos_theta * r.direction().data[2];

        let rotated = Ray::new(origin, direction);

        let mut rec = self.object.ray_intersects(&rotated, ray_t);
        if rec.is_none() { return None; }
        let mut rec = rec.unwrap();

        let mut p = rec.p;
        p.data[0] = self.cos_theta * rec.p.data[0] + self.sin_theta * rec.p.data[2];
        p.data[2] = -self.sin_theta * rec.p.data[0] + self.cos_theta * rec.p.data[2];

        let mut normal = rec.normal;
        normal.data[0] = self.cos_theta * rec.normal.data[0] + self.sin_theta * rec.normal.data[2];
        normal.data[2] = -self.sin_theta * rec.normal.data[0] + self.cos_theta * rec.normal.data[2];

        rec.p = p;
        rec.normal = normal;

        Some(rec)
    }
    fn bounding_box(&self) -> AABB { self.bbox }
    fn clone_dyn(&self) -> Box<dyn RTObject> { Box::new(self.clone()) }
}