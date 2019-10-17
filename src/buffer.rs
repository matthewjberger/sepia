use crate::vao::*;
use gl::types::*;
use std::mem;

pub enum DrawingHint {
    StreamDraw,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

pub enum BufferKind {
    Array,
    Element,
}

impl Default for BufferKind {
    fn default() -> Self {
        BufferKind::Array
    }
}

#[derive(Default)]
pub struct Buffer {
    id: GLuint,
    kind: BufferKind,
    data: Vec<GLfloat>,
    pub len: u32,
}

impl Buffer {
    pub fn new(kind: BufferKind) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Buffer {
            id,
            kind,
            ..Default::default()
        }
    }

    pub fn add_data(&mut self, data: &[GLfloat]) {
        self.data.extend(data.iter().clone());
    }

    pub fn upload(&mut self, vao: &VertexArrayObject, hint: DrawingHint) {
        vao.bind();
        self.bind();
        unsafe {
            gl::BufferData(
                self.kind(),
                (self.data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                self.data.as_ptr() as *const GLvoid,
                Buffer::map_hint(&hint),
            );
        }
        self.len = self.data.len() as u32;
        self.data.clear();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.kind(), self.id as u32);
        }
    }

    pub fn free(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id as *const u32);
        }
    }

    fn kind(&self) -> GLuint {
        Buffer::map_type(&self.kind)
    }

    fn map_type(buffer_type: &BufferKind) -> GLuint {
        match buffer_type {
            BufferKind::Array => gl::ARRAY_BUFFER,
            BufferKind::Element => gl::ELEMENT_ARRAY_BUFFER,
        }
    }

    fn map_hint(drawing_hint: &DrawingHint) -> GLuint {
        match drawing_hint {
            DrawingHint::StreamDraw => gl::STREAM_DRAW,
            DrawingHint::StreamRead => gl::STREAM_READ,
            DrawingHint::StreamCopy => gl::STREAM_COPY,
            DrawingHint::StaticDraw => gl::STATIC_DRAW,
            DrawingHint::StaticRead => gl::STATIC_READ,
            DrawingHint::StaticCopy => gl::STATIC_COPY,
            DrawingHint::DynamicDraw => gl::DYNAMIC_DRAW,
            DrawingHint::DynamicRead => gl::DYNAMIC_READ,
            DrawingHint::DynamicCopy => gl::DYNAMIC_COPY,
        }
    }
}
