use crate::{Color, ShaderProgram};
use crate::material::MaterialProperties;

/// Defines a material that has no parameters to pass to it's associated shader
pub struct NoParamMaterialProperties {}

impl MaterialProperties for NoParamMaterialProperties {
    fn set_properties_to_shader(&self, shader: &ShaderProgram) {}
}
