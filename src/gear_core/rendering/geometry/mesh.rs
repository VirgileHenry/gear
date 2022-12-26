mod mesh_presets;

extern crate cgmath;
extern crate gl;
use crate::gear_core::rendering::{
    geometry::primitives::Vertex, 
    material::Material,
    opengl,
};

pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<u32>,
}

pub enum MeshType {
    Owned(Mesh),
    // Shared(&Mesh),
}


pub struct MeshRenderer {
    pub mesh: MeshType,
    pub material: Material,
    // opengl stuff
    vao: opengl::buffers::VertexArray, // holds rendering parameter data
    _vbo: opengl::buffers::BufferObject, // holds vertex data
    _ebo: opengl::buffers::BufferObject, // holds index data
}


impl MeshRenderer {
    pub fn new(mesh: MeshType, material: Material) -> MeshRenderer {
        match mesh {
            MeshType::Owned(mesh) => {
                // create and bind the vao
                let vao = opengl::buffers::VertexArray::new();
                vao.bind();
                // create the vbo, bind it, upload data to it and give the vertex the attrib pointers
                let vbo = opengl::buffers::BufferObject::new(gl::ARRAY_BUFFER);
                vbo.bind();
                vbo.upload_data(&mesh.vertices, gl::STATIC_DRAW);
                Vertex::vertex_attrib_pointer();
                // create the ebo, bind it, upload data to it
                let ebo = opengl::buffers::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER);
                ebo.bind();
                ebo.upload_data(&mesh.triangles, gl::STATIC_DRAW);
                // unbind the vao first, otherwise we will unbind the vbo and ebo from the vao
                vao.unbind();
                // ubind vbo and ebo
                ebo.unbind();
                vbo.unbind();
                
                MeshRenderer { 
                    mesh: MeshType::Owned(mesh),
                    material: material,
                    vao: vao,
                    _vbo: vbo,
                    _ebo: ebo,
                }
            }
        }
    }

    pub fn vao(&self) -> &opengl::buffers::VertexArray {
        &self.vao
    }

    pub fn triangles_len(&self) -> usize {
        match &self.mesh {
            MeshType::Owned(mesh) => mesh.triangles.len(),
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub(crate) unsafe fn bind_textures(&self, mut texture_index: u32) -> u32 {
        self.material.bind_textures(texture_index)
    }

}