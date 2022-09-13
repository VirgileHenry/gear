extern crate cgmath;

pub struct Mesh {
    
}


enum MeshType {
    Owned(Mesh),
    // Shared(&Mesh),
}


pub struct MeshRenderer {
    mesh: MeshType,
    
}