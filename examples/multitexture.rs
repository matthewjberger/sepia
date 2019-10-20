use gl::types::*;
use glfw::{Action, Key};
use sepia::app::*;
use sepia::buffer::*;
use sepia::shaderprogram::*;
use sepia::texture::*;
use sepia::vao::*;

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
    texture2: Texture,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.texture = Texture::from_file("assets/textures/green.jpg");
        self.texture2 = Texture::from_file("assets/textures/wood.jpg");
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/multitexture/multitexture.vs.glsl")
            .fragment_shader_file("assets/shaders/multitexture/multitexture.fs.glsl")
            .link();

        self.vao = VertexArrayObject::new();
        self.vbo = Buffer::new(BufferKind::Array);
        self.vbo.add_data(VERTICES);
        self.vbo.upload(&self.vao, DrawingHint::StaticDraw);
        self.vao.configure_attribute(0, 3, 5, 0);
        self.vao.configure_attribute(1, 2, 5, 3);
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
        self.shader_program.set_uniform_integer("texture1", 0);

        self.texture.bind(0);
        self.shader_program.set_uniform_integer("texture2", 1);

        self.texture2.bind(1);
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
