//! Methods for loading a shape from modeling files.
use std::{fs, collections::HashMap, path::Path};

use super::*;

// FIXME: Remove this!
/// FIXME: Remove this!
pub const MAT: Material = Material {
    ambient_color: [0.2, 0.0, 0.0],
    diffuse_color: [0.6, 0.0, 0.0],
    emission_color: [0.0; 3],
    specular_color: [1.0, 1.0, 1.0],
    specular_exp: 16.0,
    transparency: 1.0,
    transmission_filter_color: None,
    optical_density: None,
    illum_model: None,
};

/// An enum represents errors that can happen in the importing process.
#[derive(Debug)]
pub enum ImportError {
    /// An error returned if the file doesn't have an extension compatible with the import method used.
    IncorrectExtension,
    /// An error returned by the file system.
    FileError(Box<dyn std::error::Error>),
    /// Our importing functions currently assumed the models have been triangulated. If they haven't, this error is returned.
    UnexpectedPolygon,
}
impl Shape {
    /// Loads the shape from the given obj file.
    ///
    /// Even if the obj file defines objects or vertex groups, it will all be imported as one shape.
    /// This is because there is no good way to find the origins of the objects/groups, so transformations of those shapes would be incorrect.
    /// Additionally, there are good reasons why a modeler would want multiple objects to be packed into the same shape.
    /// If you want to load two different shapes, split the shapes across files.
    // FIXME: Use different materials for the different objects/vertex groups in the file
    pub fn from_obj(
        path: &str,
        shader_type: ShaderType,
        transform: Option<Transform>, 
        animation: Option<Box<dyn Animation>>, 
        _bface_culling: bool,) -> Result<Shape, ImportError> 
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
            Material::from_mtl(&new_path)?
        } else { HashMap::new() };

        let mut vertices = Vec::new();
        //let mut tex_coords = Vec::new();
        let mut normals: Vec<Normal> = Vec::new();
        
        let mut vertices_out = Vec::new();
        let mut normals_out = Vec::new();

        let mut indices: Vec<u32> = Vec::new();

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

                    let v = [x, y, z];
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

                    let v = [x, y, z];
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
                            new_idx = Some(vertices_out.len() as u32 - 1);
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
                    eprintln!("WARNING: Loading two objects from the same .obj file can cause problems, and is unsupported");
                    material_name = None;
                },
                "usemtl" => {
                    material_name = Some(String::from(tokens.next().unwrap()));
                },
                "" => {},

                _ => { println!("Found line {line}, skipping") },
            }
        }

        let (vao, vao_lock) = VertexArrayObject::new().unwrap().into_inner();

        let positions = VertexBuffer::new(&vertices_out, &vao_lock);
        let normals_buffer = NormalBuffer::new(&normals_out, &vao_lock);
        let indices_buffer = IndexBuffer::new(&indices, &vao_lock);

        let new_animation = if let Some(b) = &animation {
            Some(b.clone_box())
        } else { None };

        // Is there a material for this object?
        let material = *mat_map.get(&material_name.unwrap_or_default()).unwrap_or(&Material::default());

        let s = Shape {
            vao,
            positions, 
            normals: normals_buffer, 
            indices: indices_buffer, 
            transform: transform.unwrap_or_default(), 
            animation: new_animation, shader_type, 
            material
        };

        Ok(s)
    }
        
}

#[derive(Clone, Copy, Debug)]
// This could be done better as multiple settings instead of one list
/// These are all the illumination models of the OBJ file format.
/// An illumination model roughly defines what type of rendering program should be used to render the object.
///
/// They are currently not used by the program.
// FIXME: Document these variants.
pub enum IlluminationModel {
    #[doc(hidden)]
    COnAmOff,
    #[doc(hidden)]
    COnAmOn,
    #[doc(hidden)]
    HighOn,
    #[doc(hidden)]
    ReOnRTOn,
    #[doc(hidden)]
    TransGlassOnReRTOn,
    #[doc(hidden)]
    ReFresOnRTOn,
    #[doc(hidden)]
    TransRefracOnReFresOffRtOn,
    #[doc(hidden)]
    TransRefracOnReFresOnRtOn,
    #[doc(hidden)]
    ReOnRTOff,
    #[doc(hidden)]
    TransGlassOnReRTOff,
    #[doc(hidden)]
    InvisShadow,
}

/// This struct represents an MTL material/
#[derive(Clone, Copy, Debug)]
pub struct Material {
    /// The ambient color of the object.
    pub ambient_color: [f32; 3],
    /// The diffuse color of the object.
    pub diffuse_color: [f32; 3],
    /// The emission color of the object.
    pub emission_color: [f32; 3],
    /// The specular color of the object.
    pub specular_color: [f32; 3],
    /// The specular exponent of the object.
    pub specular_exp: f32,
    /// The transparency of the object.
    pub transparency: f32,
    /// The transmission color filter of the object.
    pub transmission_filter_color: Option<[f32; 3]>,
    /// The optical density of the object.
    pub optical_density: Option<f32>,
    /// The illumination model of the object.
    pub illum_model: Option<IlluminationModel>,
}

/// If an object has no specified specular exponent, this value is used.
pub const DEFAULT_SPEC_EXP: f32 = 16.0;

impl Default for Material {
    fn default() -> Self {
        Material { ambient_color: [0.0; 3], diffuse_color: [1.0; 3], emission_color: [0.0; 3],
            specular_color: [0.0; 3], specular_exp: DEFAULT_SPEC_EXP, 
            transparency: 1.0, transmission_filter_color: None, optical_density: None, illum_model: None}
    }
}

impl Material {
    /// Load a material from an MTL file.
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

        'lines_loop: for line in lines {
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

                    current_mat.as_mut().unwrap().ambient_color = [r, g, b];
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
                "Ke" => {
                    let r = tokens.next().unwrap().parse().unwrap();
                    let g = tokens.next().unwrap().parse().unwrap();
                    let b = tokens.next().unwrap().parse().unwrap();

                    // I've seen this in a file and it does weird things
                    if r > 1.0 || g > 1.0 || b > 1.0 {
                        println!("WARNING: Emission color values greater than 1");
                        continue 'lines_loop;
                    }

                    current_mat.as_mut().unwrap().emission_color = [r, g, b];
                }
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