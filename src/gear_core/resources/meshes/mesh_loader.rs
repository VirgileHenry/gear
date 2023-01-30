use crate::gear_core::rendering::geometry::mesh::Mesh;

pub enum MeshLoadingError {

}

pub fn load_static_mesh(path: &str) -> Result<Mesh, MeshLoadingError> {
    // the file format is as following : 
    // u32, u32 : number of vertices and triangles
    // for each vertex :
    // (f32, f32, f32), (f32, f32, f32), (f32, f32) : position, normal, uv
    // for each triangle :
    // (u32, u32, u32) : triangle
    unimplemented!()
}