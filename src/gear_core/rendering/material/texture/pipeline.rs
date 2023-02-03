use std::collections::HashMap;
use cgmath::{Matrix4, Vector3};
use gl::types::GLuint;
use crate::gear_core::material::texture::{Texture2D, TexturePresets};
use crate::{Material, Mesh, MeshRenderer, NoParamMaterialProperties, ShaderProgram};

pub enum ShaderPipelineNodeInput {
    NotSet,
    Texture(String, Texture2D),
    Nodes(HashMap<String, ShaderPipelineNode>),
}

pub struct ShaderPipelineNodeParam {
    pub ints: HashMap<String, i32>,
    pub floats: HashMap<String, f32>,
    pub vec3s: HashMap<String, Vector3<f32>>,
    pub mat4s: HashMap<String, Matrix4<f32>>,
}

impl ShaderPipelineNodeParam {
    pub fn new() -> Self {
        Self{
            ints: Default::default(),
            floats: Default::default(),
            vec3s: Default::default(),
            mat4s: Default::default(),
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
    // A node has either an input texture OR a non empty list of input nodes
    input: ShaderPipelineNodeInput,

    texture: Texture2D,
    shader: ShaderProgram,
    param: ShaderPipelineNodeParam,

    framebuffer_id: GLuint,
}

impl ShaderPipelineNode {

    pub fn new(dimensions: (i32, i32),
               shader: ShaderProgram) -> Self {

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
            input: ShaderPipelineNodeInput::NotSet,

            texture,
            shader,
            param: ShaderPipelineNodeParam::new(),

            framebuffer_id,
        }
    }

    pub fn add_input_node(&mut self, tex_name: &str, node: ShaderPipelineNode) {
        match &mut self.input {
            ShaderPipelineNodeInput::NotSet => {
                let mut hm = HashMap::new();
                hm.insert(tex_name.to_string(), node);
                self.input = ShaderPipelineNodeInput::Nodes(hm);
            },
            ShaderPipelineNodeInput::Nodes(hm) => { hm.insert(tex_name.to_string(), node); },
            _ => panic!("Cannot add node"),
        }
    }

    pub fn set_input_texture(&mut self, name: &str, texture: Texture2D) {
        match &self.input {
            ShaderPipelineNodeInput::NotSet => {
                self.input = ShaderPipelineNodeInput::Texture(name.parse().unwrap(), texture);
            },
            ShaderPipelineNodeInput::Texture(_, _) => panic!("Texture already set"),
            _ => panic!("Cannot add node"),
        }
    }


    /// Set current_input to None to use the pipeline
    pub unsafe fn compute(&self) {
        let plane_mesh = Mesh::plane(Vector3::unit_x()*2., Vector3::unit_y()*2.);
        let material = Material::from_program("copy_shader", Box::new(NoParamMaterialProperties{})); // todo Brice: ne pas utiliser ca
        let mesh_renderer = MeshRenderer::new(&plane_mesh, material);
        self.compute_rec_with_plane(&mesh_renderer);
    }

    unsafe fn compute_rec_with_plane(&self, plane: &MeshRenderer) {

        // Binding current node framebuffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);

        // Setting shader parameters
        self.shader.set_used();

        let mut current_active_tex = 0;
        gl::ActiveTexture(gl::TEXTURE0 + current_active_tex);
        // Set up input textures
        match &self.input {
            ShaderPipelineNodeInput::Texture(name, tex) => {
                tex.bind();self.shader.set_int(name, current_active_tex as i32);
            },
            ShaderPipelineNodeInput::Nodes(nodes) => {
                for (name, input_node) in nodes.into_iter() {
                    input_node.compute_rec_with_plane(plane);
                    // set up input texture
                    gl::ActiveTexture(current_active_tex+gl::TEXTURE0);
                    input_node.get_texture().bind();
                    self.shader.set_int(name, current_active_tex as i32);
                    current_active_tex += 1;
                }
            },
            _ => (),
        }

        self.bind_all_params(&self.shader);

        // Drawing result onto node's texture
        plane.draw(&self.shader);
    }



    pub fn get_texture(&self) -> Texture2D {
        self.texture.clone()
    }


    /* PARAM UTILITY METHODS */

    pub fn set_int(&mut self, name: &str, val: i32) {
        self.param.ints.insert(name.parse().unwrap(),val);
    }

    pub fn set_float(&mut self, name: &str, val: f32) {
        self.param.floats.insert(name.parse().unwrap(), val);
    }

    pub fn set_vec3(&mut self, name: &str, val: Vector3<f32>) {
        self.param.vec3s.insert(name.parse().unwrap(), val);
    }

    pub fn set_mat4(&mut self, name: &str, val: Matrix4<f32>) {
        self.param.mat4s.insert(name.parse().unwrap(), val);
    }

    pub unsafe fn bind_all_params(&self, shader: &ShaderProgram) {
        for (name, val) in &self.param.ints {
            shader.set_int(name, *val);
        }

        for (name, val) in &self.param.floats {
            shader.set_float(name, *val);
        }

        for (name, val) in &self.param.mat4s {
            shader.set_mat4(name, *val);
        }

        for (name, val) in &self.param.vec3s {
            shader.set_vec3(name, *val);
        }
    }

}
