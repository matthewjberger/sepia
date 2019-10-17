use crate::buffer::*;
use crate::shader::*;
use crate::shaderprogram::*;
use crate::texture::*;
use crate::vao::*;
use nalgebra_glm as glm;

// TODO: Make common primitive geometry file (with indices)
#[rustfmt::skip]
const VERTEX_POSITIONS: &[GLfloat; 108] =
    &[
       -1.0,  1.0, -1.0,
       -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
       -1.0,  1.0, -1.0,

        1.0, -1.0, -1.0,
        1.0, -1.0,  1.0,
        1.0,  1.0, -1.0,

        1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0, -1.0,

        1.0, -1.0,  1.0,
       -1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,

       -1.0, -1.0,  1.0,
       -1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,

       -1.0, -1.0,  1.0,
       -1.0, -1.0, -1.0,
       -1.0,  1.0,  1.0,

       -1.0, -1.0, -1.0,
       -1.0,  1.0, -1.0,
       -1.0,  1.0,  1.0,

       -1.0, -1.0,  1.0,
        1.0, -1.0,  1.0,
        1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
       -1.0, -1.0, -1.0,
       -1.0, -1.0,  1.0,

       -1.0,  1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0,  1.0,  1.0,

        1.0,  1.0,  1.0,
       -1.0,  1.0,  1.0,
       -1.0,  1.0, -1.0
    ];

#[derive(Default)]
pub struct Skybox {
    vao: VertexArrayObject,
    vbo: Buffer,
    shader_program: ShaderProgram,
    projection_matrix_location: i32,
    view_matrix_location: i32,
    skybox_location: i32,
    texture: Texture,
}

impl Skybox {
    pub fn new(paths: &[String; 6]) -> Self {
        let mut skybox = Skybox::default();
        skybox.shader_program = ShaderProgram::new();
        skybox
            .shader_program
            .vertex_shader_file("assets/shaders/skybox/skybox.vs.glsl")
            .fragment_shader_file("assets/shaders/skybox/skybox.fs.glsl")
            .link();

        skybox.vao = VertexArrayObject::new();
        skybox.vbo = Buffer::new();
        skybox.vbo.add_data(VERTEX_POSITIONS);
        skybox.vbo.upload(&skybox.vao, DrawingHint::StaticDraw);
        skybox.vao.configure_attribute(0, 3, 3, 0);
        skybox.projection_matrix_location = skybox.shader_program.uniform_location("projection");
        skybox.view_matrix_location = skybox.shader_program.uniform_location("view");
        skybox.skybox_location = skybox.shader_program.uniform_location("skybox");
        skybox.texture = Texture::cubemap_from_files(paths);
        skybox
    }

    pub fn render(&self, projection_matrix: &glm::Mat4, view_matrix: &glm::Mat4) {
        self.shader_program.activate();

        let view_matrix = glm::mat3_to_mat4(&glm::mat4_to_mat3(&*view_matrix));
        self.vao.bind();
        self.texture.bind(0);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::UniformMatrix4fv(
                self.projection_matrix_location,
                1,
                gl::FALSE,
                projection_matrix.as_ptr(),
            );

            gl::UniformMatrix4fv(
                self.view_matrix_location,
                1,
                gl::FALSE,
                view_matrix.as_ptr(),
            );

            gl::Uniform1i(self.skybox_location, 0);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::DepthFunc(gl::LESS);
        }
    }
}
