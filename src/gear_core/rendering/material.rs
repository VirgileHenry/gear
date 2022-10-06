use super::shaders::{ShaderProgramRef, ShaderProgram};
use crate::gear_core::rendering::opengl::color::Color;

pub struct Material {
    /// shader program to use
    pub program_ref: ShaderProgramRef,
    // needs params depending on the program. Generics ?
    properties: Box<dyn MaterialProperties>,
}

impl Material {
    pub fn from_program(program: &ShaderProgram, properties: Box<dyn MaterialProperties>) -> Material {
        Material {
            program_ref: ShaderProgramRef::new(program),
            properties: properties,
        }
    }

    pub fn from_ref(program_ref: ShaderProgramRef, properties: Box<dyn MaterialProperties>) -> Material {
        Material {
            program_ref: program_ref,
            properties: properties,
        }
    }

    #[inline]
    pub fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        self.properties.set_properties_to_shader(shader);
    }
}

pub trait MaterialProperties {
    fn set_properties_to_shader(&self, shader: &ShaderProgram);
}

pub struct MonochromeMaterialProperties {
    pub color: Color,
}

impl MaterialProperties for MonochromeMaterialProperties {
    fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        shader.set_vec3("color", self.color.as_vector());
    }
}