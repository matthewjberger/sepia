use crate::shader::*;

// TODO: Make common primitive geometry file (with indices)
#[rustfmt::skip]
static VERTICES: &[GLfloat; 36] =
&[
    // Positions
    -1.0f,  1.0f, -1.0f,
    -1.0f, -1.0f, -1.0f,
     1.0f, -1.0f, -1.0f,
     1.0f, -1.0f, -1.0f,
     1.0f,  1.0f, -1.0f,
    -1.0f,  1.0f, -1.0f,

    -1.0f, -1.0f,  1.0f,
    -1.0f, -1.0f, -1.0f,
    -1.0f,  1.0f, -1.0f,
    -1.0f,  1.0f, -1.0f,
    -1.0f,  1.0f,  1.0f,
    -1.0f, -1.0f,  1.0f,

     1.0f, -1.0f, -1.0f,
     1.0f, -1.0f,  1.0f,
     1.0f,  1.0f,  1.0f,
     1.0f,  1.0f,  1.0f,
     1.0f,  1.0f, -1.0f,
     1.0f, -1.0f, -1.0f,

    -1.0f, -1.0f, 1.0f,
    -1.0f,  1.0f, 1.0f,
     1.0f,  1.0f, 1.0f,
     1.0f,  1.0f, 1.0f,
     1.0f, -1.0f, 1.0f,
    -1.0f, -1.0f, 1.0f,

    -1.0f, 1.0f, -1.0f,
     1.0f, 1.0f, -1.0f,
     1.0f, 1.0f,  1.0f,
     1.0f, 1.0f,  1.0f,
    -1.0f, 1.0f,  1.0f,
    -1.0f, 1.0f, -1.0f,

    -1.0f, -1.0f, -1.0f,
    -1.0f, -1.0f,  1.0f,
     1.0f, -1.0f, -1.0f,
     1.0f, -1.0f, -1.0f,
    -1.0f, -1.0f,  1.0f,
     1.0f, -1.0f,  1.0f
];

struct Skybox {
    vao: u32,
    vbo: u32,
    shader_program: ShaderProgram,
}

impl Skybox {
    fn new(image: &[str; 6]) {
        let mut shader_program = ShaderProgram::new();
        shader_program
            .vertex_shader("assets/shaders/skybox/skybox.vs.glsl")
            .fragment_shader("assets/shaders/skybox/skybox.fs.glsl")
            .link();

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                VERTICES.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(0);
        }
    }

    fn render(view_matrix: &Matrix4<f32>) {
        self.shader_program.activate();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
    }
}
