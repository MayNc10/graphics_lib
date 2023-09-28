use gl::types::*;
use std::mem;
use super::VAO::VAOLock;

pub type Vertex = [GLfloat; 3];

pub type Normal = [GLfloat; 3];

pub struct VertexBuffer {
    id: GLuint,
}

impl VertexBuffer {
    pub fn new(data: &[Vertex], vao_lock: &VAOLock) -> VertexBuffer {
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
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            //gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct NormalBuffer {
    id: GLuint,
}

impl NormalBuffer {
    pub fn new(data: &[Normal], vao_lock: &VAOLock) -> NormalBuffer {
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
    pub fn new(data: &[GLuint], vao_lock: &VAOLock) -> IndexBuffer {
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
            //gl::DeleteBuffers(1, &self.id);
        }
    }
}