use gl::types::*;
use glfw::{Action, Key};
use sepia::app::*;
use sepia::buffer::*;
use sepia::shader::*;
use sepia::texture::*;
use sepia::vao::*;
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
    vao: VertexArrayObject,
    vbo: Buffer,
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

        self.vao = VertexArrayObject::new();
        self.vbo = Buffer::new();
        self.vbo.add_data(VERTICES);
        self.vbo.upload(&self.vao, DrawingHint::StaticDraw);

        let data_length = (5 * mem::size_of::<GLfloat>()) as i32;

        unsafe {
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
        self.vao.bind();
        unsafe {
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
