use crate::rendering::{shaders::ShaderProgram, renderer::RenderObject, camera::Camera};


pub enum RenderType<'a> {
    Simple(RenderObject<'a>),
    None,
}

pub trait GameObject { 
    fn update(&mut self, delta:f32);

    fn to_render_objects(&self) -> RenderType;

    fn render(&self, camera: &Camera);
    // how ? -> set shader prog -> set uniforms -> call draw on mesh

    fn set_uniform(&self, shader_program: &ShaderProgram);
}
