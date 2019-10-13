use crate::shader::*;
use crate::texture::*;
use na::Matrix4;
use nalgebra as na;
use std::{mem, ptr};

// TODO: Make common primitive geometry file (with indices)
#[rustfmt::skip]
const VERTICES: &[GLfloat; 108] =
&[
    // Positions
    -1.0,  1.0, -1.0,
    -1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
     1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,

    -1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,
    -1.0, -1.0,  1.0,

     1.0, -1.0, -1.0,
     1.0, -1.0,  1.0,
     1.0,  1.0,  1.0,
     1.0,  1.0,  1.0,
     1.0,  1.0, -1.0,
     1.0, -1.0, -1.0,

    -1.0, -1.0, 1.0,
    -1.0,  1.0, 1.0,
     1.0,  1.0, 1.0,
     1.0,  1.0, 1.0,
     1.0, -1.0, 1.0,
    -1.0, -1.0, 1.0,

    -1.0, 1.0, -1.0,
     1.0, 1.0, -1.0,
     1.0, 1.0,  1.0,
     1.0, 1.0,  1.0,
    -1.0, 1.0,  1.0,
    -1.0, 1.0, -1.0,

    -1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
     1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0
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
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(0);
        }

        skybox.projection_matrix_location = shader_program.uniform_location("projection");
        skybox.view_matrix_location = shader_program.uniform_location("view");
        skybox.skybox_location = shader_program.uniform_location("skybox");

        skybox.texture = Texture::cubemap_from_files(paths);

        skybox
    }

    pub fn render(&self, projection_matrix: &Matrix4<f32>, view_matrix: &Matrix4<f32>) {
        self.shader_program.activate();

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::UniformMatrix4fv(
                self.projection_matrix_location,
                1,
                gl::FALSE,
                (*view_matrix).as_ptr(),
            );
            gl::UniformMatrix4fv(
                self.view_matrix_location,
                1,
                gl::FALSE,
                (*projection_matrix).as_ptr(),
            );

            gl::BindVertexArray(self.vao);

            self.texture.bind(0);
            gl::Uniform1fv(self.skybox_location, 1, ptr::null());

            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::DepthFunc(gl::LESS);
        }
    }
}
