use std::collections::HashMap;
use std::iter::Map;
use std::os::unix::fs::FileTypeExt;

use cgmath::{Matrix4, Vector3, Vector4};
use gl::types::GLuint;

use crate::{Material, Mesh, MeshRenderer, NoParamMaterialProperties, ShaderPipelineNodeInput, ShaderProgram};
use crate::gear_core::material::texture::{Texture2D, TexturePresets};

pub struct ShaderPipelineNodeParam {
    pub ints: HashMap<String, i32>,
    pub floats: HashMap<String, f32>,
    pub vec3s: HashMap<String, Vector3<f32>>,
    pub vec4s: HashMap<String, Vector4<f32>>,
    pub mat4s: HashMap<String, Matrix4<f32>>,
}

// Shader Pipeline node
pub struct ShaderPipelineNode {
    texture: Texture2D,
    shader: ShaderProgram,
    param: ShaderPipelineNodeParam,

    framebuffer_id: GLuint,
}

impl ShaderPipelineNodeParam {
    pub fn new() -> Self {
        Self{
            ints: Default::default(),
            floats: Default::default(),
            vec3s: Default::default(),
            vec4s: Default::default(),
            mat4s: Default::default(),
        }
    }
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
            texture,
            shader,
            param: ShaderPipelineNodeParam::new(),

            framebuffer_id,
        }
    }


    pub unsafe fn compute(&self, plane: &MeshRenderer, texture_map: &ShaderPipelineNodeInput, pipeline_nodes: &HashMap<String, ShaderPipelineNode>) {


        // Binding current node framebuffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);

        // Setting shader parameters
        self.shader.set_used();

        gl::Viewport(0, 0, self.texture.dimensions.0, self.texture.dimensions.1);

        let mut current_active_tex = 0;
        gl::ActiveTexture(gl::TEXTURE0 + current_active_tex);

        // Set up input textures
        match texture_map {
            ShaderPipelineNodeInput::Texture(name, tex) => {
                tex.bind();
                self.shader.set_int(name, current_active_tex as i32);
            },
            ShaderPipelineNodeInput::Nodes(nodes) => {
                for (name, input_node_name) in nodes.into_iter() {
                    let input_node = pipeline_nodes.get(input_node_name).unwrap();
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

    pub fn set_vec4(&mut self, name: &str, val: Vector4<f32>) {
        self.param.vec4s.insert(name.parse().unwrap(), val);
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
        for (name, val) in &self.param.vec4s {
            shader.set_vec4(name, *val);
        }
    }

}