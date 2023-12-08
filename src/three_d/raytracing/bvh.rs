use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use crate::three_d::raytracing::aabb::AABB;
use crate::three_d::raytracing::hit_record::HitRecord;
use crate::three_d::raytracing::interval::Interval;
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::shape::{RTObject, RTObjectVec};

/// A struct representing a BVHNode
/// A BVH renders raytracing objects faster by splitting itself into two halves, each with their own bounding boxes
/// This means that when calculating lighting, many rays can be rejected because they don't intersect the half that a certain object is in
/// A BVH with more than two base objects creates a tree structure, with each half being its own BVHNode
/// This creates a binary tree of smaller and smaller regions, which makes rendering faster because more rays can be rejected based on bounding-box collisions
#[derive(Clone)]
pub struct BVHNode {
    left: Box<dyn RTObject>,
    right: Box<dyn RTObject>,
    bbox: AABB,
}

impl BVHNode {
    /// Create a new BVHNode from a list of raytracing objects, and indices to specify what part of the vector should be used
    pub fn new(src_objects: &Vec<Box<dyn RTObject>>, start: usize, end: usize) -> BVHNode {
        let mut objects = (*src_objects).clone();
        let axis = thread_rng().gen_range(0..3);
        let comparator = |a: &Box<dyn RTObject>, b: &Box<dyn RTObject>| { BVHNode::box_compare(a.bounding_box(), b.bounding_box(), axis) };

        let object_span = end - start;
        if object_span == 0 {
            // this is pretty useless but it's nice to not stack overflow so
            let left = Box::new(RTObjectVec::new());
            let right = Box::new(RTObjectVec::new());
            BVHNode { left, right, bbox: AABB::empty() }
        }
        else if object_span == 1 {
            let left = objects[start].clone();
            let right = Box::new(RTObjectVec::new()); // The original code is using Arc instead of Box, and so it has left == right
            // Here we've just made right empty, it should be fine
            let bbox = left.bounding_box(); // The right bounding box doesn't exist.
            BVHNode { left, right, bbox}
        }
        else if object_span == 2 {
            let left = objects[start].clone();
            let right = objects[start + 1].clone();
            let bbox = AABB::new_from_boxes(left.bounding_box(), right.bounding_box());
            BVHNode { left, right, bbox}
        }
        else {
            let mut slc = objects[start..end].to_vec();
            slc.sort_by(comparator);

            let mid = object_span / 2;
            let left = BVHNode::new(&mut slc, 0, mid);
            let right = BVHNode::new(&mut slc, mid, object_span);
            let bbox = AABB::new_from_boxes(left.bounding_box(), right.bounding_box());
            BVHNode { left: Box::new(left), right: Box::new(right), bbox }
        }
    }

    fn box_compare(a: AABB, b: AABB, axis: u32) -> Ordering {
        a.axis(axis).min.partial_cmp(&b.axis(axis).min).unwrap()
    }
}

impl RTObject for BVHNode {
    fn ray_intersects(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) { None }
        else {
            let hit_left = self.left.ray_intersects(r, ray_t);
            let hit_right = self.right.ray_intersects(r,
                  Interval::new(ray_t.min, if hit_left.is_some() { hit_left.as_ref().unwrap().t } else { ray_t.max }) );
            // To stay with the original implementation, if hit right hits, it overwrites hit left
            // So we return it first if possible
            if hit_right.is_some() { hit_right }
                // Otherwise we just default to hit left
            else { hit_left }
        }
    }
    fn bounding_box(&self) -> AABB { self.bbox }
    fn clone_dyn(&self) -> Box<dyn RTObject> {
        Box::new(self.clone())
    }
}

impl From<RTObjectVec> for BVHNode {
    fn from(value: RTObjectVec) -> Self {
        BVHNode::new(value.objects(), 0, value.objects().len())
    }
}