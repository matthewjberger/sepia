use gl::types::*;
use glfw::{Action, Key};
use sepia::app::*;
use sepia::buffer::*;
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
    texcolorbuffer: u32,
    fbo: u32,
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

        let mut rbo = 0;
        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            // Create an fbo
            gl::GenFramebuffers(1, &mut self.fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

            // Create a texture to use as the colorbuffer
            gl::GenTextures(1, &mut self.texcolorbuffer);
            gl::BindTexture(gl::TEXTURE_2D, self.texcolorbuffer);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                800,
                600,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            // Attach the colorbuffer to the fbo
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.texcolorbuffer,
                0,
            );

            // Create a renderbuffer
            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, 800, 600);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete!")
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
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
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.shader_program.activate();
        self.texture.bind(0);
        self.vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        self.screen_vao.bind();
        self.screen_program.activate();
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // back to default
            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindTexture(gl::TEXTURE_2D, self.texcolorbuffer);
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
