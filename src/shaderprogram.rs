use crate::shader::*;
pub use gl::types::*;
use std::ffi::CString;
use std::str;

#[derive(Default)]
pub struct ShaderProgram {
    pub id: GLuint,
    pub shader_ids: Vec<GLuint>,
}

impl ShaderProgram {
    pub fn new() -> Self {
        ShaderProgram {
            id: unsafe { gl::CreateProgram() },
            shader_ids: Vec::new(),
        }
    }

    fn attach(&mut self, kind: ShaderKind, path: &str) -> &mut Self {
        let mut shader = Shader::new(kind);
        shader.load_file(path);
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
        self.shader_ids.push(shader.id);
        self
    }

    pub fn vertex_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::Vertex, path)
    }

    pub fn geometry_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::Geometry, path)
    }

    pub fn tessellation_control_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::TessellationControl, path)
    }

    pub fn tessellation_evaluation_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::TessellationEvaluation, path)
    }

    pub fn compute_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::Compute, path)
    }

    pub fn fragment_shader(&mut self, path: &str) -> &mut Self {
        self.attach(ShaderKind::Fragment, path)
    }

    pub fn link(&mut self) {
        unsafe {
            gl::LinkProgram(self.id);
            for id in &self.shader_ids {
                gl::DeleteShader(*id);
            }
        }
        self.shader_ids.clear();
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn uniform_location(&self, name: &str) -> GLint {
        let name: CString = CString::new(name.as_bytes()).unwrap();
        unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
