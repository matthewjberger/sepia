use na::Matrix4;
use nalgebra as na;
use nalgebra_glm as glm;
use sepia::app::*;
use sepia::camera::*;
use sepia::shaderprogram::*;
use sepia::skybox::*;
use sepia::texture::*;
use std::mem;
use std::path::Path;
use std::ptr;

const ONES: &[GLfloat; 1] = &[1.0];

#[derive(Default)]
struct MainState {
    camera: Camera,
    vao: u32,
    vbo: u32,
    shader_program: ShaderProgram,
    data: Vec<GLfloat>,
    texture: Texture,
    skybox: Skybox,
}

// #[derive(Copy, Clone)]
// struct Vertex {
//     position: [f32; 3],
//     normal: [f32; 3],
//     color_diffuse: [f32; 3],
//     color_specular: [f32; 4],
// }

// struct Mesh {
//     vao: u32,
//     vbo: u32,
//     ibo: u32,
// }
// type Model = Vec<Mesh>;

// impl Model {
//     fn load_file(path: &str) -> Self {
//     }
// }

impl State for MainState {
    fn initialize(&mut self) {
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CCW);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }

        self.skybox = Skybox::new(&[
            "assets/textures/skyboxes/mountains/right.tga".to_string(),
            "assets/textures/skyboxes/mountains/left.tga".to_string(),
            "assets/textures/skyboxes/mountains/bottom.tga".to_string(),
            "assets/textures/skyboxes/mountains/top.tga".to_string(),
            "assets/textures/skyboxes/mountains/back.tga".to_string(),
            "assets/textures/skyboxes/mountains/front.tga".to_string(),
        ]);

        self.texture = Texture::from_file("assets/textures/blue.jpg");
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/model/model.vs.glsl")
            .fragment_shader_file("assets/shaders/model/model.fs.glsl")
            .link();

        let (models, _) =
            tobj::load_obj(&Path::new("assets/models/wolf/Wolf_One_obj.obj")).unwrap();
        self.data = Vec::new();
        for model in &models {
            println!("Uploading model: {}", model.name);
            let mesh = &model.mesh;

            for i in &mesh.indices {
                let i = *i as usize;
                // pos = [x; y; z]
                self.data.push(mesh.positions[i * 3]);
                self.data.push(mesh.positions[i * 3 + 1]);
                self.data.push(mesh.positions[i * 3 + 2]);

                if !mesh.normals.is_empty() {
                    // normal = [x; y; z]
                    // self.data.push(mesh.normals[i * 3]);
                    // self.data.push(mesh.normals[i * 3 + 1]);
                    // self.data.push(mesh.normals[i * 3 + 2]);
                }

                if !mesh.texcoords.is_empty() {
                    // texcoord = [u; v];
                    self.data.push(mesh.texcoords[i * 2]);
                    self.data.push(mesh.texcoords[i * 2 + 1]);
                }
            }
        }

        let data_length = (5 * mem::size_of::<GLfloat>()) as i32;

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                self.data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

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

        self.skybox.render(&projection, &self.camera.view_matrix());

        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CCW);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }

        let modelview = self.camera.view_matrix()
            * Matrix4::new_rotation(glm::vec3(
                180_f32.to_radians(),
                (state_data.current_time as f32 * 45_f32).to_radians(),
                0.0,
            ));

        self.shader_program.activate();
        self.texture.bind(0);
        let modelview_matrix_location = self.shader_program.uniform_location("modelview_matrix");
        let projection_matrix_location = self.shader_program.uniform_location("projection_matrix");

        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
            gl::BindVertexArray(self.vao);
            gl::UniformMatrix4fv(
                projection_matrix_location,
                1,
                gl::FALSE,
                projection.as_ptr(),
            );
            gl::UniformMatrix4fv(modelview_matrix_location, 1, gl::FALSE, modelview.as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, self.data.len() as i32);
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
