use crate::buffer::*;
use crate::vao::*;
pub use gl::types::*;
pub use gl::types::*;
use gltf::image::Format;
use nalgebra_glm as glm;
use std::ptr;

// TODO: join up crate use statements

// TODO: Load bounding volumes using ncollide

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

pub struct MeshInfo {
    primitives: Vec<PrimitiveInfo>,
    pub transform: glm::Mat4,
}

// TODO: This is actually representing a primitive.
//       Call this primitive and make a separate 'Mesh' class containing the transform.
#[derive(Default)]
pub struct PrimitiveInfo {
    vao: VertexArrayObject,
    vbo: Buffer,
    ibo: Buffer,
    num_indices: i32,
    material_index: i32,
}

pub struct GltfScene {
    texture_ids: Vec<u32>,
    gltf: gltf::Document,
    meshes: Vec<MeshInfo>,
}

impl GltfScene {
    pub fn from_file(path: &str) -> Self {
        let (gltf, buffers, textures) = gltf::import(path).expect("Couldn't import file!");
        let texture_ids = prepare_textures_gl(&textures);
        let mut meshes: Vec<MeshInfo> = Vec::new();
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                visit_children(&node, &buffers, &mut meshes);
            }
        }
        GltfScene {
            texture_ids,
            gltf,
            meshes,
        }
    }

    // TODO: Create a shader cache and retrieve the shader to use from there.
    //       Need pbr shaders and need basic shaders
    pub fn render_meshes<F>(&self, handle_mesh: F)
    where
        F: Fn(&MeshInfo, &[f32; 4]),
    {
        for mesh in self.meshes.iter() {
            // TODO: Store ref to material in mesh

            for primitive_info in mesh.primitives.iter() {
                let material = self.lookup_material(primitive_info.material_index);
                let pbr = material.pbr_metallic_roughness();
                let base_color = pbr.base_color_factor();

                if !self.texture_ids.is_empty() {
                    let base_color_index = pbr
                        .base_color_texture()
                        .expect("Couldn't get base color texture!")
                        .texture()
                        .index();
                    unsafe {
                        gl::BindTexture(gl::TEXTURE_2D, self.texture_ids[base_color_index]);
                    }
                }

                handle_mesh(&mesh, &base_color);

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

    fn lookup_material(&self, index: i32) -> gltf::Material {
        self.gltf
            .materials()
            .nth(index as usize)
            .expect("Couldn't get material!")
    }
}

fn prepare_textures_gl(textures: &[gltf::image::Data]) -> Vec<u32> {
    let mut texture_ids = Vec::new();
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
        texture_ids.push(texture_id);
    }
    texture_ids
}

fn visit_children(
    node: &gltf::Node,
    buffers: &[gltf::buffer::Data],
    loaded_meshes: &mut Vec<MeshInfo>,
) {
    if let Some(mesh) = node.mesh() {
        let mut all_primitive_info = Vec::new();
        for primitive in mesh.primitives() {
            let (vertices, indices) = read_buffer_data(&primitive, &buffers);

            // TODO: Make mesh store transform for all its primitives
            let material_index = primitive
                .material()
                .index()
                .expect("Couldn't get material index!") as i32;

            let mut primitive_info = prepare_primitive_gl(&vertices, &indices);
            primitive_info.material_index = material_index;
            all_primitive_info.push(primitive_info);
        }

        let transform_matrix = determine_transform(&node);

        loaded_meshes.push(MeshInfo {
            primitives: all_primitive_info,
            transform: transform_matrix,
        });
    }

    for child in node.children() {
        visit_children(&child, buffers, loaded_meshes);
    }
}

fn determine_transform(node: &gltf::Node) -> glm::Mat4 {
    let transform = node.transform().matrix();
    let mut matrix_data = Vec::new();

    for row in transform.iter() {
        for item in row.iter() {
            matrix_data.push(*item);
        }
    }

    glm::make_mat4(matrix_data.as_slice())
}

fn read_buffer_data(
    primitive: &gltf::Primitive,
    buffers: &[gltf::buffer::Data],
) -> (Vec<Vertex>, Vec<u32>) {
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    let positions = reader
        .read_positions()
        .expect("Couldn't read positions!")
        .collect::<Vec<_>>();
    let normals = reader
        .read_normals()
        .expect("Couldn't read normals")
        .collect::<Vec<_>>();
    let tex_coords = reader
        .read_tex_coords(0)
        .map(|read_tex_coords| read_tex_coords.into_f32().collect::<Vec<_>>())
        .unwrap_or_else(|| vec![[0.0; 2]; positions.len()]);

    // TODO: Load and configure second set of tex_coords 'read_tex_coords(1)'

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

    let indices = reader
        .read_indices()
        .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>())
        .unwrap();

    (vertices, indices)
}

fn prepare_primitive_gl(vertices: &[Vertex], indices: &[u32]) -> PrimitiveInfo {
    let mut primitive_info = PrimitiveInfo::default();
    primitive_info.vao = VertexArrayObject::new();
    primitive_info.vbo = Buffer::new(BufferKind::Array);
    primitive_info.ibo = Buffer::new(BufferKind::Element);
    primitive_info.num_indices = indices.len() as i32;

    primitive_info.vbo.add_data(vertices);
    primitive_info
        .vbo
        .upload(&primitive_info.vao, DrawingHint::StaticDraw);
    primitive_info.ibo.add_data(indices);
    primitive_info
        .ibo
        .upload(&primitive_info.vao, DrawingHint::StaticDraw);

    primitive_info.vao.configure_attribute(0, 3, 8, 0);
    primitive_info.vao.configure_attribute(1, 3, 8, 3);
    primitive_info.vao.configure_attribute(2, 2, 8, 6);

    primitive_info
}
