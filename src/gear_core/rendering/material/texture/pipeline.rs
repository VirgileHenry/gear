use std::collections::HashMap;
use cgmath::{Matrix4, Vector3};
use gl::types::GLuint;
use crate::gear_core::material::texture::{Texture2D, TexturePresets};
use crate::{Material, Mesh, MeshRenderer, NoParamMaterialProperties, ShaderProgram};

pub struct ShaderPipelineNodeParam {
    ints: HashMap<String, i32>,
    floats: HashMap<String, f32>,
    vec3s: HashMap<String, Vector3<f32>>,
    mat4s: HashMap<String, Matrix4<f32>>,
}

impl ShaderPipelineNodeParam {
    pub fn set_int(&mut self, name: String, val: i32) {
        self.ints.entry(name).or_insert(val);
    }

    pub fn set_float(&mut self, name: String, val: f32) {
        self.floats.entry(name).or_insert(val);
    }

    pub fn set_vec3(&mut self, name: String, val: Vector3<f32>) {
        self.vec3s.entry(name).or_insert(val);
    }

    pub fn set_mat4(&mut self, name: String, val: Matrix4<f32>) {
        self.mat4s.entry(name).or_insert(val);
    }

    pub fn set_texture(&mut self, name: String, val: i32) {
        self.ints.entry(name).or_insert(val);
    }

    pub unsafe fn bind_all_params(&self, shader: &ShaderProgram) {
        for (name, val) in &self.ints {
            shader.set_int(name, *val);
        }

        for (name, val) in &self.floats {
            shader.set_float(name, *val);
        }

        for (name, val) in &self.mat4s {
            shader.set_mat4(name, *val);
        }

        for (name, val) in &self.vec3s {
            shader.set_vec3(name, *val);
        }
    }

}


/*
ça serait pas plus propre ? ça parait plus "rust" et moins "C", et ça force ta condition
En rust on aime bien que si ya des cas impossible, ils soient juste pas représentables
pub enum ShaderPipelineInput {
    Texture(Texture2D),
    Nodes(Vec<ShaderPipelineNode>),
} 
*/

// Shader Pipeline node
pub struct ShaderPipelineNode {
    // A node has either an input texture OR a non empty list of input nodes // todo brice : regarde au dessus 
    input_texture: Option<Texture2D>,
    input_nodes: Vec<ShaderPipelineNode>,

    texture: Texture2D,
    shader: ShaderProgram,
    param: Option<ShaderPipelineNodeParam>,

    framebuffer_id: GLuint,
}

impl ShaderPipelineNode {

    pub fn new(dimensions: (i32, i32),
               input_nodes: Vec<ShaderPipelineNode>,
               shader: ShaderProgram,
               param: Option<ShaderPipelineNodeParam>) -> Self {

        let texture = Texture2D::new_from_presets(dimensions, TexturePresets::pipeline_default(), None);

        let mut framebuffer_id = 0;
        unsafe {
            gl::Viewport(0, 0, dimensions.0, dimensions.1);

            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.get_id(),
                0,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT {
                panic!("Framebuffer is not complete !")
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // unbind the framebuffer until needed
        }

        Self{
            input_texture: None,
            input_nodes,

            texture,
            shader,
            param,

            framebuffer_id,
        }
    }

    /// Set current_input to None to use the pipeline
    pub unsafe fn compute(&self, input_textures: &Vec<Texture2D>) {
        let plane_mesh = Mesh::plane(Vector3::unit_x()*2., Vector3::unit_y()*2.);
        let material = Material::from_program("copy_shader", Box::new(NoParamMaterialProperties{})); // todo Brice: ne pas utiliser ca
        let mesh_renderer = MeshRenderer::new(&plane_mesh, material);
        self.compute_rec_with_plane(input_textures, &mesh_renderer);
    }

    unsafe fn compute_rec_with_plane(&self, input_textures: &Vec<Texture2D>, plane: &MeshRenderer) {

        let mut current_active_tex = gl::TEXTURE0;
        gl::ActiveTexture(current_active_tex);
        // Set up input textures
        match &self.input_texture {
            Some(val) => val.bind(),
            _ => {
                for input_node in &self.input_nodes {
                    input_node.compute_rec_with_plane(&input_textures, plane);
                    // set up input texture
                    gl::ActiveTexture(current_active_tex);
                    input_node.get_texture().bind();
                    current_active_tex += 1;
                }
            }
        }

        // Binding current node framebuffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);

        // Setting shader parameters
        self.shader.set_used();
        match &self.param {
            Some(param) => param.bind_all_params(&self.shader),
            _ => (),
        }


        // Drawing result onto node's texture
        plane.draw(&self.shader);
    }

    pub fn set_input_texture(&mut self, texture: Texture2D) {
        if self.input_nodes.len() != 0 {
            panic!("Cannot set an input texture for a node that has parents");
        }
        self.input_texture = Some(texture);
    }


    pub fn get_texture(&self) -> Texture2D {
        self.texture.clone()
    }
}
