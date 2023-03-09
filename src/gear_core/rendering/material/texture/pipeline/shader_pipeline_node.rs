use std::collections::HashMap;

use cgmath::{Matrix4, Vector2, Vector3, Vector4};
use gl::types::{GLenum, GLuint};

use crate::{ComputeShader, MeshRenderer, ShaderPipelineNodeInput, ShaderProgram};
use crate::gear_core::material::texture::{Texture2D, TexturePresets};
use crate::pipeline::shader_pipeline_node::NodeType::{Compute, Render};

/// Wrapper used to store the uniforms for a node
struct ShaderPipelineNodeParam {
    pub ints: HashMap<String, i32>,
    pub floats: HashMap<String, f32>,
    pub vec2s: HashMap<String, Vector2<f32>>,
    pub vec3s: HashMap<String, Vector3<f32>>,
    pub vec4s: HashMap<String, Vector4<f32>>,
    pub mat4s: HashMap<String, Matrix4<f32>>,
}

/// A node is either linked to a compute shader or a frag shader
/// In case of a compute shader multiple textures can be bound to the node
/// those textures must be named in order to select them
/// A frag shader has only one output texture and a framebuffer id
pub enum NodeType {
    Compute(ComputeShader),
    Render(ShaderProgram, GLuint, Texture2D),
}

pub struct ShaderPipelineNode {
    param: ShaderPipelineNodeParam,
    node_type: NodeType,
}

impl ShaderPipelineNodeParam {
    pub fn new() -> Self {
        Self {
            ints: Default::default(),
            floats: Default::default(),
            vec2s: Default::default(),
            vec3s: Default::default(),
            vec4s: Default::default(),
            mat4s: Default::default(),
        }
    }
}

impl ShaderPipelineNode {

    /// creates a new node with a fragment shader
    pub fn new_frag(dimensions: (i32, i32),
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
                texture.unwrap_id(),
                0,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT {
                panic!("Framebuffer is not complete !")
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // unbind the framebuffer until needed
        }

        Self{
            param: ShaderPipelineNodeParam::new(),

            node_type: Render(shader, framebuffer_id, texture),
        }
    }

    /// creates a new node with a compute shader
    pub fn new_compute(shader: ComputeShader) -> Self {
        Self{
            param: ShaderPipelineNodeParam::new(),

            node_type: Compute(shader),
        }
    }

    pub fn execute(&self, plane: &MeshRenderer, texture_map: &HashMap<String, ShaderPipelineNodeInput>, pipeline_nodes: &HashMap<String, (ShaderPipelineNode, bool)>) {
        unsafe {
            match self.node_type {
                Compute(_) => self.compute(texture_map, pipeline_nodes),
                Render(_, _, _) => self.render(plane, texture_map, pipeline_nodes),
            }
        }
    }

    unsafe fn render(&self, plane: &MeshRenderer, texture_map: &HashMap<String, ShaderPipelineNodeInput>, pipeline_nodes: &HashMap<String, (ShaderPipelineNode, bool)>) {

        // Binding current node framebuffer
        let shader = match &self.node_type {
            Render(shader, framebuffer_id, texture) => {
                gl::BindFramebuffer(gl::FRAMEBUFFER, *framebuffer_id);
                gl::Viewport(0, 0, texture.dimensions.0, texture.dimensions.1);
                shader
            }
            _ => {
                panic!("Render should only be called on a render node");
            }
        };

        // Setting shader parameters
        shader.set_used();

        let mut current_active_tex = 0;

        // Set up input textures
        for (input_texture_name, map) in texture_map {
            shader.set_int(input_texture_name, current_active_tex as i32);
            match map {
                ShaderPipelineNodeInput::Nodes(output_node_name, output_tex_name) => {
                    let input_node = &pipeline_nodes.get(output_node_name).unwrap().0;
                    // set up input texture
                    gl::ActiveTexture(current_active_tex+gl::TEXTURE0);
                    input_node.get_texture(output_tex_name).bind();
                    current_active_tex += 1;
                }
                ShaderPipelineNodeInput::Texture(output_texture) => {
                    gl::ActiveTexture(gl::TEXTURE0+current_active_tex);
                    output_texture.bind();
                    current_active_tex += 1;
                }
            }
        }

        self.bind_all_params(shader);
        // Drawing result onto node's texture
        plane.draw(shader);

    }

    pub unsafe fn compute(&self, texture_map: &HashMap<String, ShaderPipelineNodeInput>, pipeline_nodes: &HashMap<String, (ShaderPipelineNode, bool)>) {

        let mut current_active_tex: GLenum = 0;

        // Binding current node framebuffer
        let shader = match &self.node_type {
            Compute(compute_shader) => {
                current_active_tex += compute_shader.get_texture_count();
                compute_shader.set_used()
            }
            _ => {
                panic!("Render should only be called on a render node");
            }
        };

        // Set up input textures
        for (input_texture_name, map) in texture_map {
            shader.set_int(input_texture_name, current_active_tex as i32);
            match map {
                ShaderPipelineNodeInput::Nodes(output_node_name, output_tex_name) => {
                    let input_node = &pipeline_nodes.get(output_node_name).unwrap().0;
                    // set up input texture
                    gl::ActiveTexture(current_active_tex+gl::TEXTURE0);
                    input_node.get_texture(output_tex_name).bind();
                    current_active_tex += 1;
                }
                ShaderPipelineNodeInput::Texture(output_texture) => {
                    gl::ActiveTexture(gl::TEXTURE0+current_active_tex);
                    output_texture.bind();
                    current_active_tex += 1;
                }
            }
        }

        self.bind_all_params(shader);
        // Drawing result onto node's texture
        if let Compute(compute_shader) = &self.node_type {
            compute_shader.begin_computation();
            compute_shader.wait_for_result();
        }
    }


    /// Returns the resulting texture of this node
    /// An optional name is required to select the output of a compute shader node
    pub fn get_texture(&self, tex_name: &Option<String>) -> Texture2D {
        match tex_name {
            Some(name) => {
                match &self.node_type {
                    Compute(compute_shader) => {
                        compute_shader.get_texture(name)
                    }
                    _ => panic!("Only compute shader node have named textures")
                }
            },
            None => {
                match self.node_type {
                    Render(_, _, texture) => {
                        texture.clone()
                    }
                    Compute(_) => {
                        panic!("A name is required to access a compute shader texture")
                    }
                }
            },
        }
    }


    /* PARAM UTILITY METHODS */

    pub fn set_int(&mut self, name: &str, val: i32) {
        self.param.ints.insert(name.parse().unwrap(),val);
    }

    pub fn set_float(&mut self, name: &str, val: f32) {
        self.param.floats.insert(name.parse().unwrap(), val);
    }

    pub fn set_vec2(&mut self, name: &str, val: Vector2<f32>) {
        self.param.vec2s.insert(name.parse().unwrap(), val);
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

    /// Binds all uniforms needed for the node's shader
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
        for (name, val) in &self.param.vec2s {
            shader.set_vec2(name, *val);
        }
        for (name, val) in &self.param.vec3s {
            shader.set_vec3(name, *val);
        }
        for (name, val) in &self.param.vec4s {
            shader.set_vec4(name, *val);
        }
    }

    pub fn get_compute_shader_mut(&mut self) -> &mut ComputeShader {
        match &mut self.node_type {
            Compute(shader) => shader,
            _ => panic!("This node is not a compute shader !"),
        }
    }

    pub fn get_node_type_mut(&mut self) -> &mut NodeType {
        &mut self.node_type
    }

}