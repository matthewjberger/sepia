use gl::types::*;
use std::{mem, ptr};

#[derive(Default)]
pub struct VertexArrayObject {
    id: GLuint,
}

impl VertexArrayObject {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArrayObject { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn configure_attribute(&self, index: u32, count: u32, total: u32, offset: u32) {
        self.bind();
        let float_size = mem::size_of::<GLfloat>() as u32;
        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(
                index,
                count as i32,
                gl::FLOAT,
                gl::FALSE,
                (total * float_size) as i32,
                (offset * float_size) as *const GLvoid,
            );
        }
    }
}
