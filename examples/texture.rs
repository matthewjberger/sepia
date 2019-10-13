use gl::types::*;
use glfw::{Action, Key};
use sepia::app::*;
use sepia::shader::*;
use sepia::texture::*;
use std::{mem, ptr};

#[rustfmt::skip]
const VERTICES: &[GLfloat; 15] =
    &[
       -0.5, -0.5, 0.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 1.0, 0.0,
        0.0,  0.5, 0.0, 0.5, 1.0
    ];

#[derive(Default)]
struct MainState {
    vao: u32,
    vbo: u32,
    shader_program: ShaderProgram,
    texture: Texture,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.texture = Texture::from_file("assets/textures/blue.jpg");
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/texture/texture.vs.glsl")
            .fragment_shader("assets/shaders/texture/texture.fs.glsl")
            .link();

        let data_length = (5 * mem::size_of::<GLfloat>()) as i32;

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

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, data_length, ptr::null());

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                data_length,
                (3 * mem::size_of::<GLfloat>()) as *const GLvoid,
            );
        }
    }

    fn handle_events(&mut self, state_data: &mut StateData, event: &glfw::WindowEvent) {
        match *event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                state_data.window.set_should_close(true);
            }
            _ => (),
        }
    }

    fn render(&mut self, _: &mut StateData) {
        self.shader_program.activate();
        self.texture.bind(0);
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
