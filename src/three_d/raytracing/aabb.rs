use std::ops::Add;
use crate::three_d::raytracing::interval::Interval;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::vector::Vec3;

/// A struct representing an axis-aligned bounding box
/// This is used to speed up ray intersection calculations by rejecting rays faster
/// Calculating intersections with a bounding box is much faster than for an arbitrary raytracing object, so this allows the code to reject more rays faster
#[derive(Copy, Clone, Debug)]
pub struct AABB {
    /// The interval along the x-axis that the bounding box occupies
    pub x: Interval,
    /// The interval along the y-axis that the bounding box occupies
    pub y: Interval,
    /// The interval along the z-axis that the bounding box occupies
    pub z: Interval,
}

impl AABB {
    /// Create an empty bounding box
    pub fn empty() -> AABB {
        AABB { x: Interval::empty(), y: Interval::empty(), z: Interval::empty()}
    }
    /// Create a bounding box given three intervals along each of the axes
    pub fn new(x: Interval, y: Interval, z: Interval) -> AABB {
        AABB {x, y, z}
    }

    /// Create a bounding box given two opposite points of the box
    pub fn new_from_points(a: Vec3, b: Vec3) -> AABB {
        // a and b are the extrema of the bounding box
        AABB {
            x: Interval::new(a.x().min(b.x()), a.x().max(b.x())),
            y: Interval::new(a.y().min(b.y()), a.y().max(b.y())),
            z: Interval::new(a.z().min(b.z()), a.z().max(b.z())),
        }
    }

    /// Create a bounding box from two smaller bounding boxes
    pub fn new_from_boxes(b1: AABB, b2: AABB) -> AABB {
        AABB {
            x: Interval::new_from_intervals(b1.x, b2.x),
            y: Interval::new_from_intervals(b1.y, b2.y),
            z: Interval::new_from_intervals(b1.z, b2.z),
        }
    }

    /// Return an AABB that has no side narrower than some delta, padding if necessary
    pub fn pad(&self) -> AABB {
        // Return an AABB that has no side narrower than some delta, padding if necessary
        let delta = 0.0001;
        let new_x = if self.x.size() > delta { self.x } else { self.x.expand(delta) };
        let new_y = if self.y.size() > delta { self.y } else { self.y.expand(delta) };
        let new_z = if self.z.size() > delta { self.z } else { self.z.expand(delta) };

        AABB::new(new_x, new_y, new_z)
    }

    /// Turn a number specifying a dimensional axis into the bounding box interval along that axis
    pub fn axis(&self, n: u32) -> Interval {
        if n == 0 { self.x } else if n == 1 { self.y } else { self.z }
    }

    /// Determine whether a given ray in a given interval insersects the box
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