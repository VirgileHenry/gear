use crate::{Color, ShaderProgram};
use crate::material::MaterialProperties;

pub struct MonochromeMaterialProperties {
    pub color: Color,
}

impl MonochromeMaterialProperties {
    pub fn rgb(r: f32, g: f32, b: f32) -> MonochromeMaterialProperties {
        MonochromeMaterialProperties {
            color: Color::from_rgb(r, g, b),
        }
    }
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
