use crate::ShaderProgram;
use crate::material::MaterialProperties;

/// Defines a material that has no parameters to pass to it's associated shader
pub struct NoParamMaterialProperties {}

impl MaterialProperties for NoParamMaterialProperties {
    fn set_properties_to_shader(&self, _shader: &ShaderProgram) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
