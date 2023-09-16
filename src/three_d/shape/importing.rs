use std::{fs, collections::HashMap};

use crate::three_d::shaders;

use super::*;

#[derive(Debug)]
pub enum ImportError {
    FileError(Box<dyn std::error::Error>),
    UnexpectedPolygon,
}

impl Shape {
    pub fn from_obj(
        path: &str,
        shader_type: shaders::ShaderType,
        display: &Display, 
        transform: Option<Transform>, 
        animation: Option<Box<dyn Animation>>, 
        bface_culling: bool) -> Result<Shape, ImportError> 
    {
        let bytes = fs::read(path);
        if let Err(err) = bytes {
            return Err(ImportError::FileError(Box::new(err)));
        }
        let f = String::from_utf8_lossy(bytes.as_ref().unwrap());
        let lines = f.split("\n");
        
        let mut vertices = Vec::from([Vertex { position: (0.0,0.0,0.0) }]);
        //let mut tex_coords = Vec::new();
        let mut normals = Vec::from([Normal { normal: (0.0,0.0,0.0) }]);
        
        let mut vertices_out = Vec::from([Vertex { position: (0.0,0.0,0.0) }]);
        let mut normals_out = Vec::from([Normal { normal: (0.0,0.0,0.0) }]);

        let mut indices: Vec<u16> = Vec::new();

        for line in lines {
            let mut tokens = line.split_ascii_whitespace();

            match tokens.next().unwrap_or("") {
                // Vertex coords
                "v" => {   
                    // We don't care about w, so just find the first three numbers
                    let x = tokens.next().unwrap().parse().unwrap();
                    let y = tokens.next().unwrap().parse().unwrap();
                    let z = tokens.next().unwrap().parse().unwrap();

                    let v = Vertex { position: (x, y, z) };
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

                    let v = Normal { normal: (x, y, z) };
                    normals.push(v);
                },
                "vp" => {
                    // We skip these for now
                    // TODO: What do these do?
                    println!("Found parameter space vertex, skipping");
                },
                "f" => {
                    // Create a face
                    // TODO: I currently don't understand how glium matches normals with vertices
                    // I think it assumes that one index refers to both the vertex and the normal
                    // This is not true, and we need to fix this

                    // TODO: This code currently assumes that the obj provides normal information. Fix!

                    // Closure to create a new index
                    let mut add_idx = |v: usize, n: usize| {
                        // If we already have this vertex/normal pair, just reuse it
                        let mut new_idx = None;
                        for &idx in &indices {
                            if (vertices[v], normals[n]) == (vertices_out[idx as usize], normals_out[idx as usize]) {
                                new_idx = Some(idx);
                                break;
                            }
                        }
                        if new_idx.is_none() {
                            // Make a new pair and add it
                            vertices_out.push(vertices[v]);
                            normals_out.push(normals[n]);
                            // Leaving this here for testing
                            assert_eq!(vertices_out.len(), normals_out.len());
                            new_idx = Some(vertices_out.len() as u16 - 1);
                        }
                        indices.push(new_idx.unwrap());
                    };

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

                    add_idx(v1, n1);
                    add_idx(v2, n2);
                    add_idx(v3, n3);

                    // Are there actually more vertices
                    if let Some(tok) = tokens.next() {
                        // If this line isn't a comment, then there are more faces
                        if &tok[0..1] != "#" {
                            // We don't handle polgyons at this point, so return an error
                            println!("{}{:?}", tok, tokens);
                            return Err(ImportError::UnexpectedPolygon);
                        }
                    }                    
                },
                "" => {},

                _ => { println!("Found line {line}, skipping") },
            }
        }

        let positions  = glium::VertexBuffer::new(display, &vertices_out).unwrap();
        let normals = glium::VertexBuffer::new(display, &normals_out).unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList, 
            &indices).unwrap();

        Ok(Shape {positions, normals, indices, transform: transform.unwrap_or_default(), animation, shader_type, bface_culling: match bface_culling {
            true => glium::BackfaceCullingMode::CullClockwise,
            false => glium::BackfaceCullingMode::CullingDisabled,
        }})
    }
}