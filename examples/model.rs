use na::Matrix4;
use nalgebra as na;
use nalgebra_glm as glm;
use sepia::app::*;
use sepia::buffer::*;
use sepia::camera::*;
use sepia::shaderprogram::*;
use sepia::skybox::*;
use sepia::texture::*;
use sepia::vao::*;
use std::path::Path;

const ONES: &[GLfloat; 1] = &[1.0];

#[derive(Default)]
struct MainState {
    camera: Camera,
    shader_program: ShaderProgram,
    data: Vec<Mesh>,
    texture: Texture,
    skybox: Skybox,
}

#[derive(Default)]
pub struct Mesh {
    vao: VertexArrayObject,
    vbo: Buffer,
    ebo: Buffer,
}

impl Mesh {
    fn new() -> Mesh {
        Mesh {
            vao: VertexArrayObject::new(),
            vbo: Buffer::new(BufferKind::Array),
            ebo: Buffer::new(BufferKind::Element),
        }
    }
}

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

        // self.texture = Texture::from_file("assets/textures/blue.jpg");
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/model/model.vs.glsl")
            .fragment_shader_file("assets/shaders/model/model.fs.glsl")
            .link();

        let (models, _) =
            tobj::load_obj(&Path::new("assets/models/wolf/Wolf_One_obj.obj")).unwrap();
        self.data = Vec::new();
        for model in &models {
            let mesh = &model.mesh;
            let mut mesh_data = Mesh::new();
            for i in 0..mesh.positions.len() / 3 {
                // pos = [x; y; z]
                mesh_data.vbo.add_data(&[
                    mesh.positions[i * 3] as GLfloat,
                    mesh.positions[i * 3 + 1] as GLfloat,
                    mesh.positions[i * 3 + 2] as GLfloat,
                ]);

                // normal = [x; y; z]
                // if !mesh.normals.is_empty() {
                //     mesh_data.vbo.add_data(&[
                //         mesh.normals[i * 3] as GLfloat,
                //         mesh.normals[i * 3 + 1] as GLfloat,
                //         mesh.normals[i * 3 + 2] as GLfloat,
                //     ]);
                // }

                // texcoord = [u; v];
                // if !mesh.texcoords.is_empty() {
                //     mesh_data.vbo.add_data(&[
                //         mesh.texcoords[i * 2] as GLfloat,
                //         mesh.texcoords[i * 2 + 1] as GLfloat,
                //     ]);
                // }
            }
            mesh_data
                .vbo
                .upload(&mesh_data.vao, DrawingHint::StaticDraw);

            mesh_data.ebo.add_data(
                mesh.indices
                    .iter()
                    .map(|x| *x as GLfloat)
                    .collect::<Vec<f32>>()
                    .as_slice(),
            );
            mesh_data
                .ebo
                .upload(&mesh_data.vao, DrawingHint::StaticDraw);

            mesh_data.vao.configure_attribute(0, 3, 3, 0);
            // mesh_data.vao.configure_attribute(1, 2, 5, 3);
            // mesh_data.vao.configure_attribute(1, 3, 8, 3);
            // mesh_data.vao.configure_attribute(2, 2, 8, 6);
            self.data.push(mesh_data);
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

        let modelview = self.camera.view_matrix()
            * Matrix4::new_rotation(glm::vec3(
                180_f32.to_radians(),
                (state_data.current_time as f32 * 45_f32).to_radians(),
                0.0,
            ));

        self.shader_program.activate();
        self.shader_program
            .set_uniform_matrix4x4("modelview_matrix", modelview.as_slice());
        self.shader_program
            .set_uniform_matrix4x4("projection_matrix", projection.as_slice());

        for mesh in self.data.iter() {
            mesh.vao.bind();
            // self.texture.bind(0);

            unsafe {
                gl::Enable(gl::CULL_FACE);
                gl::FrontFace(gl::CCW);
                gl::Enable(gl::DEPTH_TEST);
                gl::DepthFunc(gl::LEQUAL);
                gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.ebo.len as i32,
                    gl::UNSIGNED_INT,
                    0 as *const GLvoid,
                );
            }
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
