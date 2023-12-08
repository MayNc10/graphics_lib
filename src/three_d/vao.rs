//! This module provides a wrapper around an OpenGL VertexArrayObject

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

/// This represents a theoretical lock on the VAO.
/// Some OpenGL functions don't make sense to be called unless a VAO has been bound.
/// This object is created when a VAO is bound, and is a required argument for those functions.
pub struct VAOLock<'a> {
    _guard: MutexGuard<'a, ()>,
}

impl VAOLock<'_> {
    /// Only make one of these when creating a VertexArrayObject
    unsafe fn new(guard: MutexGuard<'_, ()>) -> VAOLock<'_> {
        VAOLock { _guard: guard }
    }
}

/// When we drop a VAOLock, we unbind the vertex buffers
impl<'a> Drop for VAOLock<'a> {
    fn drop(&mut self) {
        unsafe { unbind_buffers(); }
    }
}

/// A VertexArrayObject represents an OpenGL VAO.
///
/// This holds information about the vertices of a shape.
#[derive(Debug)]
pub struct VertexArrayObject {
    id: GLuint,

}

impl VertexArrayObject {
    /// Create a new VertexArrayObject.
    ///
    /// This method will return `None` if a VertexArrayObject already exists in the current scope.
    /// This method returns the VertexArrayObject, and also returns a VAOLock.
    /// Other methods in this library require a VAOLock to be called.
    /// This ensures that a VAO has been created or bound before other methods are called.
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

    /// This method binds an already-existing VAO and gives back a VAOLock
    ///
    /// Just like `VertexArrayObject::new()`, this method will fail if a VAO has already been bound
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