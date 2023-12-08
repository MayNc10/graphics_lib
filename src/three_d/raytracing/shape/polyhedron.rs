use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use crate::three_d;
use crate::three_d::raytracing::aabb::AABB;
use crate::three_d::raytracing::bvh::BVHNode;
use crate::three_d::raytracing::hit_record::HitRecord;
use crate::three_d::raytracing::interval::Interval;
use crate::three_d::raytracing::material::{Lambertian, Material};
use crate::three_d::raytracing::ray::Ray;
use crate::three_d::raytracing::shape::{RTObject, RTObjectVec, Tri};
use crate::three_d::raytracing::vector::Vec3;
use crate::three_d::shape::importing::ImportError;

#[derive(Clone)]
pub struct Polyhedron {
    faces: Box<dyn RTObject>,
    aabb: AABB,
}

impl RTObject for Polyhedron {
    fn ray_intersects(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.faces.ray_intersects(r, ray_t)
    }
    fn bounding_box(&self) -> AABB {
        self.aabb
    }
    fn clone_dyn(&self) -> Box<dyn RTObject> {
        Box::new(self.clone())
    }
}

// FIXME: Parts of this code should be merged with importing logic from three_d::shape::importing
impl Polyhedron {
    pub fn from_obj(path: &str) -> Result<Polyhedron, ImportError>
    {
        if &path[path.len() - 4..] != ".obj" { return Err(ImportError::IncorrectExtension); }
        let bytes = fs::read(path);
        if let Err(err) = bytes {
            return Err(ImportError::FileError(Box::new(err)));
        }
        let f = String::from_utf8_lossy(bytes.as_ref().unwrap());
        let lines = f.split("\n");

        // Get materials library
        let mut new_path = String::from(&path[..path.len() - 3]);
        new_path.push_str("mtl");
        let mat_map = if Path::new(&new_path).exists() {
            three_d::shape::importing::Material::from_mtl(&new_path)?
        } else { HashMap::new() };

        let mut vertices = Vec::new();
        //let mut tex_coords = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut triangles = Vec::new();

        let mut material_name = None;
        let mut current_material = None;

        for line in lines {
            let mut tokens = line.split_ascii_whitespace();

            match tokens.next().unwrap_or("") {
                // Vertex coords
                "v" => {
                    // We don't care about w, so just find the first three numbers
                    let x = tokens.next().unwrap().parse().unwrap();
                    let y = tokens.next().unwrap().parse().unwrap();
                    let z = tokens.next().unwrap().parse().unwrap();

                    let v = [x, y, z].into();
                    vertices.push(v);
                },
                "vt" => {
                    // We don't handle these yet
                    println!("Found texture coord, skipping");
                },
                "vn" => {
                    // Normal
                    // We will assume these are unit vectors for now
                    // TODO: Fix this assumption
                    let x = tokens.next().unwrap().parse().unwrap();
                    let y = tokens.next().unwrap().parse().unwrap();
                    let z = tokens.next().unwrap().parse().unwrap();

                    let v = [x, y, z].into();
                    normals.push(v);
                },
                "vp" => {
                    // We skip these for now
                    // TODO: What do these do?
                    println!("Found parameter space vertex, skipping");
                },
                "f" => {
                    // Create a face

                    let tok = tokens.next().unwrap();

                    let mut tok = tok.split("/");
                    let v1: usize = tok.next().unwrap().parse().unwrap();
                    let n1: usize = tok.skip(1).next().unwrap().parse().unwrap();

                    let mut tok2 = tokens.next().unwrap().split("/");
                    let v2: usize = tok2.next().unwrap().parse().unwrap();
                    let n2: usize = tok2.skip(1).next().unwrap().parse().unwrap();

                    let mut tok3 = tokens.next().unwrap().split("/");
                    let v3: usize = tok3.next().unwrap().parse().unwrap();
                    let n3: usize = tok3.skip(1).next().unwrap().parse().unwrap();

                    // average out the vertex normals
                    let normal = (normals[n1 - 1] + normals[n2 - 1] + normals[n3 - 1]) / 3.0;
                    let points = [vertices[v1 - 1], vertices[v2 - 1], vertices[v3 - 1]];
                    let material = current_material.unwrap_or(crate::three_d::shape::importing::Material::default());
                    let color = material.diffuse_color;
                    let tri = Tri::new(points, normal, Arc::new(
                        Lambertian::new(color.into())
                    ));
                    triangles.push(Box::new(tri) as Box<dyn RTObject>);
                },
                "o" => {
                    eprintln!("WARNING: Loading two objects from the same .obj file can cause problems, and is unsupported");
                    material_name = None;
                },
                "usemtl" => {
                    material_name = Some(String::from(tokens.next().unwrap()));
                    current_material = Some(*mat_map.get(&material_name.unwrap_or_default()).unwrap_or(&crate::three_d::shape::importing::Material::default()));
                },
                "" => {},

                _ => { println!("Found line {line}, skipping") },
            }
        }
        let mut aabb = AABB::empty();
        for tri in &triangles {
            aabb = AABB::new_from_boxes(aabb, tri.bounding_box());
        }

        Ok(Polyhedron { faces: Box::new( BVHNode::new(&triangles, 0, triangles.len()) ), aabb })
    }
}
