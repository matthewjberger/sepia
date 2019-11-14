use crate::buffer::*;
use crate::vao::*;
pub use gl::types::*;
pub use gl::types::*;
use gltf::{
    animation::{util::ReadOutputs, Interpolation},
    image::Format,
};
use nalgebra::{Matrix4, Quaternion, UnitQuaternion};
use nalgebra_glm as glm;
use petgraph::graph::{Graph, NodeIndex};

// TODO: Load bounding volumes using ncollide

pub type NodeGraph = Graph<Node, ()>;

#[derive(Debug)]
enum TransformationSet {
    Translations(Vec<glm::Vec3>),
    Rotations(Vec<glm::Vec4>),
    Scales(Vec<glm::Vec3>),
    MorphTargetWeights(Vec<f32>),
}

#[repr(C)]
#[derive(Default)]
pub struct Vertex {
    position: Option<glm::Vec3>,
    normal: Option<glm::Vec3>,
    tex_coords_0: Option<glm::Vec2>,
    tex_coords_1: Option<glm::Vec2>,
    joints_0: Option<glm::Vec4>,
    weights_0: Option<glm::Vec4>,
}

impl Vertex {
    // Packs the vertex data in a specific order
    pub fn pack_data(&self) -> Vec<f32> {
        let mut data: Vec<f32> = Vec::new();

        if let Some(position) = self.position {
            data.extend(&position.as_slice().to_vec());
        }

        if let Some(normal) = self.normal {
            data.extend(&normal.as_slice().to_vec());
        }

        if let Some(tex_coords_0) = self.tex_coords_0 {
            data.extend(&tex_coords_0.as_slice().to_vec());
        }

        if let Some(joints_0) = self.joints_0 {
            data.extend(&joints_0.as_slice().to_vec());
        }

        if let Some(weights_0) = self.weights_0 {
            data.extend(&weights_0.as_slice().to_vec());
        }

        data
    }
}

pub struct VertexSet {
    pub vertices: Vec<Vertex>,
}

impl VertexSet {
    fn pack_vertices(&self) -> Vec<f32> {
        self.vertices
            .iter()
            .map(|vertex| vertex.pack_data())
            .flatten()
            .collect::<Vec<_>>()
    }

    // This determines the order that attributes are configured in
    // This must patch the packing order for vertices
    fn data_lengths(&self) -> Vec<u32> {
        let vec2_length = 2;
        let vec3_length = 3;
        let vec4_length = 4;

        let mut data_lengths: Vec<u32> = Vec::new();

        if self.vertices.is_empty() {
            return data_lengths;
        }

        // first vertex is representative of all other vertices
        let first_vertex = &self.vertices[0];

        if first_vertex.position.is_some() {
            data_lengths.push(vec3_length);
        }

        if first_vertex.normal.is_some() {
            data_lengths.push(vec3_length);
        }

        if first_vertex.tex_coords_0.is_some() {
            data_lengths.push(vec2_length);
        }

        if first_vertex.joints_0.is_some() {
            data_lengths.push(vec4_length);
        }

        if first_vertex.weights_0.is_some() {
            data_lengths.push(vec4_length);
        }

        data_lengths
    }
}

