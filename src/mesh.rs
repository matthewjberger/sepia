use crate::buffer::*;
use crate::texture::*;
use crate::vao::*;
use nalgebra_glm as glm;
use std::ptr;

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
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut mesh = Mesh {
            vao: VertexArrayObject::new(),
            vbo: Buffer::new(BufferKind::Array),
            ibo: Buffer::new(BufferKind::Element),
            vertices,
            indices,
            textures,
        };

        mesh.vbo.add_data(&mesh.vertices);
        mesh.vbo.upload(&mesh.vao, DrawingHint::StaticDraw);
        mesh.ibo.add_data(&mesh.indices);
        mesh.ibo.upload(&mesh.vao, DrawingHint::StaticDraw);
        mesh.vao.configure_attribute(0, 3, 8, 0);
        mesh.vao.configure_attribute(1, 3, 8, 3);
        mesh.vao.configure_attribute(2, 2, 8, 6);
        mesh
    }

    pub fn render(&self) {
        self.vao.bind();
        // TODO: Break into diffuse, specular, ambient, etc textures
        // TODO: Set shader uniforms for textures
        for (index, texture) in self.textures.iter().enumerate() {
            texture.bind(index as u32);
        }
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}
