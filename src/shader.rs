pub use gl::types::*;
use std::ffi::CString;
use std::{fs, ptr, str};

pub enum ShaderKind {
    Vertex,
    Fragment,
    Geometry,
    TessellationControl,
    TessellationEvaluation,
    Compute,
}

impl Default for ShaderKind {
    fn default() -> Self {
        ShaderKind::Vertex
    }
}

#[derive(Default)]
pub struct Shader {
    pub id: GLuint,
    pub kind: ShaderKind,
}

impl Shader {
    pub fn new(kind: ShaderKind) -> Shader {
        let id = unsafe { gl::CreateShader(Shader::map_type(&kind)) };
        Shader { id, kind }
    }

    pub fn load_file(&mut self, path: &str) {
        let text = fs::read_to_string(path).unwrap();
        self.load(&text);
    }

    pub fn load(&self, source: &str) {
        let source_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(self.id, 1, &source_str.as_ptr(), ptr::null());
            gl::CompileShader(self.id);
        }
        self.check_compilation();
    }

    fn kind(&self) -> u32 {
        Shader::map_type(&self.kind)
    }

    fn check_compilation(&self) {
        // let mut info_log = Vec::with_capacity(1024);
        // let mut success = gl::FALSE as GLint;
        // unsafe {
        //     info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        //     gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
        // }
        // if success == gl::TRUE as GLint {
        //     return;
        // }
        // unsafe {
        //     gl::GetShaderInfoLog(
        //         self.id,
        //         1024,
        //         ptr::null_mut(),
        //         info_log.as_mut_ptr() as *mut GLchar,
        //     );
        // }
        // println!(
        //     "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
        //      -- --------------------------------------------------- -- ",
        //     self.kind(),
        //     str::from_utf8(&info_log).expect("Couldn't convert to UTF8!")
        // );
    }

    fn map_type(shader_type: &ShaderKind) -> GLuint {
        match shader_type {
            ShaderKind::Vertex => gl::VERTEX_SHADER,
            ShaderKind::Fragment => gl::FRAGMENT_SHADER,
            ShaderKind::Geometry => gl::GEOMETRY_SHADER,
            ShaderKind::TessellationControl => gl::TESS_CONTROL_SHADER,
            ShaderKind::TessellationEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderKind::Compute => gl::COMPUTE_SHADER,
        }
    }
}

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
