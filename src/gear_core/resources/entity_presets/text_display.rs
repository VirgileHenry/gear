use cgmath::Vector3;
use foundry::{ComponentTable, create_entity, EntityRef};

use crate::*;
use crate::{Material, Mesh, MeshRenderer, Transform};
use crate::gear_core::mesh;

pub fn new_text_display(components: &mut ComponentTable, text: &str, transform: &Transform) -> EntityRef {
    let text_plane = Mesh::plane(Vector3::unit_x(), Vector3::unit_y());
    let text_display_mat = Material::from_program("simple_plane_text_display")
        .with_property(TextDisplayProp::new_default(&text.to_string()));
    let mesh_renderer = MeshRenderer::new(&text_plane, text_display_mat); // todo use common vao
    create_entity!(components; mesh_renderer)
}
