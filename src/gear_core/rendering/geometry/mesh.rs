mod mesh_presets;

extern crate cgmath;
extern crate gl;

use crate::gear_core::rendering::{
    geometry::vertex::Vertex, 
};
use crate::gear_core::resources::{load_static_mesh, MeshLoadingError};

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<u32>) -> Mesh {
        Mesh { vertices, triangles }
    }

    pub fn load(from: &str) -> Result<Mesh, MeshLoadingError> {
        load_static_mesh(from)
    }
}