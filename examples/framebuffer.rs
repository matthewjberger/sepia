use gl::types::*;
use glfw::{Action, Key};
use sepia::app::*;
use sepia::buffer::*;
use sepia::framebuffer::*;
use sepia::shaderprogram::*;
use sepia::texture::*;
use sepia::vao::*;
use std::ptr;

#[rustfmt::skip]
const VERTICES: &[GLfloat; 15] =
    &[
       -0.5, -0.5, 0.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 1.0, 0.0,
        0.0,  0.5, 0.0, 0.5, 1.0
    ];

#[rustfmt::skip]
const QUAD_VERTICES: &[GLfloat; 20] =
    &[
       -1.0,  1.0, 0.0, 0.0, 1.0, // top left
        1.0,  1.0, 0.0, 1.0, 1.0, // top right
       -1.0, -1.0, 0.0, 0.0, 0.0, // bottom left
        1.0, -1.0, 0.0, 1.0, 0.0, // bottom right
    ];

#[rustfmt::skip]
const INDICES: &[GLuint; 6] = &[
    1, 2, 0, // first triangle
    2, 1, 3, // second triangle
];

#[derive(Default)]
struct MainState {
    vao: VertexArrayObject,
    vbo: Buffer,
    shader_program: ShaderProgram,
    texture: Texture,
    screen_vao: VertexArrayObject,
    screen_vbo: Buffer,
    screen_ebo: Buffer,
    screen_program: ShaderProgram,
    fbo: Framebuffer,
}

impl MainState {
    fn load_shaders(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/texture/texture.vs.glsl")
            .fragment_shader_file("assets/shaders/texture/texture.fs.glsl")
            .link();

        self.screen_program = ShaderProgram::new();
        self.screen_program
            .vertex_shader_file("assets/shaders/texture/screen.vs.glsl")
            .fragment_shader_file("assets/shaders/texture/screen.fs.glsl")
            .link();
    }
}

impl State for MainState {
    fn initialize(&mut self) {
        self.texture = Texture::from_file("assets/textures/blue.jpg");
        self.load_shaders();

        self.vao = VertexArrayObject::new();
        self.vbo = Buffer::new(BufferKind::Array);
        self.vbo.add_data(VERTICES);
        self.vbo.upload(&self.vao, DrawingHint::StaticDraw);
        self.vao.configure_attribute(0, 3, 5, 0);
        self.vao.configure_attribute(1, 2, 5, 3);

        self.screen_vao = VertexArrayObject::new();
        self.screen_vbo = Buffer::new(BufferKind::Array);
        self.screen_ebo = Buffer::new(BufferKind::Element);
        self.screen_vbo.add_data(QUAD_VERTICES);
        self.screen_vbo
            .upload(&self.screen_vao, DrawingHint::StaticDraw);
        self.screen_ebo.add_data(INDICES);
        self.screen_ebo
            .upload(&self.screen_vao, DrawingHint::StaticDraw);
        self.screen_vao.configure_attribute(0, 3, 5, 0);
        self.screen_vao.configure_attribute(1, 2, 5, 3);

        self.fbo = Framebuffer::new();
        self.fbo.create_with_texture(200, 200);
        self.fbo.add_depth_buffer();
    }

    fn handle_events(&mut self, state_data: &mut StateData, event: &glfw::WindowEvent) {
        match *event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                state_data.window.set_should_close(true);
            }
            glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => {
                self.load_shaders();
            }
            _ => (),
        }
    }

    fn render(&mut self, _: &mut StateData) {
        // Use the framebuffer
        self.fbo.bind();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Render the scene to the framebuffer
        self.shader_program.activate();
        self.texture.bind(0);
        self.vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Use the default framebuffer (render to the screen)
        Framebuffer::bind_default_framebuffer();
        self.fbo.color_texture().bind(0);
        self.screen_vao.bind();
        self.screen_program.activate();
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(
                gl::TRIANGLES,
                INDICES.len() as i32,
                self.screen_ebo.type_representation(),
                ptr::null(),
            );
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
