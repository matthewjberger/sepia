use crate::mesh::*;
use crate::texture::*;
use nalgebra_glm as glm;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Model {
    meshes: Vec<Mesh>,
}

impl Model {
    pub fn from_file(path: &str) -> Self {
        let path = Path::new(path);
        let root_dir = Path::new(path.parent().unwrap());
        let (models, materials) = tobj::load_obj(&path).unwrap();
        let mut meshes = Vec::new();
        for model in models.iter() {
            let mesh_data = &model.mesh;
            let mut vertices = Vec::new();
            for index in 0..mesh_data.positions.len() / 3 {
                let vertex = Vertex::new(
                    glm::vec3(
                        mesh_data.positions[index * 3],
                        mesh_data.positions[index * 3 + 1],
                        mesh_data.positions[index * 3 + 2],
                    ),
                    glm::vec3(
                        mesh_data.normals[index * 3],
                        mesh_data.normals[index * 3 + 1],
                        mesh_data.normals[index * 3 + 2],
                    ),
                    glm::vec2(
                        mesh_data.normals[index * 2],
                        mesh_data.normals[index * 2 + 1],
                    ),
                );
                vertices.push(vertex);
            }

            let mut textures = Vec::new();
            let material = &materials[mesh_data.material_id.unwrap()];

            let mut diffuse_texture_path = PathBuf::from(root_dir);
            diffuse_texture_path.push(&material.diffuse_texture);

            // TODO: Push other textures and add error handling
            // TODO: Use a texture cache

            println!(
                "{}",
                diffuse_texture_path.as_path().as_os_str().to_str().unwrap()
            );

            let texture =
                Texture::from_file(diffuse_texture_path.as_path().as_os_str().to_str().unwrap());
            textures.push(texture);

            meshes.push(Mesh::new(vertices, mesh_data.indices.clone(), textures));
        }
        Model { meshes }
    }

    pub fn render(&self) {
        for mesh in self.meshes.iter() {
            mesh.render();
        }
    }
}
