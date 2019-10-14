use nalgebra_glm as glm;
use sepia::app::*;
use sepia::camera::*;
use sepia::shader::*;
use sepia::texture::*;
use std::{mem, ptr};

const ONES: &[GLfloat; 1] = &[1.0];

#[rustfmt::skip]
const VERTEX_POSITIONS: &[GLfloat; 108] =
    &[
       -1.0,  1.0, -1.0,
       -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
       -1.0,  1.0, -1.0,

        1.0, -1.0, -1.0,
        1.0, -1.0,  1.0,
        1.0,  1.0, -1.0,

        1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0, -1.0,

        1.0, -1.0,  1.0,
       -1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,

       -1.0, -1.0,  1.0,
       -1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,

       -1.0, -1.0,  1.0,
       -1.0, -1.0, -1.0,
       -1.0,  1.0,  1.0,

       -1.0, -1.0, -1.0,
       -1.0,  1.0, -1.0,
       -1.0,  1.0,  1.0,

       -1.0, -1.0,  1.0,
        1.0, -1.0,  1.0,
        1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
       -1.0, -1.0, -1.0,
       -1.0, -1.0,  1.0,

       -1.0,  1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0,  1.0,  1.0,

        1.0,  1.0,  1.0,
       -1.0,  1.0,  1.0,
       -1.0,  1.0, -1.0
    ];

#[derive(Default)]
struct MainState {
    vao: u32,
    vbo: u32,
    shader_program: ShaderProgram,
    camera: Camera,
    projection_matrix_location: i32,
    view_matrix_location: i32,
    skybox_location: i32,
    texture: Texture,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader("assets/shaders/skybox/skybox.vs.glsl")
            .fragment_shader("assets/shaders/skybox/skybox.fs.glsl")
            .link();

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_POSITIONS.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                VERTEX_POSITIONS.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as i32,
                ptr::null(),
            );

            // gl::Enable(gl::CULL_FACE);
            // gl::FrontFace(gl::CW);

            // gl::Enable(gl::DEPTH_TEST);
            // gl::DepthFunc(gl::LEQUAL);
        }

        self.projection_matrix_location = self.shader_program.uniform_location("projection");
        self.view_matrix_location = self.shader_program.uniform_location("view");
        self.skybox_location = self.shader_program.uniform_location("skybox");
        let paths = &[
            "assets/textures/skyboxes/mountains/right.tga".to_string(),
            "assets/textures/skyboxes/mountains/left.tga".to_string(),
            "assets/textures/skyboxes/mountains/bottom.tga".to_string(),
            "assets/textures/skyboxes/mountains/top.tga".to_string(),
            "assets/textures/skyboxes/mountains/back.tga".to_string(),
            "assets/textures/skyboxes/mountains/front.tga".to_string(),
        ];
        self.texture = Texture::cubemap_from_files(paths);
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

        self.shader_program.activate();

        let view_matrix = glm::mat3_to_mat4(&glm::mat4_to_mat3(&glm::convert(
            self.camera.view_matrix().to_homogeneous(),
        )));

        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);

            gl::BindVertexArray(self.vao);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::UniformMatrix4fv(
                self.projection_matrix_location,
                1,
                gl::FALSE,
                projection.as_ptr(),
            );

            gl::UniformMatrix4fv(
                self.view_matrix_location,
                1,
                gl::FALSE,
                view_matrix.as_slice().as_ptr(),
            );

            self.texture.bind(0);
            gl::Uniform1i(self.skybox_location, 0);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::DepthFunc(gl::LESS);
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
