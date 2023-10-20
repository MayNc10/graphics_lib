use gl::types::*;
use lazy_static::lazy_static;
use std::ffi::CString;
use std::ptr;
use std::str;

// Shader types
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    None,
    Gouraud,
    BlinnPhong,
}

// Create an OpenGL vertex shader for a vertex
pub const DEFAULT_3D_SHADER: &str = include_str!("shaders/default.glsl");

// Create an OpenGL fragment shader for a color vertex
// This determines the color (and other aspects) of a fragment (which is basically just a pixel)
pub const DEFAULT_3D_FRAG_SHADER: &str = include_str!("shaders/default_frag.glsl");

// Create an OpenGL vertex shader for a vertex, using gouraud shading
pub const GOURAUD_3D_SHADER: &str = include_str!("shaders/gouraud.glsl");

// Create an OpenGL fragment shader for a color vertex
// This determines the color (and other aspects) of a fragment (which is basically just a pixel
// This uses gouraud shading
pub const GOURAUD_3D_FRAG_SHADER: &str = include_str!("shaders/gouraud_frag.glsl");

pub const BLINN_PHONG_3D_SHADER: &str = include_str!("shaders/blinn_phong.glsl");

pub const BLINN_PHONG_3D_FRAG_SHADER: &str = include_str!("shaders/blinn_phong_frag.glsl");

pub const PREPASS_SHADER: &str = include_str!("shaders/deferred/prepass.glsl");

pub const PREPASS_FRAG_SHADER: &str = include_str!("shaders/deferred/prepass_frag.glsl");

pub const LIGHTING_SHADER: &str = include_str!("shaders/deferred/lighting.glsl");

pub const BLINN_PHONG_3D_LIGHTING_FRAG_SHADER: &str = include_str!("shaders/deferred/blinn_phong_lighting_frag.glsl");

pub const BLINN_PHONG_3D_POINT_LIGHTING_FRAG_SHADER: &str = include_str!("shaders/deferred/blinn_phong_point_light_frag.glsl");

pub const EMISSION_FRAG_SHADER: &str = include_str!("shaders/deferred/emission_frag.glsl");

#[derive(PartialEq, Eq)]
pub struct Shader(pub GLuint);

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Program(pub GLuint);

impl Program {
    pub fn bind_color_output(&self, name: &CString) {
        unsafe {
            gl::UseProgram(self.0); 
            gl::BindFragDataLocation(self.0, 0, name.as_ptr());
            gl::UseProgram(0);
        }
    }

    pub fn use_progam(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}

// TODO: Add more shader types!
#[repr(u32)]
pub enum ShaderProgramType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

pub fn compile_shader(src: &str, ty: ShaderProgramType) -> Shader {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty as GLenum);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    Shader(shader)
}

pub fn link_program(vs: Shader, fs: Shader) -> Program {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs.0);
        gl::AttachShader(program, fs.0);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        Program(program)
    }
}

lazy_static! {
    pub static ref BLINN_PHONG: Program = {
        let vs = compile_shader(BLINN_PHONG_3D_SHADER, ShaderProgramType::Vertex);
        let fs = compile_shader(BLINN_PHONG_3D_FRAG_SHADER, ShaderProgramType::Fragment);
        link_program(vs, fs)
    };

    pub static ref PREPASS: Program = {
        let vs = compile_shader(PREPASS_SHADER, ShaderProgramType::Vertex);
        let fs = compile_shader(PREPASS_FRAG_SHADER, ShaderProgramType::Fragment);
        link_program(vs, fs)
    };

    pub static ref BLINN_PHONG_LIGHTING: Program = {
        let vs = compile_shader(LIGHTING_SHADER, ShaderProgramType::Vertex);
        let fs = compile_shader(BLINN_PHONG_3D_LIGHTING_FRAG_SHADER, ShaderProgramType::Fragment);
        link_program(vs, fs)
    };

    pub static ref BLINN_PHONG_POINT_LIGHTING: Program = {
        let vs = compile_shader(LIGHTING_SHADER, ShaderProgramType::Vertex);
        let fs = compile_shader(BLINN_PHONG_3D_POINT_LIGHTING_FRAG_SHADER, ShaderProgramType::Fragment);
        link_program(vs, fs)
    };

    pub static ref EMISSION: Program = {
        let vs = compile_shader(LIGHTING_SHADER, ShaderProgramType::Vertex);
        let fs = compile_shader(EMISSION_FRAG_SHADER, ShaderProgramType::Fragment);
        link_program(vs, fs)
    };
}