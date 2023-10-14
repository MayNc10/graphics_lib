use gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use super::shaders::Program;
use super::vao::VAOLock;

pub type Vertex = [GLfloat; 3];

pub type Normal = [GLfloat; 3];

#[derive(Debug)]
pub struct VertexBuffer {
    id: GLuint,
}

impl VertexBuffer {
    pub fn new(data: &[Vertex], _vao_lock: &VAOLock) -> VertexBuffer {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                &data[0] as *const Vertex as *const _,
                gl::STATIC_DRAW,
            );
        }
        VertexBuffer { id }
    }
    pub fn id(&self) -> &GLuint {
        &self.id
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct NormalBuffer {
    id: GLuint,
}

impl NormalBuffer {
    pub fn new(data: &[Normal], _vao_lock: &VAOLock) -> NormalBuffer {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<Normal>()) as GLsizeiptr,
                &data[0] as *const Normal as *const _,
                gl::STATIC_DRAW,
            );
        }
        NormalBuffer { id }
    }
    pub fn id(&self) -> &GLuint {
        &self.id
    }
}

impl Drop for NormalBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct IndexBuffer {
    id: GLuint,
    pub num_indices: usize,
}

impl IndexBuffer {
    pub fn new(data: &[GLuint], _vao_lock: &VAOLock) -> IndexBuffer {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                &data[0] as *const GLuint as *const _,
                gl::STATIC_DRAW,
            );
        }
        IndexBuffer { id, num_indices: data.len() }
    }
    pub fn get_id(&self) -> &GLuint {
        &self.id
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct FrameBuffer {
    buffer_id: GLuint,
    position_id: GLuint,
    normal_id: GLuint,
    color_diffuse_id: GLuint,
    color_emission_id: GLuint,
    color_specular_id: GLuint,

    rbo_depth_id: GLuint,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32) -> FrameBuffer {
        let mut buffer_id = 0;
        let mut position_id = 0;
        let mut normal_id = 0;
        let mut color_diffuse_id = 0;
        let mut color_emission_id = 0;
        let mut color_specular_id = 0;
        let mut rbo_depth_id = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut buffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, buffer_id);
            
            // - position color buffer
            gl::GenTextures(1, &mut position_id);
            gl::BindTexture(gl::TEXTURE_2D, position_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as i32, width, height, 0, 
                gl::RGBA, gl::FLOAT, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, position_id, 0);
            
            // - normal color buffer
            gl::GenTextures(1, &mut normal_id);
            gl::BindTexture(gl::TEXTURE_2D, normal_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as i32, width, height, 0, 
                gl::RGBA, gl::FLOAT, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, gl::TEXTURE_2D, normal_id, 0);
            
            // - diffuse color buffer
            gl::GenTextures(1, &mut color_diffuse_id);
            gl::BindTexture(gl::TEXTURE_2D, color_diffuse_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, 
                gl::RGBA, gl::FLOAT, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT2, gl::TEXTURE_2D, color_diffuse_id, 0);
    
            // Emmision color buffer
            gl::GenTextures(1, &mut color_emission_id);
            gl::BindTexture(gl::TEXTURE_2D, color_emission_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, 
                gl::RGBA, gl::FLOAT, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT3, gl::TEXTURE_2D, color_emission_id, 0);
    
            // Specular color buffer
            gl::GenTextures(1, &mut color_specular_id);
            gl::BindTexture(gl::TEXTURE_2D, color_specular_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, 
                gl::RGBA, gl::FLOAT, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT4, gl::TEXTURE_2D, color_specular_id, 0);
            
            // - tell OpenGL which color attachments we'll use (of this framebuffer) for rendering 
            let attachments = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1, gl::COLOR_ATTACHMENT2, 
                gl::COLOR_ATTACHMENT3, gl::COLOR_ATTACHMENT4];
            gl::DrawBuffers(5, &attachments[0] as *const u32);
            
            gl::GenRenderbuffers(1, &mut rbo_depth_id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_depth_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, rbo_depth_id);
            // finally check if framebuffer is complete
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("Framebuffer not complete!");
            }
                
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        FrameBuffer { buffer_id, position_id, normal_id, color_diffuse_id, color_emission_id, color_specular_id, rbo_depth_id}
    }

    pub fn bind_textures(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.position_id);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.normal_id);
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.color_diffuse_id);
            gl::ActiveTexture(gl::TEXTURE3);
            gl::BindTexture(gl::TEXTURE_2D, self.color_emission_id);
            gl::ActiveTexture(gl::TEXTURE4);
            gl::BindTexture(gl::TEXTURE_2D, self.color_specular_id);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.buffer_id);
        }
    }

    pub fn add_uniforms(&self, names: &[CString; 5], program: &Program) {
        unsafe {

            let g_position_handle = gl::GetUniformLocation(program.0, names[0].as_ptr());
            let g_normal_handle = gl::GetUniformLocation(program.0, names[1].as_ptr());
            let g_color_diffuse_handle = gl::GetUniformLocation(program.0, names[2].as_ptr());
            let g_color_emission_handle = gl::GetUniformLocation(program.0, names[3].as_ptr());
            let g_color_specular_handle = gl::GetUniformLocation(program.0, names[4].as_ptr());

            gl::Uniform1i(g_position_handle, 0);
            gl::Uniform1i(g_normal_handle, 1);
            gl::Uniform1i(g_color_diffuse_handle, 2);
            gl::Uniform1i(g_color_emission_handle, 3);
            gl::Uniform1i(g_color_specular_handle, 4);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.rbo_depth_id as *const GLuint);
            let textures = [
                self.position_id,
                self.normal_id,
                self.color_diffuse_id,
                self.color_emission_id,
                self.color_specular_id,
            ];
            gl::DeleteTextures(textures.len() as GLsizei, textures.as_ptr());
            gl::DeleteFramebuffers(1, &self.buffer_id as *const GLuint);
        }
    }
}
