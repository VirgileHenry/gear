mod mesh_presets;

extern crate cgmath;
extern crate gl;

use crate::gear_core::rendering::{
    geometry::primitives::Vertex, 
};


pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<u32>,
}

