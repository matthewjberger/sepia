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
    projection_matrix_location: u32,
    view_matrix_location: u32,
    skybox_location: u32,
}

impl Skybox {
    fn new(images: &[str; 6]) {
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

        self.projection_matrix_location = shader_program.uniform_location("projection");
        self.view_matrix_location = shader_program.uniform_location("view");
        self.skybox_location = shader_program.uniform_location("skybox");

        // Create a cubemap
        let texture_id = 0;
        unsafe {
            gl::GenTextures(1, &texture_id);
            gl::ActiveTexture(gl::TEXTURE0 + texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Set default wrapping parameters
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE);

            // Set default filtering options
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR);

            for index in 0..6 {
                let image_name = images[index];

                // Check number of channels
                // gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + index, 0, pixelFormat, width, height, 0, pixelFormat, gl::UNSIGNED_BYTE, image);
            }
        }
    }

    fn render(projection_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>) {
        self.shader_program.activate();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::UniformMatrix4fv(projection_matrix_location, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(view_matrix_location, 1, gl::FALSE, projection.as_ptr());

            gl::BindVertexArray(vao);
            // Bind cubemap
            // gl::Uniform1fv(skybox_location, 1, gl::FALSE, 0.0 ));

            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::DepthFunc(gl::LESS);
        }
    }
}
