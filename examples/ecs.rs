pub use gl::types::*;
pub use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use sepia::buffer::*;
use sepia::shaderprogram::*;
use sepia::texture::*;
use sepia::vao::*;
use specs::prelude::*;
use specs::Component;
use std::cmp;

const BACKGROUND_COLOR: &[GLfloat; 4] = &[0.25, 0.25, 0.25, 1.0];

#[rustfmt::skip]
const VERTICES: &[GLfloat; 15] =
    &[
       -0.5, -0.5, 0.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 1.0, 0.0,
        0.0,  0.5, 0.0, 0.5, 1.0
    ];

#[derive(Default)]
struct DeltaTime(f32);

#[derive(Default)]
struct AspectRatio(f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct MeshInfo {
    vertices: Vec<GLfloat>,
}

#[derive(Default)]
struct RenderSystem {
    vao: VertexArrayObject,
    vbo: Buffer,
    shader_program: ShaderProgram,
    texture: Texture,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        Read<'a, AspectRatio>,
        ReadStorage<'a, MeshInfo>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (delta_time, aspect_ratio, mesh_info) = data;
        let delta_time = delta_time.0;
        let aspect_ratio = aspect_ratio.0;

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, BACKGROUND_COLOR as *const f32);
        }

        self.shader_program.activate();
        self.texture.bind(0);
        self.vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        // TODO: Make a material/shader cache and have mesh component store mesh data and material data
        self.texture = Texture::from_file("assets/textures/blue.jpg");
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/texture/texture.vs.glsl")
            .fragment_shader_file("assets/shaders/texture/texture.fs.glsl")
            .link();

        self.vao = VertexArrayObject::new();
        self.vbo = Buffer::new(BufferKind::Array);
        self.vbo.add_data(VERTICES);
        self.vbo.upload(&self.vao, DrawingHint::StaticDraw);
        self.vao.configure_attribute(0, 3, 5, 0);
        self.vao.configure_attribute(1, 2, 5, 3);
    }
}

fn main() {
    // Windowing
    let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = context
        .create_window(800, 600, "Sepia", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Create world and add entities
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(RenderSystem::default())
        .build();
    dispatcher.setup(&mut world);
    world
        .create_entity()
        .with(MeshInfo {
            vertices: VERTICES.to_vec(),
        })
        .build();

    let mut current_time = context.get_time();
    let mut last_frame_time = current_time;

    while !window.should_close() {
        // Calculate the aspect ratio
        let (window_width, window_height) = window.get_size();
        let aspect_ratio = window_width as f32 / cmp::max(0, window_height) as f32;
        *world.write_resource::<AspectRatio>() = AspectRatio(aspect_ratio);

        // Compute delta time
        current_time = context.get_time();
        let delta_time = (current_time - last_frame_time) as f32;
        *world.write_resource::<DeltaTime>() = DeltaTime(delta_time);
        last_frame_time = current_time;

        context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                _ => (),
            }

            // Handle events
        }

        // Update

        // Render
        dispatcher.dispatch(&mut world);
        window.swap_buffers();
        world.maintain();
    }
}
