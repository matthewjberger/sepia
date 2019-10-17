use na::{Matrix4, Vector3};
use nalgebra as na;
use nalgebra_glm as glm;
use sepia::app::*;
use sepia::buffer::*;
use sepia::camera::*;
use sepia::shader::*;
use sepia::shaderprogram::*;
use sepia::vao::*;

const ONES: &[GLfloat; 1] = &[1.0];

#[rustfmt::skip]
const VERTEX_POSITIONS: &[GLfloat; 108] =
    &[
       -0.25,  0.25, -0.25,
       -0.25, -0.25, -0.25,
        0.25, -0.25, -0.25,

        0.25, -0.25, -0.25,
        0.25,  0.25, -0.25,
       -0.25,  0.25, -0.25,

        0.25, -0.25, -0.25,
        0.25, -0.25,  0.25,
        0.25,  0.25, -0.25,

        0.25, -0.25,  0.25,
        0.25,  0.25,  0.25,
        0.25,  0.25, -0.25,

        0.25, -0.25,  0.25,
       -0.25, -0.25,  0.25,
        0.25,  0.25,  0.25,

       -0.25, -0.25,  0.25,
       -0.25,  0.25,  0.25,
        0.25,  0.25,  0.25,

       -0.25, -0.25,  0.25,
       -0.25, -0.25, -0.25,
       -0.25,  0.25,  0.25,

       -0.25, -0.25, -0.25,
       -0.25,  0.25, -0.25,
       -0.25,  0.25,  0.25,

       -0.25, -0.25,  0.25,
        0.25, -0.25,  0.25,
        0.25, -0.25, -0.25,

        0.25, -0.25, -0.25,
       -0.25, -0.25, -0.25,
       -0.25, -0.25,  0.25,

       -0.25,  0.25, -0.25,
        0.25,  0.25, -0.25,
        0.25,  0.25,  0.25,

        0.25,  0.25,  0.25,
       -0.25,  0.25,  0.25,
       -0.25,  0.25, -0.25
    ];

#[derive(Default)]
struct MainState {
    vao: VertexArrayObject,
    vbo: Buffer,
    shader_program: ShaderProgram,
    camera: Camera,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/spinny-cube/spinny-cube.vs.glsl")
            .fragment_shader("assets/shaders/spinny-cube/spinny-cube.fs.glsl")
            .link();

        self.vao = VertexArrayObject::new();
        self.vbo = Buffer::new();
        self.vbo.add_data(VERTEX_POSITIONS);
        self.vbo.upload(&self.vao, DrawingHint::StaticDraw);
        self.vao.configure_attribute(0, 3, 3, 0);

        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CW);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
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
        self.vao.bind();
        self.shader_program.activate();
        let modelview_matrix_location = self.shader_program.uniform_location("modelview_matrix");
        let projection_matrix_location = self.shader_program.uniform_location("projection_matrix");

        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
            gl::UniformMatrix4fv(
                projection_matrix_location,
                1,
                gl::FALSE,
                projection.as_ptr(),
            );
        }
        for cube_id in 0..24 {
            let factor: f32 = cube_id as f32 + (state_data.current_time as f32 * 0.3);
            let modelview = self.camera.view_matrix()
                * Matrix4::new_translation(&Vector3::new(0.0, 0.0, -4.0))
                * Matrix4::new_rotation(Vector3::new(
                    0.0,
                    (state_data.current_time as f32 * 45_f32).to_radians(),
                    (state_data.current_time as f32 * 21_f32).to_radians(),
                ))
                * Matrix4::new_translation(&Vector3::new(
                    (2.1 * factor).sin() * 2.0,
                    (1.7 * factor).cos() * 2.0,
                    (1.3 * factor).sin() * (1.5 * factor).cos() * 2.0,
                ));

            unsafe {
                gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        let modelview = self.camera.view_matrix()
            * Matrix4::new_translation(&Vector3::new(0.0, 10.0, 0.0))
            * Matrix4::new_nonuniform_scaling(&Vector3::new(100.0, 0.2, 100.0));

        unsafe {
            gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
