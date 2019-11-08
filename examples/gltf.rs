use nalgebra_glm as glm;
use petgraph::{prelude::*, visit::Dfs};
use sepia::app::*;
use sepia::{camera::*, gltf::*, shaderprogram::*, skybox::*};
use std::ptr;

const ONES: &[GLfloat; 1] = &[1.0];

// TODO: Eventually remove default derivations where not necessary
// TODO: Add gl::Delete calls
#[derive(Default)]
struct MainState {
    shader_program: ShaderProgram,
    lamp_program: ShaderProgram,
    camera: Camera,
    skybox: Skybox,
    asset: Option<GltfAsset>,
    animation_time: f32,
    fbo: u32,
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

        self.asset = Some(GltfAsset::from_file("assets/models/Box.glb"));

        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
        }

        let mut rbo = 0;
        let mut texcolorbuffer = 0;
        unsafe {
            // Create an fbo
            gl::GenFramebuffers(1, &mut self.fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

            // Create a texture to use as the colorbuffer
            gl::GenTextures(1, &mut texcolorbuffer);
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
                texcolorbuffer,
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
            80_f32.to_radians(),
            0.1_f32,
            100000_f32,
        );
        unsafe {
            gl::ClearBufferfv(gl::DEPTH, 0, ONES as *const f32);
        }

        let view = self.camera.view_matrix();

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

                    let current_transform =
                        transform_indices
                            .iter()
                            .fold(glm::Mat4::identity(), |transform, index| {
                                transform
                                    * graph[*index].transform
                                    * graph[*index].animation_transform.matrix()
                            });

                    let transform = current_transform
                        * graph[node_index].transform
                        * graph[node_index].animation_transform.matrix();

                    // If the node has children, store the index for children to use
                    if outgoing_walker.next(&graph).is_some() {
                        transform_indices.push(node_index);
                    }

                    // Render with the given transform
                    if let Some(mesh) = graph[node_index].mesh.as_ref() {
                        for primitive_info in mesh.primitives.iter() {
                            if let Some(material_index) = primitive_info.material_index {
                                let material = asset.lookup_material(material_index);
                                let pbr = material.pbr_metallic_roughness();
                                let base_color = pbr.base_color_factor();
                                if !asset.texture_ids.is_empty() {
                                    if let Some(base_color_texture_info) = pbr.base_color_texture()
                                    {
                                        unsafe {
                                            gl::BindTexture(
                                                gl::TEXTURE_2D,
                                                asset.texture_ids
                                                    [base_color_texture_info.texture().index()],
                                            );
                                        }
                                    }

                                    self.shader_program
                                        .set_uniform_vec4("base_color", &base_color);
                                }
                            }

                            // TODO: Compute normal matrix

                            // Light properties
                            // let lamp_color = glm::vec3(1.0, 0.5, 0.31);
                            let lamp_color = glm::vec3(
                                1.0 * state_data.current_time.sin(),
                                0.75,
                                1.0 * state_data.current_time.cos(),
                            );
                            let lamp_position = glm::vec3(
                                2.0 * state_data.current_time.sin(),
                                2.0 * state_data.current_time.sin()
                                    + 2.0 * state_data.current_time.cos(),
                                2.0 * state_data.current_time.cos(),
                            );

                            // Draw the model
                            {
                                self.shader_program.activate();
                                self.shader_program
                                    .set_uniform_vec3("light_pos", lamp_position.as_slice());
                                self.shader_program
                                    .set_uniform_vec3("light_color", lamp_color.as_slice());
                                self.shader_program
                                    .set_uniform_vec3("view_pos", self.camera.position.as_slice());
                                self.shader_program
                                    .set_uniform_matrix4x4("model", transform.as_slice());
                                self.shader_program
                                    .set_uniform_matrix4x4("view", view.as_slice());
                                self.shader_program
                                    .set_uniform_matrix4x4("projection", projection.as_slice());
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

                            {
                                // Draw the same model, but as a lamp
                                let lamp_mvp = projection
                                    * view
                                    * glm::translate(&glm::Mat4::identity(), &lamp_position)
                                    * glm::scale(
                                        &glm::Mat4::identity(),
                                        &glm::vec3(0.25, 0.25, 0.25),
                                    )
                                    * transform;
                                self.lamp_program.activate();
                                self.lamp_program
                                    .set_uniform_vec3("lamp_color", lamp_color.as_slice());
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

        self.skybox.render(&projection, &view);
    }
}

fn main() {
    let mut state = MainState::default();
    let mut state_machine: Vec<&mut dyn State> = Vec::new();
    state_machine.push(&mut state);
    App::new(state_machine).run();
}
