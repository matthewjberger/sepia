use nalgebra_glm as glm;
use petgraph::{prelude::*, visit::Dfs};
use sepia::app::*;
use sepia::{camera::*, gltf::*, shaderprogram::*, skybox::*};
use std::ptr;

const ONES: &[GLfloat; 1] = &[1.0];

// TODO: Eventually remove default derivations where not necessary
#[derive(Default)]
struct MainState {
    shader_program: ShaderProgram,
    lamp_program: ShaderProgram,
    camera: Camera,
    skybox: Skybox,
    asset: Option<GltfAsset>,
    animation_time: f32,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/gltf/lit.vs.glsl")
            .fragment_shader_file("assets/shaders/gltf/lit.fs.glsl")
            .link();
        self.lamp_program = ShaderProgram::new();
        self.lamp_program
            .vertex_shader_file("assets/shaders/gltf/lamp.vs.glsl")
            .fragment_shader_file("assets/shaders/gltf/lamp.fs.glsl")
            .link();
        self.skybox = Skybox::new(&[
            "assets/textures/skyboxes/bluemountains/right.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/left.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/top.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/bottom.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/back.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/front.jpg".to_string(),
        ]);

        self.asset = Some(GltfAsset::from_file("assets/models/RiggedSimple.glb"));

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
        // Update animation transforms
        // let seconds = state_data.current_time;
        let asset = self.asset.as_mut().unwrap();
        if !asset.animations.is_empty() {
            self.asset.as_mut().unwrap().animate(self.animation_time);
        }

        if state_data.window.get_key(glfw::Key::Left) == glfw::Action::Press {
            self.animation_time -= 0.01;
            if self.animation_time < 0.0 {
                self.animation_time = 0.0;
            }
        }

