use na::{Matrix4, Vector3};
use nalgebra as na;
use nalgebra_glm as glm;
use sepia::app::*;
use sepia::camera::*;
use sepia::model::*;
use sepia::shaderprogram::*;
use sepia::skybox::*;
use sepia::texture::*;

const ONES: &[GLfloat; 1] = &[1.0];

#[derive(Default)]
struct MainState {
    shader_program: ShaderProgram,
    model: Model,
    camera: Camera,
    skybox: Skybox,
    texture: Texture,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.texture = Texture::from_file("assets/textures/blue.jpg");
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/model/model.vs.glsl")
            .fragment_shader_file("assets/shaders/model/model.fs.glsl")
            .link();
        self.model = Model::from_file("assets/models/nanosuit/nanosuit.obj");
        self.skybox = Skybox::new(&[
            "assets/textures/skyboxes/bluemountains/right.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/left.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/top.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/bottom.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/back.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/front.jpg".to_string(),
        ]);
    }

    fn handle_events(&mut self, state_data: &mut StateData, event: &glfw::WindowEvent) {
        match *event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                state_data.window.set_should_close(true);
            }
            WindowEvent::CursorPos(cursor_x, cursor_y) => {
                let (window_width, window_height) = state_data.window.get_size();
                self.camera.process_mouse_movement(
                    (window_width as f32 / 2.0) - cursor_x as f32,
                    (window_height as f32 / 2.0) - cursor_y as f32,
                );
            }

            _ => (),
        }
    }

    fn update(&mut self, state_data: &mut StateData) {
        if state_data.window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.camera
                .translate(CameraDirection::Forward, state_data.delta_time);
        }
        if state_data.window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.camera
                .translate(CameraDirection::Left, state_data.delta_time);
        }
        if state_data.window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.camera
                .translate(CameraDirection::Backward, state_data.delta_time);
        }
        if state_data.window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.camera
                .translate(CameraDirection::Right, state_data.delta_time);
        }

        let (window_width, window_height) = state_data.window.get_size();
        state_data.window.set_cursor_pos(
            f64::from(window_width) / 2.0,
            f64::from(window_height) / 2.0,
        );
        state_data.window.set_cursor_mode(CursorMode::Disabled);
    }

    fn render(&mut self, state_data: &mut StateData) {
        self.texture.bind(0);
        let projection = glm::perspective(
            state_data.aspect_ratio,
            50_f32.to_degrees(),
            0.1_f32,
            1000_f32,
        );
        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }
        self.skybox.render(&projection, &self.camera.view_matrix());
        let view = self.camera.view_matrix();
        let mvp = projection * view * Matrix4::new_translation(&Vector3::new(0.0, 0.0, -4.0));
        self.shader_program.activate();
        self.shader_program
            .set_uniform_matrix4x4("mvp_matrix", mvp.as_slice());
        self.model.render();
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
