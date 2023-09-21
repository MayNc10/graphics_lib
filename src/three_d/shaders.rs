// Shader types
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    None,
    Gouraud,
    BlinnPhong,
}

// Create an OpenGL vextex shader for a vertex
pub const DEFAULT_3D_SHADER: &str = include_str!("shaders/default.glsl");

// Create an OpenGL fragment shader for a color vertex
// This determines the color (and other aspects) of a fragment (which is bacially just a pixel)
pub const DEFAULT_3D_FRAG_SHADER: &str = include_str!("shaders/default_frag.glsl");

// Create an OpenGL vextex shader for a vertex, using gouraud shading
pub const GOURAUD_3D_SHADER: &str = include_str!("shaders/gouraud.glsl");

// Create an OpenGL fragment shader for a color vertex
// This determines the color (and other aspects) of a fragment (which is bacially just a pixel
// This uses gouraud shading
pub const GOURAUD_3D_FRAG_SHADER: &str = include_str!("shaders/gouraud_frag.glsl");

pub const BLINN_PHONG_3D_SHADER: &str = include_str!("shaders/blinn_phong.glsl");

pub const BLINN_PHONG_3D_FRAG_SHADER: &str = include_str!("shaders/blinn_phong_frag.glsl");

pub const BLINN_PHONG_3D_FRAG_10_LIGHTS_SHADER: &str = include_str!("shaders/blinn_phong_frag_10_lights.glsl");
