use nalgebra::{Matrix4, Vector3};
use nalgebra_glm as glm;
use sepia::app::*;
use sepia::camera::*;
use sepia::gltf::*;
use sepia::shaderprogram::*;
use sepia::skybox::*;
use std::ptr;

const ONES: &[GLfloat; 1] = &[1.0];

// TODO: Eventually remove default derivations where not necessary
#[derive(Default)]
struct MainState {
    shader_program: ShaderProgram,
    camera: Camera,
    skybox: Skybox,
    scene: Option<GltfScene>,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/gltf/gltf.vs.glsl")
            .fragment_shader_file("assets/shaders/gltf/gltf.fs.glsl")
            .link();
        self.skybox = Skybox::new(&[
            "assets/textures/skyboxes/bluemountains/right.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/left.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/top.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/bottom.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/back.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/front.jpg".to_string(),
        ]);

        self.camera.position_at(&glm::vec3(0.0, 35.0, 60.0));

        self.scene = Some(GltfScene::from_file("assets/models/BoxAnimated.glb"));

        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
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
        let seconds = state_data.current_time;

        // TODO: Trigger animation
        self.scene.as_mut().unwrap().animate(seconds);

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

    // TODO: Create a shader cache and retrieve the shader to use from there.
    //       Need pbr shaders and need basic shaders
    fn render(&mut self, state_data: &mut StateData) {
        let projection = glm::perspective(
            state_data.aspect_ratio,
            50_f32.to_degrees(),
            0.1_f32,
            100000_f32,
        );
        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }
        self.skybox.render(&projection, &self.camera.view_matrix());
        let view = self.camera.view_matrix();

        let scene = self.scene.as_ref().unwrap();
        for mesh in scene.meshes.iter() {
            for primitive_info in mesh.primitives.iter() {
                let material = scene.lookup_material(primitive_info.material_index);
                let pbr = material.pbr_metallic_roughness();
                let base_color = pbr.base_color_factor();
                if !scene.texture_ids.is_empty() {
                    let base_color_index = pbr
                        .base_color_texture()
                        .expect("Couldn't get base color texture!")
                        .texture()
                        .index();
                    unsafe {
                        gl::BindTexture(gl::TEXTURE_2D, scene.texture_ids[base_color_index]);
                    }
                }
                self.shader_program.activate();

                let mvp = projection
                    * view
                    * mesh.transform
                    * Matrix4::new_translation(&Vector3::new(0.0, 0.0, -20.0));
                self.shader_program
                    .set_uniform_matrix4x4("mvp_matrix", mvp.as_slice());
                self.shader_program
                    .set_uniform_vec4("base_color", &base_color);

                primitive_info.vao.bind();
                unsafe {
                    gl::DrawElements(
                        gl::TRIANGLES,
                        primitive_info.num_indices,
                        gl::UNSIGNED_INT,
                        ptr::null(),
                    );
                }
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
