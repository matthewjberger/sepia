use gl::types::*;
use glfw::{Action, Key};
use sepia::app::*;
use sepia::buffer::*;
use sepia::shaderprogram::*;
use sepia::texture::*;
use sepia::vao::*;
use std::ptr;

#[rustfmt::skip]
const VERTICES: &[GLfloat; 12] =
    &[
        0.5,  0.5, 0.0, // 1.0, 1.0, // top right
        0.5, -0.5, 0.0, // 1.0, 0.0, // bottom right
       -0.5, -0.5, 0.0, // 0.0, 0.0, // bottom left
       -0.5,  0.5, 0.0, // 1.0, 1.0  // top left
    ];

#[rustfmt::skip]
const INDICES: &[GLfloat; 6] = &[
    0.0, 1.0, 3.0, // first triangle
    1.0, 2.0, 3.0  // second triangle
];

#[derive(Default)]
struct MainState {
    vao: VertexArrayObject,
    vbo: Buffer,
    ebo: Buffer,
    shader_program: ShaderProgram,
    texture: Texture,
}

impl State for MainState {
    fn initialize(&mut self) {
        // self.texture = Texture::from_file("assets/textures/blue.jpg");
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/texture/texture.vs.glsl")
            .fragment_shader_file("assets/shaders/texture/texture.fs.glsl")
            .link();

        self.vao = VertexArrayObject::new();
        self.vbo = Buffer::new(BufferKind::Array);
        self.ebo = Buffer::new(BufferKind::Element);
        self.vbo.add_data(VERTICES);
        self.vbo.upload(&self.vao, DrawingHint::StaticDraw);
        self.ebo.add_data(INDICES);
        self.ebo.upload(&self.vao, DrawingHint::StaticDraw);
        self.vao.configure_attribute(0, 3, 3, 0);
        // self.vao.configure_attribute(0, 3, 5, 0);
        // self.vao.configure_attribute(1, 2, 5, 3);
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
        // self.texture.bind(0);
        self.vao.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::FLOAT, ptr::null());
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
