use std::{fs::{self, File}, collections::HashMap, path::Path};

use crate::three_d::shaders;

use super::*;

pub const MAT: Material = Material {
    ambient_color: [0.2, 0.0, 0.0],
    diffuse_color: [0.6, 0.0, 0.0],
    specular_color: [1.0, 1.0, 1.0],
    specular_exp: 16.0,
    transparency: 1.0,
    transmission_filter_color: None,
    optical_density: None,
    illum_model: None,
};

#[derive(Debug)]
pub enum ImportError {
    IncorrectExtension,
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
        bface_culling: bool) -> Result<Vec<Shape>, ImportError> 
    {
        if &path[path.len() - 4..] != ".obj" { return Err(ImportError::IncorrectExtension); }
        let bytes = fs::read(path);
        if let Err(err) = bytes {
            return Err(ImportError::FileError(Box::new(err)));
        }
        let f = String::from_utf8_lossy(bytes.as_ref().unwrap());
        let lines = f.split("\n");
        
        let mut shapes = Vec::new();

        // Get materials library
        let mut new_path = String::from(&path[..path.len() - 3]);
        new_path.push_str("mtl");
        let mat_map = if Path::new(&new_path).exists() {
            Material::from_mtl(&new_path)?
        } else { HashMap::new() };

        let mut vertices = Vec::new();
        //let mut tex_coords = Vec::new();
        let mut normals = Vec::new();
        
        let mut vertices_out = Vec::new();
        let mut normals_out = Vec::new();

        let mut indices: Vec<u16> = Vec::new();

        let mut object_name = None;
        let mut material_name = None;

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
                        for i in 0..indices.len() {
                            let idx = indices[i] as usize;
                            if (vertices[v], normals[n]) == (vertices_out[idx], normals_out[idx]) {
                                new_idx = Some(indices[i]);
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

                    add_idx(v1 - 1, n1 - 1);
                    add_idx(v2 - 1, n2 - 1);
                    add_idx(v3 - 1, n3 - 1);

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
                "o" => {
                    // Have we actually parsed a shape yet?
                    if object_name.is_some() {
                        let positions  = glium::VertexBuffer::new(display, &vertices_out).unwrap();
                        let normals_buffer = glium::VertexBuffer::new(display, &normals_out).unwrap();
                        let indices_buffer = glium::IndexBuffer::new(
                            display,
                            glium::index::PrimitiveType::TrianglesList, 
                            &indices).unwrap();
                        
                        let new_animation = if let Some(b) = &animation {
                            Some(b.clone_box())
                        } else { None };

                        // Is there a material for this object?
                        let material = *mat_map.get(&material_name.unwrap_or_default()).unwrap_or(&Material::default());

                        let s = Shape {positions, normals: normals_buffer, indices: indices_buffer, 
                            transform: transform.unwrap_or_default(), 
                            animation: new_animation, shader_type, 
                            bface_culling: match bface_culling {
                                true => glium::BackfaceCullingMode::CullClockwise,
                                false => glium::BackfaceCullingMode::CullingDisabled,
                        }, material};
                        shapes.push(s);

                        // We don't clear the vertices or normals lists
                        vertices_out.clear();
                        normals_out.clear();
                        indices.clear();
                    }
                    object_name = Some(String::from(tokens.next().unwrap()));
                    material_name = None;
                },
                "usemtl" => {
                    material_name = Some(String::from(tokens.next().unwrap()));
                },
                "" => {},

                _ => { println!("Found line {line}, skipping") },
            }
        }

        let positions  = glium::VertexBuffer::new(display, &vertices_out).unwrap();
        let normals_buffer = glium::VertexBuffer::new(display, &normals_out).unwrap();
        let indices_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList, 
            &indices).unwrap();

        // Is there a material for this object?
        let material = *mat_map.get(&material_name.unwrap_or_default()).unwrap_or(&Material::default());

        let s = Shape {positions, normals: normals_buffer, indices: indices_buffer, 
            transform: transform.unwrap_or_default(), 
            animation, shader_type, 
            bface_culling: match bface_culling {
                true => glium::BackfaceCullingMode::CullClockwise,
                false => glium::BackfaceCullingMode::CullingDisabled,
        }, material};
        shapes.push(s);

        Ok(shapes)
    }
            
        

        
}

#[derive(Clone, Copy, Debug)]
// This could be done better as multiple settings instead of one list
pub enum IlluminationModel {
    COnAmOff,
    COnAmOn,
    HighOn,
    ReOnRTOn,
    TransGlassOnReRTOn,
    ReFresOnRTOn,
    TransRefracOnReFresOffRtOn,
    TransRefracOnReFresOnRtOn,
    ReOnRTOff,
    TransGlassOnReRTOff,
    InvisShadow,
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub specular_exp: f32,
    pub transparency: f32,
    pub transmission_filter_color: Option<[f32; 3]>,
    pub optical_density: Option<f32>,
    pub illum_model: Option<IlluminationModel>,
}