#[derive(Debug)]
pub struct Skin {
    // When this is none,each matrix should be assumed to be the
    // 4x4 identity matrix, which implies that the inverse-bind matrices were pre-applied
    pub inverse_bind_matrices: Option<Vec<glm::Mat4>>,
    pub joint_indices: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct Transform {
    translation: Option<glm::Vec3>,
    rotation: Option<glm::Quat>,
    scale: Option<glm::Vec3>,
}

impl Transform {
    pub fn matrix(&self) -> glm::Mat4 {
        let mut matrix = glm::Mat4::identity();
        if let Some(translation) = self.translation {
            matrix *= Matrix4::new_translation(&translation);
        }
        if let Some(rotation) = self.rotation {
            matrix *= Matrix4::from(UnitQuaternion::from_quaternion(rotation));
        }
        if let Some(scale) = self.scale {
            matrix *= Matrix4::new_nonuniform_scaling(&scale);
        }
        matrix
    }
}

#[derive(Debug)]
pub struct Node {
    pub transform: glm::Mat4,
    pub animation_transform: Transform,
    pub mesh: Option<Mesh>,
    pub skin: Option<Skin>,
    pub index: usize,
}

#[derive(Debug)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

#[derive(Debug)]
pub struct Primitive {
    pub vao: VertexArrayObject,
    pub num_indices: i32,
    pub material_index: Option<usize>,
}

#[derive(Debug)]
pub struct Channel {
    node_index: usize,
    inputs: Vec<f32>,
    transformations: TransformationSet,
    interpolation: Interpolation,
    previous_key: usize,
    previous_time: f32,
}

#[derive(Debug)]
pub struct Animation {
    channels: Vec<Channel>,
}

#[derive(Debug)]
pub struct Scene {
    pub node_graphs: Vec<NodeGraph>,
}

pub struct GltfAsset {
    pub texture_ids: Vec<u32>,
    pub gltf: gltf::Document,
    pub scenes: Vec<Scene>,
    pub animations: Vec<Animation>,
}

impl GltfAsset {
    pub fn from_file(path: &str) -> Self {
        let (gltf, buffers, textures) = gltf::import(path).expect("Couldn't import file!");
        let texture_ids = prepare_textures_gl(&textures);
        let scenes = prepare_scenes(&gltf, &buffers);
        let animations = prepare_animations(&gltf, &buffers);

        GltfAsset {
            texture_ids,
            gltf,
            scenes,
            animations,
        }
    }

    pub fn lookup_material(&self, index: usize) -> gltf::Material {
        self.gltf
            .materials()
            .nth(index)
            .expect("Couldn't get material!")
    }

