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

    pub fn vertex_shader_file(&mut self, path: &str) -> &mut Self {
        self.attach_shader_file(ShaderKind::Vertex, path)
    }

    pub fn vertex_shader_source(&mut self, source: &str) -> &mut Self {
        self.attach_shader_source(ShaderKind::Vertex, source)
    }

    pub fn geometry_shader_file(&mut self, path: &str) -> &mut Self {
        self.attach_shader_file(ShaderKind::Geometry, path)
    }

    pub fn geometry_shader_source(&mut self, source: &str) -> &mut Self {
        self.attach_shader_source(ShaderKind::Geometry, source)
    }

    pub fn tessellation_control_shader_file(&mut self, path: &str) -> &mut Self {
        self.attach_shader_file(ShaderKind::TessellationControl, path)
    }

    pub fn tessellation_control_shader_source(&mut self, source: &str) -> &mut Self {
        self.attach_shader_source(ShaderKind::TessellationControl, source)
    }

    pub fn tessellation_evaluation_shader_file(&mut self, path: &str) -> &mut Self {
        self.attach_shader_file(ShaderKind::TessellationEvaluation, path)
    }

    pub fn tessellation_evaluation_shader_source(&mut self, source: &str) -> &mut Self {
        self.attach_shader_source(ShaderKind::TessellationEvaluation, source)
    }

    pub fn compute_shader_file(&mut self, path: &str) -> &mut Self {
        self.attach_shader_file(ShaderKind::Compute, path)
    }

    pub fn compute_shader_source(&mut self, source: &str) -> &mut Self {
        self.attach_shader_source(ShaderKind::Compute, source)
    }

    pub fn fragment_shader_file(&mut self, path: &str) -> &mut Self {
        self.attach_shader_file(ShaderKind::Fragment, path)
    }

    pub fn fragment_shader_source(&mut self, source: &str) -> &mut Self {
        self.attach_shader_source(ShaderKind::Fragment, source)
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

    pub fn free(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

    // TODO: Add helpers for setting uniforms
    // Set floats
    // Vec2
    // Vec3
    // Vec4
    // Set integers
    // Get attribute location

    pub fn set_uniform_integer(&self, name: &str, value: i32) {
        self.activate();
        let location = self.uniform_location(name);
        unsafe {
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_uniform_matrix4x4(&self, name: &str, data: &[GLfloat]) {
        self.activate();
        let location = self.uniform_location(name);
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, data.as_ptr());
        }
    }

    fn attach_shader_file(&mut self, kind: ShaderKind, path: &str) -> &mut Self {
        let mut shader = Shader::new(kind);
        shader.load_file(path);
        self.attach(&shader)
    }

    fn attach_shader_source(&mut self, kind: ShaderKind, source: &str) -> &mut Self {
        let shader = Shader::new(kind);
        shader.load(source);
        self.attach(&shader)
    }

    fn attach(&mut self, shader: &Shader) -> &mut Self {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
        self.shader_ids.push(shader.id);
        self
    }
}
