extern crate cgmath;
extern crate gl;
use crate::gear_core::rendering::geometry::primitives::Vertex;
use crate::gear_core::rendering::material::Material;

use super::material;

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
    vbo: gl::types::GLuint, // holds vertex data
    vao: gl::types::GLuint, // holds rendering parameter data
    ebo: gl::types::GLuint, // holds index data
}

impl MeshRenderer {
    pub fn new(mesh: MeshType, material: Material) -> MeshRenderer {
        match mesh {
            MeshType::Owned(mesh) => {
                let mut vbo = 0;
                let mut vao = 0;
                let mut ebo = 0;
                unsafe {
                    // vbo
                    // create the buffer and bind it
                    gl::GenBuffers(1, &mut vbo);
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    // put the vertex data in the buffer
                    gl::BufferData(
                        gl::ARRAY_BUFFER,                                                       // target
                        (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
                        mesh.vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                        gl::STATIC_DRAW,                               // usage
                    );
                    // unbind the buffer
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    // ebo
                    // create the buffer and bind it
                    gl::GenBuffers(1, &mut ebo);
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                    // put the vertex data in the buffer
                    gl::BufferData(
                        gl::ELEMENT_ARRAY_BUFFER,                                                       // target
                        (mesh.triangles.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
                        mesh.triangles.as_ptr() as *const gl::types::GLvoid, // pointer to data
                        gl::STATIC_DRAW,                               // usage
                    );
                    // unbind the buffer
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    // vao
                    // create the buffer and bind it
                    gl::GenVertexArrays(1, &mut vao);
                    gl::BindVertexArray(vao);
                    // tell the vao where is the vbo
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    // tell the vao where is the ebo
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                    // shader layouts 
                    gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
                    gl::VertexAttribPointer(
                        0,         // index of the generic vertex attribute ("layout (location = 0)")
                        3,         // the number of components per generic vertex attribute
                        gl::FLOAT, // data type
                        gl::FALSE, // normalized (int-to-float conversion)
                        (std::mem::size_of::<Vertex>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                        std::ptr::null(),                                     // offset of the first component
                    );
                    gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
                    gl::VertexAttribPointer(
                        1,         // index of the generic vertex attribute ("layout (location = 0)")
                        3,         // the number of components per generic vertex attribute
                        gl::FLOAT, // data type
                        gl::FALSE, // normalized (int-to-float conversion)
                        (std::mem::size_of::<Vertex>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                        (std::mem::size_of::<cgmath::Vector3<f32>>()) as *const std::ffi::c_void, // offset of the first component
                    );
                    gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
                    gl::VertexAttribPointer(
                        2,         // index of the generic vertex attribute ("layout (location = 0)")
                        2,         // the number of components per generic vertex attribute
                        gl::FLOAT, // data type
                        gl::FALSE, // normalized (int-to-float conversion)
                        (std::mem::size_of::<Vertex>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                        (2 * std::mem::size_of::<cgmath::Vector3<f32>>()) as *const std::ffi::c_void, // offset of the first component
                    );
                    // unbind
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    gl::BindVertexArray(0);
                }
                MeshRenderer { 
                    mesh: MeshType::Owned(mesh),
                    material: material,
                    vbo: vbo,
                    vao: vao,
                    ebo: ebo,
                }
            }
        }
    }

    pub fn vao(&self) -> gl::types::GLuint {
        self.vao
    }

    pub fn vbo(&self) -> gl::types::GLuint {
        self.vbo
    }

    pub fn triangles_len(&self) -> usize {
        match &self.mesh {
            MeshType::Owned(mesh) => mesh.triangles.len(),
        }
    } 
}