        if state_data.window.get_key(glfw::Key::Right) == glfw::Action::Press {
            self.animation_time += 0.01;
        }

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
            90_f32.to_radians(),
            0.1_f32,
            100000_f32,
        );
        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }

        let view = self.camera.view_matrix();
        self.skybox.render(&projection, &view);

        // Render the asset's scene graph
        let asset = self.asset.as_mut().expect("Couldn't get asset!");
        for scene in asset.scenes.iter() {
            for graph in scene.node_graphs.iter() {
                let mut transform_indices: Vec<NodeIndex> = Vec::new();
                let mut dfs = Dfs::new(&graph, NodeIndex::new(0));
                while let Some(node_index) = dfs.next(&graph) {
                    let mut incoming_walker =
                        graph.neighbors_directed(node_index, Incoming).detach();
                    let mut outgoing_walker =
                        graph.neighbors_directed(node_index, Outgoing).detach();

                    if let Some(parent) = incoming_walker.next_node(&graph) {
                        while let Some(last_index) = transform_indices.last() {
                            if *last_index == parent {
                                break;
                            }
                            // Discard indices for transforms that are no longer needed
                            transform_indices.pop();
                        }
                    }

                    transform_indices.push(node_index);
                    let global_transform =
                        transform_indices
                            .iter()
                            .fold(glm::Mat4::identity(), |transform, index| {
                                transform
                                    * graph[*index].local_transform
                                    * graph[*index].animation_transform.matrix()
                            });

                    // If the node has no children, don't store the index
                    if !outgoing_walker.next(&graph).is_some() {
                        transform_indices.pop();
                    }

                    // Skinning
                    if let Some(skin) = graph[node_index].skin.as_ref() {
                        for (index, joint) in skin.joints.iter().enumerate() {
                            // TODO: This must not be correct, fix it
                            let joint_matrix =
                            // Inverse transform of the node the mesh is attached to
                                glm::inverse(&graph[node_index].local_transform) *
                            // Current global transform of the joint node
                                graph[NodeIndex::new(joint.index)].local_transform *
                            // Transform of the joint's inverse bind matrix
                                joint.inverse_bind_matrix;

                            self.shader_program.set_uniform_matrix4x4(
                                &format!("u_jointMatrix[{}]", index),
                                joint_matrix.as_slice(),
                            );
                        }
                    }

                    // Render with the given transform
                    if let Some(mesh) = graph[node_index].mesh.as_ref() {
                        for primitive_info in mesh.primitives.iter() {
                            if let Some(material_index) = primitive_info.material_index {
                                let material = asset.lookup_material(material_index);
                                let pbr = material.pbr_metallic_roughness();
                                //let base_color = glm::Vec4::from(pbr.base_color_factor()).xyz();
                                if !asset.texture_ids.is_empty() {
                                    let base_color_index = pbr
                                        .base_color_texture()
                                        .expect("Couldn't get base color texture!")
                                        .texture()
                                        .index();
                                    unsafe {
                                        gl::BindTexture(
                                            gl::TEXTURE_2D,
                                            asset.texture_ids[base_color_index],
                                        );
                                    }
                                };
                                self.shader_program
                                    .set_uniform_int("material.diffuse_texture", 0);
                                self.shader_program
                                    .set_uniform_float("material.shininess", 32.0);
                            }

                            // Flashlight settings
                            self.shader_program.set_uniform_vec3(
                                "spotlight.position",
                                &self.camera.position.as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "spotlight.direction",
                                &self.camera.front.as_slice(),
                            );
                            self.shader_program
                                .set_uniform_float("spotlight.cutOff", 12.5_f32.to_radians().cos());
                            self.shader_program.set_uniform_float(
                                "spotlight.outerCutOff",
                                17.5_f32.to_radians().cos(),
                            );
                            self.shader_program
                                .set_uniform_float("spotlight.constant", 1.0);
                            self.shader_program
                                .set_uniform_float("spotlight.linear", 0.007);
                            self.shader_program
                                .set_uniform_float("spotlight.quadratic", 0.0002);
                            self.shader_program.set_uniform_vec3(
                                "spotlight.ambient",
                                &glm::vec3(0.3, 0.24, 0.14).as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "spotlight.diffuse",
                                &glm::vec3(0.7, 0.42, 0.26).as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "spotlight.specular",
                                &glm::vec3(0.5, 0.5, 0.5).as_slice(),
                            );

                            // Directional Light
                            self.shader_program.set_uniform_vec3(
                                "directional_light.direction",
                                &glm::vec3(-0.2, -1.0, -0.3).as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "directional_light.ambient",
                                &glm::vec3(0.3, 0.24, 0.14).as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "directional_light.diffuse",
                                &glm::vec3(0.7, 0.42, 0.26).as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "directional_light.specular",
                                &glm::vec3(0.5, 0.5, 0.5).as_slice(),
                            );

                            // Point light 1
                            let point_light_pos1 = glm::vec3(10.0, 15.0, 45.0);
                            let point_light_color1 = glm::vec3(0.0, 1.0, 0.5);
                            let point_light_color1_ambient = point_light_color1 * 0.1;
                            self.shader_program.set_uniform_vec3(
                                "point_lights[0].position",
                                &point_light_pos1.as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "point_lights[0].ambient",
                                &point_light_color1_ambient.as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "point_lights[0].diffuse",
                                &point_light_color1.as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "point_lights[0].specular",
                                &point_light_color1.as_slice(),
                            );
                            self.shader_program
                                .set_uniform_float("point_lights[0].constant", 1.0);
                            self.shader_program
                                .set_uniform_float("point_lights[0].linear", 0.007);
                            self.shader_program
                                .set_uniform_float("point_lights[0].quadratic", 0.0002);

                            // Point light 2
                            let point_light_pos2 = glm::vec3(-10.3, 15.3, -10.0);
                            let point_light_color2 = glm::vec3(1.0, 0.0, 0.0);
                            let point_light_color2_ambient = point_light_color2 * 0.1;
                            self.shader_program.set_uniform_vec3(
                                "point_lights[1].position",
                                &point_light_pos2.as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "point_lights[1].ambient",
                                &point_light_color2_ambient.as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "point_lights[1].diffuse",
                                &point_light_color2.as_slice(),
                            );
                            self.shader_program.set_uniform_vec3(
                                "point_lights[1].specular",
                                &point_light_color2.as_slice(),
                            );
                            self.shader_program
                                .set_uniform_float("point_lights[1].constant", 1.0);
                            self.shader_program
                                .set_uniform_float("point_lights[1].linear", 0.007);
                            self.shader_program
                                .set_uniform_float("point_lights[1].quadratic", 0.0002);

                            self.shader_program
                                .set_uniform_vec3("view_pos", &self.camera.position.as_slice());
                            self.shader_program
                                .set_uniform_matrix4x4("view", view.as_slice());
                            self.shader_program
                                .set_uniform_matrix4x4("projection", projection.as_slice());

                            for row in 0..10 {
                                for column in 0..10 {
                                    self.shader_program.set_uniform_matrix4x4(
                                        "model",
                                        (glm::translate(
                                            &glm::Mat4::identity(),
                                            &glm::vec3(
                                                row as f32 * -10.0,
                                                0.0,
                                                column as f32 * 10.0,
                                            ),
                                        ) * glm::scale(
                                            &glm::Mat4::identity(),
                                            &glm::vec3(6.0, 6.0, 6.0),
                                        ) * global_transform)
                                            .as_slice(),
                                    );

                                    self.shader_program.activate();
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

                            let lamp_mvp = projection
                                * view
                                * glm::translate(&glm::Mat4::identity(), &point_light_pos1.xyz())
                                * glm::scale(&glm::Mat4::identity(), &glm::vec3(2.0, 2.0, 2.0))
                                * global_transform;
                            self.lamp_program.activate();
                            self.lamp_program
                                .set_uniform_vec3("lamp_color", &point_light_color1.as_slice());
                            self.lamp_program
                                .set_uniform_matrix4x4("mvp_matrix", lamp_mvp.as_slice());
                            primitive_info.vao.bind();
                            unsafe {
                                gl::DrawElements(
                                    gl::TRIANGLES,
                                    primitive_info.num_indices,
                                    gl::UNSIGNED_INT,
                                    ptr::null(),
                                );
                            }

                            let lamp_mvp = projection
                                * view
                                * glm::translate(&glm::Mat4::identity(), &point_light_pos2.xyz())
                                * glm::scale(&glm::Mat4::identity(), &glm::vec3(2.0, 2.0, 2.0))
                                * global_transform;
                            self.lamp_program.activate();
                            self.lamp_program
                                .set_uniform_vec3("lamp_color", &point_light_color2.as_slice());
                            self.lamp_program
                                .set_uniform_matrix4x4("mvp_matrix", lamp_mvp.as_slice());
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
        }
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
