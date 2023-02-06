use std::collections::HashMap;
use cgmath::{Matrix4, Vector3};
use crate::pipeline::shader_pipeline_node::{ShaderPipelineNode};
use crate::{Material, Mesh, MeshRenderer, NoParamMaterialProperties, ShaderProgram, Texture2D};

mod shader_pipeline_node;

pub enum ShaderPipelineNodeInput {
    NotSet, // enlever ?
    Texture(String, Texture2D),
    // (texture name, node_name)
    Nodes(HashMap<String, String>),
}

pub struct ShaderPipeline {
    nodes: HashMap<String, ShaderPipelineNode>,
    links: HashMap<String, ShaderPipelineNodeInput>,

    mesh_renderer: MeshRenderer,
}

impl ShaderPipeline {
    pub fn new() -> Self {
        let plane_mesh = Mesh::plane(Vector3::unit_x()*2., Vector3::unit_y()*2.);
        let material = Material::from_program("copy_shader", Box::new(NoParamMaterialProperties{})); // todo Brice: ne pas utiliser ca
        let mesh_renderer = MeshRenderer::new(&plane_mesh, material);
        ShaderPipeline {
            nodes: Default::default(),
            links: Default::default(),

            mesh_renderer,
        }
    }

    pub fn add_node(&mut self, node_name: &str, dimensions: (i32, i32), shader: ShaderProgram) {
        self.nodes.insert(node_name.parse().unwrap(), ShaderPipelineNode::new(dimensions, shader));
        self.links.insert(node_name.parse().unwrap(), ShaderPipelineNodeInput::NotSet);
    }

    pub fn link_nodes(&mut self, output_node_name: &str, tex_name: &str, input_node_name: &str) {
        match self.links.get_mut(input_node_name).expect(&*format!("Node {} not found", input_node_name)) {
            ShaderPipelineNodeInput::Nodes(links) => {
                links.insert(
                    tex_name.parse().unwrap(),
                    output_node_name.parse().unwrap(),
                );
            }
            ShaderPipelineNodeInput::NotSet => {
                let mut hm = HashMap::<String, String>::new();
                hm.insert(
                    tex_name.parse().unwrap(),
                    output_node_name.parse().unwrap(),
                );
                self.links.insert(input_node_name.parse().unwrap(), ShaderPipelineNodeInput::Nodes(hm));
            }
            _ => panic!("The node {} only accept textures as input", input_node_name)
        }
    }

    pub fn set_input_texture(&mut self, tex_name: &str, texture: Texture2D, input_node_name: &str) {
        match self.links.get_mut(input_node_name).expect(&*format!("Node {} not found", input_node_name)) {
            ShaderPipelineNodeInput::NotSet | ShaderPipelineNodeInput::Texture(_, _) => {
                self.links.insert(input_node_name.parse().unwrap(), ShaderPipelineNodeInput::Texture(tex_name.parse().unwrap(), texture));
            }
            _ => panic!("The node {} doesn't accept textures as input", input_node_name)
        }
    }

    pub unsafe fn compute(&self, shader_node_name: &str) {
        // making sure that each node is compted with the right order
        match self.links.get(shader_node_name).unwrap() {
            ShaderPipelineNodeInput::Nodes(hm) => {
                for (_, node) in hm {
                    self.compute(&node);
                }
            }
            _ => ()
        }

        self.get_node(shader_node_name).compute(&self.mesh_renderer, self.links.get(shader_node_name).unwrap(), &self.nodes);
    }

    pub fn get_texture(&self, shader_node_name: &str) -> Texture2D {
        self.get_node(shader_node_name).get_texture()
    }

    fn get_node(&self, node_name: &str) -> &ShaderPipelineNode {
        self.nodes.get(node_name).expect("Trying to access an unexisting node")
    }

    fn get_mut_node(&mut self, node_name: &str) -> &mut ShaderPipelineNode {
        self.nodes.get_mut(node_name).expect("Trying to access an unexisting node")
    }

    pub fn set_int(&mut self, node_name: &str, name: &str, val: i32) {
        self.get_mut_node(node_name).set_int(name, val);
    }

    pub fn set_float(&mut self, node_name: &str, name: &str, val: f32) {
        self.get_mut_node(node_name).set_float(name, val);
    }

    pub fn set_vec3(&mut self, node_name: &str, name: &str, val: Vector3<f32>) {
        self.get_mut_node(node_name).set_vec3(name, val);
    }

    pub fn set_mat4(&mut self, node_name: &str, name: &str, val: Matrix4<f32>) {
        self.get_mut_node(node_name).set_mat4(name, val);
    }
}

