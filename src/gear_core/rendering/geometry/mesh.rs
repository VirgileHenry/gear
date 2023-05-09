extern crate cgmath;
extern crate gl;

use cgmath::Vector3;

use crate::gear_core::rendering::geometry::vertex::Vertex;
use crate::gear_core::resources::{load_static_mesh, MeshLoadingError};

mod mesh_presets;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<u32>,
    pub bounding_box: Option<([Vector3<f32>; 2])>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<u32>) -> Mesh {
        Mesh { vertices, triangles, bounding_box: None }
    }

    pub fn set_bounding_box(&mut self, bounding_box: Option<([Vector3<f32>; 2])>) {
        self.bounding_box = bounding_box;
    }

    pub fn load(from: &str) -> Result<Mesh, MeshLoadingError> {
        load_static_mesh(from)
    }
}