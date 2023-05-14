use std::{fs::File, io::{BufReader, Read}};

use crate::{gear_core::rendering::geometry::mesh::Mesh, Vertex};

#[derive(Debug)]
pub enum MeshLoadingError {
    FileNotFound,
    InvalidData,
    NotEnoughData,
}

pub fn load_static_mesh(path: &str) -> Result<Mesh, MeshLoadingError> {
    let mut reader = BufReader::new(
        match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(MeshLoadingError::FileNotFound),
        }
    );
    let mut u32_buffer = [0u8; 4];
    let vertex_count: usize = match reader.read_exact(&mut u32_buffer) {
        Err(_) => return Err(MeshLoadingError::InvalidData),
        Ok(_) => match u32::from_ne_bytes(u32_buffer).try_into() {
            Ok(value) => value,
            Err(_) => return Err(MeshLoadingError::NotEnoughData),
        },
    };
    let triangle_count: usize = match reader.read_exact(&mut u32_buffer) {
        Err(_) => return Err(MeshLoadingError::InvalidData),
        Ok(_) => match u32::from_ne_bytes(u32_buffer).try_into() {
            Ok(value) => value,
            Err(_) => return Err(MeshLoadingError::NotEnoughData),
        },
    };

    let mut vertices = Vec::with_capacity(vertex_count);
    let mut vertex_buffer = [0u8; std::mem::size_of::<Vertex>()];
    for _ in 0..vertex_count {
        match reader.read_exact(&mut vertex_buffer) {
            Ok(_) => {
                vertices.push(match Vertex::load(&vertex_buffer) {
                    Ok(vert) => vert,
                    Err(e) => {
                        return Err(MeshLoadingError::InvalidData)
                    },
                })
            },
            Err(_) => return Err(MeshLoadingError::NotEnoughData),
        }
    }

    let mut triangles = Vec::with_capacity(3 * triangle_count);
    let mut triangle_buffer = [0u8; std::mem::size_of::<u32>()];
    for i in 0..3*triangle_count {
        match reader.read_exact(&mut triangle_buffer) {
            Ok(_) => triangles.push(u32::from_ne_bytes(triangle_buffer)),
            Err(_) => return Err(MeshLoadingError::NotEnoughData),
        }
    }


    Ok(Mesh::new(vertices, triangles))
}