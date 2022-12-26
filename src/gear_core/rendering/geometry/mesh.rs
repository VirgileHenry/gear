mod mesh_presets;

extern crate cgmath;
extern crate gl;
use crate::gear_core::rendering::{
    geometry::primitives::Vertex, 
    material::Material,
    opengl,
};
use crate::ShaderProgram;

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

    pub unsafe fn draw(&self, shader_program: &ShaderProgram) {

        // set material properties
        self.material.set_properties_to_shader(shader_program);
        // bind the vertex array
        self.vao.bind();

        // (bind textures)
        let mut texture_index = self.material.bind_textures(gl::TEXTURE0);

        // (change states)
        // draw elements (glDrawArrays or glDrawElements)

        gl::DrawElements(
            gl::TRIANGLES, // mode
            self.triangles_len() as i32, // starting index in the enabled arrays
            gl::UNSIGNED_INT,
            0 as *const std::ffi::c_void, // number of indices to be rendered
        );

        // Unbinding all textures
        while texture_index != gl::TEXTURE0 {
            gl::ActiveTexture(texture_index);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            texture_index -= 1;
        }

        // unbinding last gl::TEXTURE0
        gl::ActiveTexture(texture_index);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
}