use std::ops::Add;
use crate::three_d::raytracing::bvh::BVHNode;
use crate::three_d::raytracing::interval::Interval;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;

#[derive(Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn empty() -> AABB {
        AABB { x: Interval::empty(), y: Interval::empty(), z: Interval::empty()}
    }
    pub fn new(x: Interval, y: Interval, z: Interval) -> AABB {
        AABB {x, y, z}
    }

    pub fn new_from_points(a: Vec3, b: Vec3) -> AABB {
        // a and b are the extrema of the bounding box
        AABB {
            x: Interval::new(a.x().min(b.x()), a.x().max(b.x())),
            y: Interval::new(a.y().min(b.y()), a.y().max(b.y())),
            z: Interval::new(a.z().min(b.z()), a.z().max(b.z())),
        }
    }

    pub fn new_from_boxes(b1: AABB, b2: AABB) -> AABB {
        AABB {
            x: Interval::new_from_intervals(b1.x, b2.x),
            y: Interval::new_from_intervals(b1.y, b2.y),
            z: Interval::new_from_intervals(b1.z, b2.z),
        }
    }

    pub fn pad(&self) -> AABB {
        // Return an AABB that has no side narrower than some delta, padding if necessary
        let delta = 0.0001;
        let new_x = if self.x.size() > delta { self.x } else { self.x.expand(delta) };
        let new_y = if self.y.size() > delta { self.y } else { self.y.expand(delta) };
        let new_z = if self.z.size() > delta { self.z } else { self.z.expand(delta) };

        AABB::new(new_x, new_y, new_z)
    }

    pub fn axis(&self, n: u32) -> Interval {
        if n == 0 { self.x } else if n == 1 { self.y } else { self.z }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let inv_d = 1.0 / r.direction().data[axis];
            let orig = r.origin().data[axis];

            let mut t0 = (self.axis(axis as u32).min - orig) * inv_d;
            let mut t1 = (self.axis(axis as u32).max - orig) * inv_d;

            if inv_d < 0.0 { (t0, t1) = (t1, t0); }

            if t0 > ray_t.min { ray_t.min = t0; }
            if t1 < ray_t.max { ray_t.max = t1; }

            if ray_t.max <= ray_t.min { return false; }
        }
        true
    }
}

impl Default for AABB {
    fn default() -> Self {
        AABB::empty()
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;
    fn add(self, rhs: Vec3) -> Self::Output {
        AABB::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}