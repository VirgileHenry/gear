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

impl Mesh {
    pub fn cube(side_size: f32) -> Mesh {
        Mesh {
            vertices: vec![
                // top
                Vertex::new(-side_size, side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                // front
                Vertex::new(-side_size, -side_size, -side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, -side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, -side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, -side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                // right
                Vertex::new(side_size, -side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                // bottom
                Vertex::new(-side_size, -side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, -side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                // back
                Vertex::new(-side_size, -side_size, side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(side_size, side_size, side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                Vertex::new(side_size, -side_size, side_size, 0.0, 0.0, -1.0, 0.0, 0.0),
                // left
                Vertex::new(-side_size, -side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, -side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, side_size, 0.0, 1.0, 0.0, 0.0, 0.0),
                Vertex::new(-side_size, side_size, -side_size, 0.0, 1.0, 0.0, 0.0, 0.0),

            ],
            triangles: vec![
                0, 1, 2,    // top
	            0, 2, 3, 
	            4, 5, 6,    // front
	            4, 6, 7, 
                8, 9, 10,   // right
                8, 10, 11, 
                12, 13, 14, // bottom
                12, 14, 15,
                16, 17, 18, // back
                16, 18, 19,
                20, 21, 22, //left
                20, 22, 23,
            ],
        }
    }

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
}