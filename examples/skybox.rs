use nalgebra_glm as glm;
use sepia::app::*;
use sepia::camera::*;
use sepia::skybox::*;

const ONES: &[GLfloat; 1] = &[1.0];

#[derive(Default)]
struct MainState {
    camera: Camera,
    skybox: Skybox,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.skybox = Skybox::new(&[
            "assets/textures/skyboxes/mountains/right.tga".to_string(),
            "assets/textures/skyboxes/mountains/left.tga".to_string(),
            "assets/textures/skyboxes/mountains/bottom.tga".to_string(),
            "assets/textures/skyboxes/mountains/top.tga".to_string(),
            "assets/textures/skyboxes/mountains/back.tga".to_string(),
            "assets/textures/skyboxes/mountains/front.tga".to_string(),
        ]);
    }

    fn handle_events(&mut self, state_data: &mut StateData, event: &glfw::WindowEvent) {
        match *event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
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
        let projection = glm::perspective(
            state_data.aspect_ratio,
            50_f32.to_degrees(),
            0.1_f32,
            1000_f32,
        );

        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
            self.skybox.render(&projection, &self.camera.view_matrix());
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
