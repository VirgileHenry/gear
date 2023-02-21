use crate::{Color, ShaderProgram};
use crate::material::MaterialProperties;

pub struct TexturedMaterialProperties {
    pub texture: Texture2D,
}

impl MaterialProperties for MonochromeMaterialProperties {
    fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        shader.set_vec3("color", self.color.as_vector());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
