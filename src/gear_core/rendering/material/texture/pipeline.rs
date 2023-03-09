use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use cgmath::{Matrix4, Vector2, Vector3, Vector4};

use shader_pipeline_node::NodeType;

use crate::{ComputeShader, Material, Mesh, MeshRenderer, ShaderProgram, Texture2D};
use crate::pipeline::shader_pipeline_node::ShaderPipelineNode;

mod shader_pipeline_node;

pub enum ShaderPipelineNodeInput {
    Texture(Texture2D),
    // (node_name, name of the texture for the eventual compute shader node)
    Nodes(String, Option<String>),
}

pub struct ShaderPipeline {
    // (NodeName -> (NodeObject, RequireUpdate))
    nodes: HashMap<String, (ShaderPipelineNode, bool)>,
    links: HashMap<String, HashMap<String, ShaderPipelineNodeInput>>,
    children: HashMap<String, Vec<String>>,

    mesh_renderer: MeshRenderer,
}

impl ShaderPipeline {
    pub fn new() -> Self {
        let plane_mesh = Mesh::plane(Vector3::unit_x()*2., Vector3::unit_y()*2.);
        let material = Material::from_program("copy_shader"); // todo Brice: ne pas utiliser ca
        let mesh_renderer = MeshRenderer::new(&plane_mesh, material);
        ShaderPipeline {
            nodes: Default::default(),
            links: Default::default(),
            children: Default::default(),

            mesh_renderer,
        }
    }

    pub fn add_render_node(&mut self, node_name: &str, dimensions: (i32, i32), shader: ShaderProgram) {
        self.nodes.insert(node_name.parse().unwrap(), (ShaderPipelineNode::new_frag(dimensions, shader), true));
        self.links.insert(node_name.parse().unwrap(), Default::default());
    }

    pub fn add_compute_node(&mut self, node_name: &str, compute_shader: ComputeShader) {
        self.nodes.insert(node_name.parse().unwrap(), (ShaderPipelineNode::new_compute(compute_shader), true));
        self.links.insert(node_name.parse().unwrap(), Default::default());
    }

    pub fn link_render_to_node(&mut self, output_node_name: &str, input_tex_name: &str, input_node_name: &str) {
        self.links.get_mut(input_node_name)
            .expect(&*format!("Node {} not found", input_node_name))
            .insert(input_tex_name.to_string(), ShaderPipelineNodeInput::Nodes(output_node_name.to_string(), None));
        match self.children.get_mut(output_node_name) {
            Some(vec) => { vec.push(input_node_name.to_string()) },
            None => {
                self.children.insert(output_node_name.to_string(), vec!(input_node_name.to_string()));
            }
        }
    }

    pub fn link_compute_to_node(&mut self, output_node_name: &str, output_tex_name: &str, input_tex_name: &str, input_node_name: &str) {
        self.links.get_mut(input_node_name)
            .expect(&*format!("Node {} not found", input_node_name))
            .insert(input_tex_name.to_string(), ShaderPipelineNodeInput::Nodes(output_node_name.to_string(), Some(output_tex_name.to_string())));
        match self.children.get_mut(output_node_name) {
            Some(vec) => { vec.push(input_node_name.to_string()) },
            None => {
                self.children.insert(output_node_name.to_string(), vec!(input_node_name.to_string()));
            }
        }
    }

    pub fn set_input_texture(&mut self, tex_name: &str, texture: Texture2D, input_node_name: &str) {
        self.links.get_mut(input_node_name)
            .expect(&*format!("Node {} not found", input_node_name))
            .insert(tex_name.to_string(), ShaderPipelineNodeInput::Texture(texture));
    }


    pub fn compute(&mut self, shader_node_name: &str)  {
        let mut node_hm = self.links.remove(shader_node_name).unwrap();
        for (_, node_input) in &mut node_hm {
            match node_input {
                ShaderPipelineNodeInput::Nodes(output_node_name, _) => {
                    self.compute(&output_node_name);
                },
                _ => (),
            }
        }
        self.links.insert(shader_node_name.to_string(), node_hm);

        let require_update = self.node_require_update_mut(shader_node_name);
        if !*require_update {
            return;
        }
        *require_update = false;

        self.get_node(shader_node_name).execute(
            &self.mesh_renderer,
            self.links.get(shader_node_name).expect(&*format!("no link found for {}", shader_node_name)),
            &self.nodes
        );

        match self.children.get(shader_node_name) {
            Some(children) => {
                for child_name in children {
                    let mut child_node = self.nodes.remove(child_name).unwrap();
                    child_node.1 = true;
                    self.nodes.insert(child_name.to_string(), child_node);
                }
            }
            None => (),
        }

    }

    pub fn get_texture(&self, shader_node_name: &str, tex_name: &Option<String>) -> Texture2D {
        self.get_node(shader_node_name).get_texture(tex_name)
    }

    pub fn invalidate(&mut self, node_name: &str) {
        self.nodes.get_mut(node_name).unwrap().1 = true;
    }

    pub fn get_node(&self, node_name: &str) -> &ShaderPipelineNode {
        &self.nodes.get(node_name).expect("Trying to access a non existing node").0
    }

    pub fn get_node_mut(&mut self, node_name: &str) -> &mut ShaderPipelineNode {
        &mut self.nodes.get_mut(node_name).expect(&*format!("Trying to access a non existing node : {node_name}")).0
    }

    #[allow(dead_code)]
    fn node_require_update(&self, node_name: &str) -> &bool {
        &self.nodes.get(node_name).expect("Trying to access a non existing node").1
    }

    fn node_require_update_mut(&mut self, node_name: &str) -> &mut bool {
        &mut self.nodes.get_mut(node_name).expect("Trying to access a non existing node").1
    }

    pub fn set_int(&mut self, node_name: &str, name: &str, val: i32) {
        self.get_node_mut(node_name).set_int(name, val);
    }

    pub fn set_float(&mut self, node_name: &str, name: &str, val: f32) {
        self.get_node_mut(node_name).set_float(name, val);
    }

    pub fn set_vec2(&mut self, node_name: &str, name: &str, val: Vector2<f32>) {
        self.get_node_mut(node_name).set_vec2(name, val);
    }

    pub fn set_vec3(&mut self, node_name: &str, name: &str, val: Vector3<f32>) {
        self.get_node_mut(node_name).set_vec3(name, val);
    }

    pub fn set_vec4(&mut self, node_name: &str, name: &str, val: Vector4<f32>) {
        self.get_node_mut(node_name).set_vec4(name, val);
    }

    pub fn set_mat4(&mut self, node_name: &str, name: &str, val: Matrix4<f32>) {
        self.get_node_mut(node_name).set_mat4(name, val);
    }

}