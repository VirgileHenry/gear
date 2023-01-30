use crate::{Material, Mesh, ShaderProgram, MeshRenderingBuffers};
use crate::opengl::buffers::*;


pub struct MeshRenderer {
    pub material: Material,
    triangle_count: usize,
    rendering_buffers: MeshRenderingBuffers,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh, material: Material) -> MeshRenderer {
        MeshRenderer {
            material,
            triangle_count: mesh.triangles.len(),
            rendering_buffers: MeshRenderingBuffers::from(mesh),
        }
    }

    fn vao(&self) -> &VertexArray {
        self.rendering_buffers.vao()
    }

    fn triangles_len(&self) -> usize {
        self.triangle_count
    }

    pub unsafe fn draw(&self, shader_program: &ShaderProgram) {

        // set material properties
        self.material.set_properties_to_shader(shader_program);
        // bind the vertex array
        self.rendering_buffers.vao().bind();

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