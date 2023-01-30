use crate::{buffers::{VertexArray, BufferObject}, Mesh, Vertex};

#[allow(unused)] // technically we are not using vbo and ebo, but keep them alive do avoid dropping them
pub struct MeshRenderingBuffers {
    vao: VertexArray, // holds rendering parameter data and pointer to vbo + ebo
    vbo: BufferObject, // holds vertex data
    ebo: BufferObject, // holds index data
}

impl MeshRenderingBuffers {
    pub fn new(vao: VertexArray, vbo: BufferObject, ebo: BufferObject) -> MeshRenderingBuffers {
        MeshRenderingBuffers { vao, vbo, ebo }
    }

    pub fn from(mesh: Mesh) -> MeshRenderingBuffers {
        // create and bind the vao
        let vao = VertexArray::new();
        vao.bind();
        // create the vbo, bind it, upload data to it and give the vertex the attrib pointers
        let vbo = BufferObject::new(gl::ARRAY_BUFFER);
        vbo.bind();
        vbo.upload_data(&mesh.vertices, gl::STATIC_DRAW);
        Vertex::vertex_attrib_pointer();
        // create the ebo, bind it, upload data to it
        let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER);
        ebo.bind();
        ebo.upload_data(&mesh.triangles, gl::STATIC_DRAW);
        // unbind the vao first, otherwise we will unbind the vbo and ebo from the vao
        vao.unbind();
        // ubind vbo and ebo
        ebo.unbind();
        vbo.unbind();

        MeshRenderingBuffers::new(vao, vbo, ebo)
    }

    pub fn vao(&self) -> &VertexArray {
        &self.vao
    }
}