pub const DEFAULT_SPEC_EXP: f32 = 16.0;

impl Default for Material {
    fn default() -> Self {
        Material { ambient_color: [0.0; 3], diffuse_color: [1.0; 3], specular_color: [0.0; 3], specular_exp: DEFAULT_SPEC_EXP, 
            transparency: 1.0, transmission_filter_color: None, optical_density: None, illum_model: None}
    }
}

impl Material {
    pub fn from_mtl(path: &str) -> Result<HashMap<String, Material>, ImportError> {
        let bytes = fs::read(path);
        if let Err(err) = bytes {
            return Err(ImportError::FileError(Box::new(err)));
        }
        let f = String::from_utf8_lossy(bytes.as_ref().unwrap());
        let lines = f.split("\n");

        let mut materials = HashMap::new();
        let mut current_mat: Option<Material> = None;
        let mut current_mat_name: Option<String> = None;

        for line in lines {
            let mut tokens = line.split_ascii_whitespace();
            println!("{current_mat:?}");

            match tokens.next().unwrap_or("") {
                "newmtl" => {
                    // Insert old material
                    if let Some(mat) = current_mat {
                        materials.insert(current_mat_name.unwrap().clone(), mat.clone());
                    }

                    // Create new material
                    current_mat = Some(Material::default());
                    current_mat_name = tokens.next().map(|x| x.to_string());
                }
                "Ka" => {
                    let r: f32 = tokens.next().unwrap().parse().unwrap();
                    let g: f32 = tokens.next().unwrap().parse().unwrap();
                    let b: f32 = tokens.next().unwrap().parse().unwrap();

                    // TODO: Blender exports with white ambient light, but that's wrong
                    // Fix it
                    println!("Overriding ambient color as black!");
                    current_mat.as_mut().unwrap().ambient_color = [0.0; 3];
                },
                "Kd" => {
                    let r = tokens.next().unwrap().parse().unwrap();
                    let g = tokens.next().unwrap().parse().unwrap();
                    let b = tokens.next().unwrap().parse().unwrap();

                    current_mat.as_mut().unwrap().diffuse_color = [r, g, b];
                },
                "Ks" => {
                    let r = tokens.next().unwrap().parse().unwrap();
                    let g = tokens.next().unwrap().parse().unwrap();
                    let b = tokens.next().unwrap().parse().unwrap();

                    current_mat.as_mut().unwrap().specular_color = [r, g, b];
                },
                "Ns" => {
                    let exp = tokens.next().unwrap().parse().unwrap();

                    current_mat.as_mut().unwrap().specular_exp = exp;
                },
                "d" => {
                    let transparency = tokens.next().unwrap().parse().unwrap();

                    current_mat.as_mut().unwrap().transparency = transparency;
                }
                "Tr" => {
                    let tr = 1.0 - tokens.next().unwrap().parse::<f32>().unwrap();

                    current_mat.as_mut().unwrap().transparency = tr;
                }
                "Tf" => {
                    // TODO: Parse CIEXYZ format
                    // This code will crash if given CIEXYZ

                    let r = tokens.next().unwrap().parse().unwrap();
                    let g = tokens.next().unwrap().parse().unwrap();
                    let b = tokens.next().unwrap().parse().unwrap();

                    current_mat.as_mut().unwrap().transmission_filter_color = Some([r, g, b]);
                },
                "Ni" => {
                    let density = tokens.next().unwrap().parse().unwrap();

                    current_mat.as_mut().unwrap().optical_density = Some(density);
                },
                "illum" => {
                    current_mat.as_mut().unwrap().illum_model = match tokens.next().unwrap().parse().unwrap() {
                        0 => {Some(IlluminationModel::COnAmOff)},
                        1 => {Some(IlluminationModel::COnAmOn)},
                        2 => {Some(IlluminationModel::HighOn)},
                        3 => {Some(IlluminationModel::ReOnRTOn)},
                        4 => {Some(IlluminationModel::TransGlassOnReRTOn)},
                        5 => {Some(IlluminationModel::ReFresOnRTOn)},
                        6 => {Some(IlluminationModel::TransRefracOnReFresOffRtOn)},
                        7 => {Some(IlluminationModel::TransRefracOnReFresOnRtOn)},
                        8 => {Some(IlluminationModel::ReOnRTOff)},
                        9 => {Some(IlluminationModel::TransGlassOnReRTOff)},
                        10 => {Some(IlluminationModel::InvisShadow)}, 
                        _ => None,
                    };
                }

                "" => {},

                _ => { println!("Found line {line}, skipping") },
            }
        }

        // Insert old material
        if let Some(mat) = current_mat {
            materials.insert(current_mat_name.unwrap().clone(), mat.clone());
        }

        Ok(materials)
    }
}