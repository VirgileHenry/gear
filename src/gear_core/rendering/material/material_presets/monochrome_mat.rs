use crate::{Color, ShaderProgram};
use crate::material::MaterialProperties;

pub struct MonochromeMaterialProperties {
    pub color: Color,
}

impl MaterialProperties for MonochromeMaterialProperties {
    fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        shader.set_vec3("color", self.color.as_vector());
    }
}