    pub fn animate(&mut self, seconds: f32) {
        // TODO: Allow for specifying a specific animation by name
        for animation in self.animations.iter_mut() {
            for channel in animation.channels.iter_mut() {
                for scene in self.scenes.iter_mut() {
                    for graph in scene.node_graphs.iter_mut() {
                        for node_index in graph.node_indices() {
                            if graph[node_index].index == channel.node_index {
                                let mut time = seconds % channel.inputs.last().unwrap();
                                let first_input = channel.inputs.first().unwrap();
                                if time.lt(first_input) {
                                    time = *first_input;
                                }

                                if channel.previous_time > time {
                                    channel.previous_key = 0;
                                }
                                channel.previous_time = time;

                                let mut next_key: usize = 0;
                                for index in channel.previous_key..channel.inputs.len() {
                                    let index = index as usize;
                                    if time <= channel.inputs[index] {
                                        next_key =
                                            nalgebra::clamp(index, 1, channel.inputs.len() - 1);
                                        break;
                                    }
                                }
                                channel.previous_key = nalgebra::clamp(next_key - 1, 0, next_key);

                                let key_delta =
                                    channel.inputs[next_key] - channel.inputs[channel.previous_key];
                                let normalized_time =
                                    (time - channel.inputs[channel.previous_key]) / key_delta;

                                // TODO: Interpolate with other methods
                                // Only Linear interpolation is used for now
                                match &channel.transformations {
                                    TransformationSet::Translations(translations) => {
                                        let start = translations[channel.previous_key];
                                        let end = translations[next_key];
                                        let translation = start.lerp(&end, normalized_time);
                                        let translation_vec =
                                            glm::make_vec3(translation.as_slice());
                                        graph[node_index].animation_transform.translation =
                                            Some(translation_vec);
                                    }
                                    TransformationSet::Rotations(rotations) => {
                                        let start = rotations[channel.previous_key];
                                        let end = rotations[next_key];
                                        let start_quat =
                                            Quaternion::new(start[3], start[0], start[1], start[2]);
                                        let end_quat =
                                            Quaternion::new(end[3], end[0], end[1], end[2]);
                                        let rotation_quat =
                                            start_quat.lerp(&end_quat, normalized_time);
                                        graph[node_index].animation_transform.rotation =
                                            Some(rotation_quat);
                                    }
                                    TransformationSet::Scales(scales) => {
                                        let start = scales[channel.previous_key];
                                        let end = scales[next_key];
                                        let scale = start.lerp(&end, normalized_time);
                                        let scale_vec = glm::make_vec3(scale.as_slice());
                                        graph[node_index].animation_transform.scale =
                                            Some(scale_vec);
                                    }
                                    TransformationSet::MorphTargetWeights(_weights) => {
                                        unimplemented!()
                                    }
                                }

                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

// TODO: Write this method for vec3's and vec4's
// fn interpolate(interpolation: Interpolation) {
//     match interpolation {
//         Interpolation::Linear => {}
//         Interpolation::Step => {}
//         Interpolation::CatmullRomSpline => {}
//         Interpolation::CubicSpline => {}
//     }
// }

fn prepare_animations(gltf: &gltf::Document, buffers: &[gltf::buffer::Data]) -> Vec<Animation> {
    // TODO: load names if present as well
    let mut animations = Vec::new();
    for animation in gltf.animations() {
        let mut channels = Vec::new();
        for channel in animation.channels() {
            let sampler = channel.sampler();
            let interpolation = sampler.interpolation();
            let node_index = channel.target().node().index();
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
            let inputs = reader.read_inputs().unwrap().collect::<Vec<_>>();
            let outputs = reader.read_outputs().unwrap();
            let transformations: TransformationSet;
            match outputs {
                ReadOutputs::Translations(translations) => {
                    let translations = translations.map(glm::Vec3::from).collect::<Vec<_>>();
                    transformations = TransformationSet::Translations(translations);
                }
                ReadOutputs::Rotations(rotations) => {
                    let rotations = rotations
                        .into_f32()
                        .map(glm::Vec4::from)
                        .collect::<Vec<_>>();
                    transformations = TransformationSet::Rotations(rotations);
                }
                ReadOutputs::Scales(scales) => {
                    let scales = scales.map(glm::Vec3::from).collect::<Vec<_>>();
                    transformations = TransformationSet::Scales(scales);
                }
                ReadOutputs::MorphTargetWeights(weights) => {
                    let morph_target_weights = weights.into_f32().collect::<Vec<_>>();
                    transformations = TransformationSet::MorphTargetWeights(morph_target_weights);
                }
            }
            channels.push(Channel {
                node_index,
                inputs,
                transformations,
                interpolation,
                previous_key: 0,
                previous_time: 0.0,
            });
        }
        animations.push(Animation { channels });
    }
    animations
}

// TODO: Make graph a collection of collections of graphs belonging to the scene (Vec<Vec<NodeGraph>>)
// TODO: Load names for scenes and nodes
fn prepare_scenes(gltf: &gltf::Document, buffers: &[gltf::buffer::Data]) -> Vec<Scene> {
    let mut scenes: Vec<Scene> = Vec::new();
    for scene in gltf.scenes() {
        let mut node_graphs: Vec<NodeGraph> = Vec::new();
        for node in scene.nodes() {
            let mut node_graph = NodeGraph::new();
            visit_children(&node, &buffers, &mut node_graph, NodeIndex::new(0_usize));
            node_graphs.push(node_graph);
        }
        scenes.push(Scene { node_graphs });
    }
    scenes
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
    node_graph: &mut NodeGraph,
    parent_index: NodeIndex,
) {
    let node_info = Node {
        transform: determine_transform(node),
        animation_transform: Transform::default(),
        mesh: load_mesh(node, buffers),
        skin: load_skin(node, buffers),
        index: node.index(),
    };

    let node_index = node_graph.add_node(node_info);
    if parent_index != node_index {
        node_graph.add_edge(parent_index, node_index, ());
    }

    for child in node.children() {
        visit_children(&child, buffers, node_graph, node_index);
    }
}

fn load_mesh(node: &gltf::Node, buffers: &[gltf::buffer::Data]) -> Option<Mesh> {
    if let Some(mesh) = node.mesh() {
        let mut all_primitive_info = Vec::new();
        for primitive in mesh.primitives() {
            let (vertex_set, indices) = read_buffer_data(&primitive, &buffers);
            let mut primitive_info = prepare_primitive_gl(&vertex_set, &indices);
            let material_index = primitive.material().index();
            primitive_info.material_index = material_index;
            all_primitive_info.push(primitive_info);
        }
        Some(Mesh {
            primitives: all_primitive_info,
        })
    } else {
        None
    }
}

fn load_skin(node: &gltf::Node, buffers: &[gltf::buffer::Data]) -> Option<Skin> {
    if let Some(skin) = node.skin() {
        let reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
        let inverse_bind_matrices = reader.read_inverse_bind_matrices().and_then(|matrices| {
            matrices
                .map(|matrix| Some(glm::Mat4::from(matrix)))
                .collect::<Option<Vec<_>>>()
        });

        // TODO: May not need these
        let mut joint_indices: Vec<usize> = Vec::new();
        for joint_node in skin.joints() {
            joint_indices.push(joint_node.index());
        }

        Some(Skin {
            inverse_bind_matrices,
            joint_indices,
        })
    } else {
        None
    }
}

fn determine_transform(node: &gltf::Node) -> glm::Mat4 {
    let transform: Vec<f32> = node
        .transform()
        .matrix()
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();
    glm::make_mat4(&transform.as_slice())
}

fn read_buffer_data(
    primitive: &gltf::Primitive,
    buffers: &[gltf::buffer::Data],
) -> (VertexSet, Vec<u32>) {
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    let positions = reader.read_positions().map_or(Vec::new(), |positions| {
        positions.map(glm::Vec3::from).collect::<Vec<_>>()
    });

    let normals = reader.read_normals().map_or(Vec::new(), |normals| {
        normals.map(glm::Vec3::from).collect::<Vec<_>>()
    });

    let convert_coords = |coords: gltf::mesh::util::ReadTexCoords<'_>| -> Vec<glm::Vec2> {
        coords.into_f32().map(glm::Vec2::from).collect::<Vec<_>>()
    };
    let tex_coords_0 = reader.read_tex_coords(0).map_or(Vec::new(), convert_coords);
    let tex_coords_1 = reader.read_tex_coords(1).map_or(Vec::new(), convert_coords);

    let joints_0 = reader.read_joints(0).map_or(Vec::new(), |joints| {
        joints
            .into_u16()
            .map(|joint| {
                glm::vec4(
                    joint[0] as f32,
                    joint[1] as f32,
                    joint[2] as f32,
                    joint[3] as f32,
                )
            })
            .collect::<Vec<_>>()
    });

    let weights_0 = reader.read_weights(0).map_or(Vec::new(), |weights| {
        weights.into_f32().map(glm::Vec4::from).collect::<Vec<_>>()
    });

    let mut vertices = Vec::new();
    for (index, position) in positions.iter().enumerate() {
        vertices.push(Vertex {
            position: Some(*position),
            normal: normals.get(index).copied(),
            tex_coords_0: tex_coords_0.get(index).copied(),
            tex_coords_1: tex_coords_1.get(index).copied(),
            joints_0: joints_0.get(index).copied(),
            weights_0: weights_0.get(index).copied(),
        });
    }

    let indices = reader
        .read_indices()
        .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>())
        .unwrap();

    (VertexSet { vertices }, indices)
}

fn prepare_primitive_gl(vertex_set: &VertexSet, indices: &[u32]) -> Primitive {
    let vao = VertexArrayObject::new();
    let mut vbo = Buffer::new(BufferKind::Array);
    let mut ibo = Buffer::new(BufferKind::Element);

    vbo.add_data(&vertex_set.pack_vertices());
    vbo.upload(&vao, DrawingHint::StaticDraw);

    ibo.add_data(indices);
    ibo.upload(&vao, DrawingHint::StaticDraw);

    let data_lengths = vertex_set.data_lengths();
    let total_length = data_lengths.iter().sum();
    let mut offset = 0;
    for (current_attribute, length) in data_lengths.iter().enumerate() {
        vao.configure_attribute(current_attribute as u32, *length, total_length, offset);
        offset += length;
    }

    Primitive {
        vao,
        num_indices: indices.len() as i32,
        material_index: None,
    }
}
