use gl::types::*;
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use crate::must_use::MustUse;

lazy_static! {
    static ref VAO_LOCK: Mutex<()> = Mutex::new(());
}

unsafe fn unbind_buffers() {
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
/// It's also nice because it'll automatically drop at the end of scope, and then it'll unbind everything
pub struct VAOLock<'a> {
    _guard: MutexGuard<'a, ()>,
}

impl VAOLock<'_> {
    /// Only make one of these when creating a VertexArrayObject
    unsafe fn new(guard: MutexGuard<'_, ()>) -> VAOLock<'_> {
        VAOLock { _guard: guard }
    }
}

impl<'a> Drop for VAOLock<'a> {
    fn drop(&mut self) {
        unsafe { unbind_buffers(); }
    }
}

#[derive(Debug)]
pub struct VertexArrayObject {
    id: GLuint,

}

impl VertexArrayObject {
    pub fn new() -> Option<MustUse<(VertexArrayObject, VAOLock<'static>)>> {
        // Check to make sure we haven't acquired the global VAO lock yet
        let guard = VAO_LOCK.lock();
        if let Ok(vao_lock) = guard {
            let mut id = 0;
            unsafe {
                gl::GenVertexArrays(1, &mut id);
                gl::BindVertexArray(id);
            }
            Some(( VertexArrayObject { id }, unsafe{ VAOLock::new(vao_lock) } ).into())
        } else { None }
    }

    pub fn bind(&self) -> Option<MustUse<VAOLock<'static>>> {
        let guard = VAO_LOCK.lock();
        if let Ok(vao_lock) = guard {
            unsafe {
                gl::BindVertexArray(self.id);
            }
            Some( unsafe{ VAOLock::new(vao_lock) }.into())
        }
        else { None }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id)
        }
    }
}