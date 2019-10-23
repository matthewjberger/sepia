use gltf::image::Format;
use na::{Matrix4, Vector3};
use nalgebra as na;
use nalgebra_glm as glm;
use sepia::app::*;
use sepia::buffer::*;
use sepia::camera::*;
use sepia::shaderprogram::*;
use sepia::skybox::*;
use sepia::vao::*;
use std::ptr;

const ONES: &[GLfloat; 1] = &[1.0];

#[repr(C)]
pub struct Vertex {
    position: glm::Vec3,
    normal: glm::Vec3,
    tex_coords: glm::Vec2,
}

impl Vertex {
    pub fn new(position: glm::Vec3, normal: glm::Vec3, tex_coords: glm::Vec2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
        }
    }
}

#[derive(Default)]
pub struct Mesh {
    vao: VertexArrayObject,
    vbo: Buffer,
    ibo: Buffer,
    num_indices: i32,
    material_index: i32,
}

#[derive(Default)]
struct MainState {
    shader_program: ShaderProgram,
    camera: Camera,
    skybox: Skybox,
    meshes: Vec<Mesh>,
    textures: Vec<u32>,
    gltf: Option<gltf::Document>,
}

impl State for MainState {
    fn initialize(&mut self) {
        self.shader_program = ShaderProgram::new();
        self.shader_program
            .vertex_shader_file("assets/shaders/model/model.vs.glsl")
            .fragment_shader_file("assets/shaders/model/model.fs.glsl")
            .link();
        self.skybox = Skybox::new(&[
            "assets/textures/skyboxes/bluemountains/right.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/left.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/top.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/bottom.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/back.jpg".to_string(),
            "assets/textures/skyboxes/bluemountains/front.jpg".to_string(),
        ]);

        // Load a gltf model
        let (gltf, buffers, textures) = gltf::import("assets/models/car/scene.gltf").unwrap();

        for texture in textures.iter() {
            // gltf 2.0 only supports 2D texture targets
            let target = gl::TEXTURE_2D;
            let mut texture_id = 0;
            unsafe {
                gl::GenTextures(1, &mut texture_id);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(target, texture_id);
            }
            let pixel_format = match texture.format {
                Format::R8 => gl::RED,
                Format::R8G8 => gl::RG,
                Format::R8G8B8 => gl::RGB,
                Format::R8G8B8A8 => gl::RGBA,
                Format::B8G8R8 => gl::BGR,
                Format::B8G8R8A8 => gl::BGRA,
            };
            unsafe {
                gl::TexImage2D(
                    target,
                    0,
                    pixel_format as i32,
                    texture.width as i32,
                    texture.height as i32,
                    0,
                    pixel_format,
                    gl::UNSIGNED_BYTE,
                    texture.pixels.as_ptr() as *const GLvoid,
                );
                gl::GenerateMipmap(target);
            }
            self.textures.push(texture_id);
        }

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let positions = reader.read_positions().unwrap().collect::<Vec<_>>();
                let normals = reader.read_normals().unwrap().collect::<Vec<_>>();

                let tex_coords = reader
                    .read_tex_coords(0)
                    .map(|read_tex_coords| read_tex_coords.into_f32().collect::<Vec<_>>())
                    .unwrap();

                let indices = reader
                    .read_indices()
                    .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>())
                    .unwrap();

                let mut vertices = Vec::new();
                for (index, position) in positions.iter().enumerate() {
                    let normal = normals[index];
                    let tex_coord = tex_coords[index];
                    vertices.push(Vertex::new(
                        glm::vec3(position[0], position[1], position[2]),
                        glm::vec3(normal[0], normal[1], normal[2]),
                        glm::vec2(tex_coord[0], tex_coord[1]),
                    ));
                }

                let mut mesh = Mesh::default();
                mesh.vao = VertexArrayObject::new();
                mesh.vbo = Buffer::new(BufferKind::Array);
                mesh.ibo = Buffer::new(BufferKind::Element);
                mesh.num_indices = indices.len() as i32;
                mesh.material_index = primitive.material().index().unwrap() as i32;

                mesh.vbo.add_data(&vertices);
                mesh.vbo.upload(&mesh.vao, DrawingHint::StaticDraw);
                mesh.ibo.add_data(&indices);
                mesh.ibo.upload(&mesh.vao, DrawingHint::StaticDraw);

                mesh.vao.configure_attribute(0, 3, 8, 0);
                mesh.vao.configure_attribute(1, 3, 8, 3);
                mesh.vao.configure_attribute(2, 2, 8, 6);

                self.meshes.push(mesh);
            }
        }

        self.gltf = Some(gltf);
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

        // Draw the meshes
        let gltf = self.gltf.as_ref().unwrap();
        for mesh in self.meshes.iter() {
            let material = gltf.materials().nth(mesh.material_index as usize).unwrap();
            let base_color_index = material
                .pbr_metallic_roughness()
                .base_color_texture()
                .unwrap()
                .texture()
                .index();
            mesh.vao.bind();
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.textures[base_color_index]);
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.num_indices,
                    gl::UNSIGNED_INT,
                    ptr::null(),
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
