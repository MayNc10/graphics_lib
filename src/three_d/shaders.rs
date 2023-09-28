use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

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

#[derive(Copy, Clone)]
pub struct Shader(pub GLuint);

#[derive(Copy, Clone)]
pub struct Program(pub GLuint);

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