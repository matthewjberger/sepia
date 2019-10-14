use crate::shader::*;
use crate::texture::*;
use nalgebra::Matrix4;
use nalgebra_glm as glm;
use std::{mem, ptr};

// TODO: Make common primitive geometry file (with indices)
#[rustfmt::skip]
const VERTICES: &[GLfloat; 108] =
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
    vao: u32,
    vbo: u32,
    shader_program: ShaderProgram,
    projection_matrix_location: i32,
    view_matrix_location: i32,
    skybox_location: i32,
    texture: Texture,
}

impl Skybox {
    pub fn new(paths: &[String; 6]) -> Self {
        let mut skybox = Skybox::default();
        let mut shader_program = ShaderProgram::new();
        shader_program
            .vertex_shader("assets/shaders/skybox/skybox.vs.glsl")
            .fragment_shader("assets/shaders/skybox/skybox.fs.glsl")
            .link();

        unsafe {
            gl::GenVertexArrays(1, &mut skybox.vao);
            gl::BindVertexArray(skybox.vao);

            gl::GenBuffers(1, &mut skybox.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, skybox.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                VERTICES.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as i32,
                ptr::null(),
            );
        }

        skybox.projection_matrix_location = shader_program.uniform_location("projection");
        skybox.view_matrix_location = shader_program.uniform_location("view");
        skybox.skybox_location = shader_program.uniform_location("skybox");
        skybox.texture = Texture::cubemap_from_files(paths);
        skybox
    }

    pub fn render(&self, projection_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>) {
        self.shader_program.activate();

        let view_matrix = glm::mat3_to_mat4(&glm::mat4_to_mat3(&glm::convert(*view_matrix)));

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::UniformMatrix4fv(
                self.projection_matrix_location,
                1,
                gl::FALSE,
                (*projection_matrix).as_slice().as_ptr(),
            );
            gl::UniformMatrix4fv(
                self.view_matrix_location,
                1,
                gl::FALSE,
                view_matrix.as_slice().as_ptr(),
            );

            gl::BindVertexArray(self.vao);

            self.texture.bind(0);
            gl::Uniform1i(self.skybox_location, 0);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::DepthFunc(gl::LESS);
        }
    }
}
