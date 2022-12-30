use cgmath::Vector3;
use foundry::{create_entity, EntityRef, World};
use crate::{DefaultOpenGlRenderer, Material, Mesh, MeshRenderer, MeshType, NoParamMaterialProperties, Renderer, ShaderProgram, ShaderProgramRef, UI_DEFAULT_VERT_SHADER, UI_UNLIT_UV_FRAG_SHADER};
use foundry::*;
use crate::Transform;


pub struct UILayer {
    entity: EntityRef,
}

impl UILayer {
    pub fn new(world: &mut World, shader_program_ref: ShaderProgramRef) -> Self {

        let mesh = Mesh::plane(Vector3::unit_x()*2., Vector3::unit_y()*2.);
        let material = Material::from_ref(shader_program_ref, Box::new(NoParamMaterialProperties{}));
        let mesh_renderer = MeshRenderer::new(MeshType::Owned(mesh), material);

        // the mesh is placed on the z=-1 side of the cube [-1, 1]^3
        let entity = create_entity!(&mut world.components; Transform::origin().translated(0.0, 0.0, -1.0), mesh_renderer);
        Self {
            entity,
        }

    }
}
