use crate::buffer::*;
use crate::vao::*;
pub use gl::types::*;
pub use gl::types::*;
use gltf::{
    animation::{util::ReadOutputs, Interpolation},
    image::Format,
};
use nalgebra_glm as glm;

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
    node_index: usize,
    pub primitives: Vec<PrimitiveInfo>,
    pub transform: glm::Mat4,
}

pub struct PrimitiveInfo {
    pub vao: VertexArrayObject,
    pub num_indices: i32,
    pub material_index: i32,
}

pub struct AnimationInfo {
    node_index: usize,
    inputs: Vec<f32>,
    transformations: TransformationSet,
    interpolation: Interpolation,
}

enum TransformationSet {
    Translations(Vec<glm::Vec3>),
    Rotations(Vec<glm::Vec4>),
    Scales(Vec<glm::Vec3>),
    MorphTargetWeights(Vec<f32>),
}

pub struct GltfScene {
    pub texture_ids: Vec<u32>,
    pub gltf: gltf::Document,
    pub meshes: Vec<MeshInfo>,
    pub animations: Vec<AnimationInfo>,
}

impl GltfScene {
    pub fn from_file(path: &str) -> Self {
        let (gltf, buffers, textures) = gltf::import(path).expect("Couldn't import file!");
        let texture_ids = prepare_textures_gl(&textures);
        let meshes = prepare_meshes(&gltf, &buffers);
        let animations = prepare_animations(&gltf, &buffers);

        GltfScene {
            texture_ids,
            gltf,
            meshes,
            animations,
        }
    }

    pub fn lookup_material(&self, index: i32) -> gltf::Material {
        self.gltf
            .materials()
            .nth(index as usize)
            .expect("Couldn't get material!")
    }
}

pub fn animate_mesh(animations: &[AnimationInfo], mesh_info: &mut MeshInfo, seconds: f32) {
    if animations.is_empty() {
        return;
    }

    let animation = animations
        .iter()
        .find(|animation_info| animation_info.node_index == mesh_info.node_index);

    if animation.is_none() {
        return;
    }

    let animation = animation.expect("Couldn't get the mesh animation!");

    match &animation.transformations {
        TransformationSet::Translations(translations) => {
            println!("Translate!");

            // TODO: map provided seconds to animation seconds between min and max inputs
            // TODO: interpolate between translations at keyframe indices and apply to mesh transform
        }
        TransformationSet::Rotations(rotations) => {
            println!("Rotate!");
        }
        TransformationSet::Scales(scales) => unimplemented!(),
        TransformationSet::MorphTargetWeights(weights) => unimplemented!(),,
    }

    println!("Found animation!");
}

// TODO: Write this method for vec3's and vec4's
fn interpolate(interpolation: Interpolation) {
    match interpolation {
        Interpolation::Linear => {}
        Interpolation::Step => {}
        Interpolation::CatmullRomSpline => {}
        Interpolation::CubicSpline => {}
    }
}

fn prepare_animations(gltf: &gltf::Document, buffers: &[gltf::buffer::Data]) -> Vec<AnimationInfo> {
    let mut all_animation_info = Vec::new();
    for animation in gltf.animations() {
        for channel in animation.channels() {
            let sampler = channel.sampler();
            let interpolation = sampler.interpolation();
            let node_index = channel.target().node().index();
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
            let inputs = reader.read_inputs().unwrap().collect::<Vec<_>>();
            let outputs = reader.read_outputs().unwrap();

            println!("Interpolation Mode: {:?}", interpolation);
            println!("Inputs: {:?}", inputs);

            let transformations: TransformationSet;

            // TODO: Generalize the mapping to vec3 and vec4
            match outputs {
                ReadOutputs::Translations(translations) => {
                    let translations = translations
                        .map(|translation| {
                            glm::vec3(translation[0], translation[1], translation[2])
                        })
                        .collect::<Vec<_>>();
                    println!("Translations: {:?}", translations);
                    transformations = TransformationSet::Translations(translations);
                }
                ReadOutputs::Rotations(rotations) => {
                    let rotations = rotations
                        .into_f32()
                        .map(|rotation| {
                            glm::vec4(rotation[0], rotation[1], rotation[2], rotation[3])
                        })
                        .collect::<Vec<_>>();
                    println!("Rotations: {:?}", rotations);
                    transformations = TransformationSet::Rotations(rotations);
                }
                ReadOutputs::Scales(scales) => {
                    let scales = scales
                        .map(|scale| glm::vec3(scale[0], scale[1], scale[2]))
                        .collect::<Vec<_>>();
                    println!("Scales: {:?}", scales);
                    transformations = TransformationSet::Scales(scales);
                }
                ReadOutputs::MorphTargetWeights(weights) => {
                    let morph_target_weights = weights.into_f32().collect::<Vec<_>>();
                    println!("Morph Target Weights: {:?}", morph_target_weights);
                    transformations = TransformationSet::MorphTargetWeights(morph_target_weights);
                }
            }
            all_animation_info.push(AnimationInfo {
                node_index,
                inputs,
                transformations,
                interpolation,
            });
        }
    }
    all_animation_info
}

fn prepare_meshes(gltf: &gltf::Document, buffers: &[gltf::buffer::Data]) -> Vec<MeshInfo> {
    let mut meshes: Vec<MeshInfo> = Vec::new();
    for scene in gltf.scenes() {
        for node in scene.nodes() {
            visit_children(&node, &buffers, &mut meshes);
        }
    }
    meshes
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

            let mut primitive_info = prepare_primitive_gl(&vertices, &indices, material_index);
            primitive_info.material_index = material_index;
            all_primitive_info.push(primitive_info);
        }

        let transform_matrix = determine_transform(&node);

        loaded_meshes.push(MeshInfo {
            primitives: all_primitive_info,
            transform: transform_matrix,
            node_index: node.index(),
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

fn prepare_primitive_gl(
    vertices: &[Vertex],
    indices: &[u32],
    material_index: i32,
) -> PrimitiveInfo {
    let vao = VertexArrayObject::new();
    let mut vbo = Buffer::new(BufferKind::Array);
    let mut ibo = Buffer::new(BufferKind::Element);

    vbo.add_data(vertices);
    vbo.upload(&vao, DrawingHint::StaticDraw);

    ibo.add_data(indices);
    ibo.upload(&vao, DrawingHint::StaticDraw);

    vao.configure_attribute(0, 3, 8, 0); // Position
    vao.configure_attribute(1, 3, 8, 3); // Normal
    vao.configure_attribute(2, 2, 8, 6); // Texture Coordinate

    PrimitiveInfo {
        vao,
        num_indices: indices.len() as i32,
        material_index,
    }
}
