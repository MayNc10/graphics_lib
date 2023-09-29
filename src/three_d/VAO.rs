use gl::types::*;
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;

lazy_static! {
    static ref VAO_LOCK: Mutex<()> = Mutex::new(());
}

pub unsafe fn unbind_buffers() {
    unsafe {
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
}

/// This represents a theoretical lock on the VAO
/// The VAO needs a lock because each VAO corresponds to a specific VBO and EBO
/// For our code to be correct, we have to create the VAO, then create our VBO and EBO, then unbind everything
/// Therefore, we shouldn't have two VAOs at the same time
/// This represents this idea 
/// It's also nice because it'll automaticall drop at the end of scope, and then it'll unbind everything
pub struct VAOLock<'a> {
    _guard: MutexGuard<'a, ()>,
}

impl VAOLock<'_> {
    /// Only make one of these when creating a VertexArrayObject
    pub(crate) unsafe fn new<'a>(guard: MutexGuard<'a, ()>) -> VAOLock<'a> {
        VAOLock { _guard: guard }
    }
}

impl<'a> Drop for VAOLock<'a> {
    fn drop(&mut self) {
        unsafe { unbind_buffers(); }
    }
}

pub struct VertexArrayObject {
    id: GLuint,
}

impl VertexArrayObject {
    pub fn new() -> Option<(VertexArrayObject, VAOLock<'static>)> {
        // Check to make sure we haven't aquired the global VAO lock yet
        let guard = VAO_LOCK.lock();
        if let Ok(vao_lock) = guard {
            let mut id = 0;
            unsafe {
                gl::GenVertexArrays(1, &mut id);
                gl::BindVertexArray(id);
            }
            Some(( VertexArrayObject { id }, unsafe{ VAOLock::new(vao_lock) } ))
        } else {
            None
        }
    }

    pub fn id(&self) -> &GLuint {
        &self.id
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id)
        }
    }
}