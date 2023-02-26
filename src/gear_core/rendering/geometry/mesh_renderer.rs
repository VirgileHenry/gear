use crate::{Material, Mesh, MeshRenderingBuffers, ShaderProgram};

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

        // draw
        self.rendering_buffers.draw();

    }


}