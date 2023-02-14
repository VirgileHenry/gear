use crate::{Material, Mesh, MeshRenderingBuffers, ShaderProgram, Texture2D};
use crate::opengl::buffers::*;

pub struct MeshRenderer {
    pub material: Material,
    rendering_buffers: MeshRenderingBuffers,
}

impl MeshRenderer {
    pub fn new(mesh: &Mesh, material: Material) -> MeshRenderer {
        MeshRenderer {
            material,
            rendering_buffers: MeshRenderingBuffers::from(mesh),
        }
    }

    pub unsafe fn draw(&self, shader_program: &ShaderProgram) {

        // set material properties
        self.material.set_properties_to_shader(shader_program);
        // bind the vao for drawing
        self.rendering_buffers.bind();

        // (bind textures)
        let mut texture_index = self.material.bind_textures(gl::TEXTURE0);

        // (change states)

        // draw
        self.rendering_buffers.draw();

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

    pub fn attach_texture(&mut self, texture: Texture2D) {
        self.material.attach_texture(texture);
    }
    pub fn pop_texture(&mut self) -> Option<Texture2D> {
        self.material.pop_texture()
    }

}