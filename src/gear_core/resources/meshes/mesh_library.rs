use std::{collections::HashMap, rc::Rc};

use crate::MeshRenderingBuffers;

use super::mesh_loader::{load_static_mesh, MeshLoadingError};




pub struct MeshLibrary {
    library: HashMap<String, Rc<MeshRenderingBuffers>>,
}

impl MeshLibrary {
    pub fn new() -> MeshLibrary {
        MeshLibrary { library: HashMap::new() }
    }

    pub fn add(&mut self, file_path: &str, mesh_name: String) -> Result<(), MeshLoadingError> {
        self.library.insert(mesh_name, Rc::new(MeshRenderingBuffers::from(&load_static_mesh(file_path)?.into())));
        Ok(())
    }

    pub fn get_rendering_buffers(&mut self, mesh_name: String) -> Option<Rc<MeshRenderingBuffers>> {
        match self.library.get(&mesh_name) {
            Some(buffers) => Some(buffers.clone()),
            None => None,
        }
    }
}