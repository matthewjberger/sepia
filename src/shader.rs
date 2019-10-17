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
