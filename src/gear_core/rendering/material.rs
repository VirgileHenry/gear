
use std::{collections::HashMap, any::{TypeId, Any}};

use super::shaders::{ShaderProgram};


pub struct Material {
    /// shader program to use
    shader_program: String,
    // needs params depending on the program.
    properties: HashMap<TypeId, Box<dyn MaterialProperties>>,
}

impl Material {
    pub fn from_program(program: &str) -> Material {
        Material {
            shader_program: program.to_string(),
            properties: HashMap::new(),
        }
    }

    pub fn with_property<T: MaterialProperties + 'static>(mut self, property: T) -> Self {
        self.properties.insert(property.type_id(), Box::new(property));
        self
    }

    pub fn add_material_property<T: MaterialProperties + 'static>(&mut self, property: T) {
        self.properties.insert(property.type_id(), Box::new(property));
    }


    pub fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        for (_key, property) in self.properties.iter() {
            property.set_properties_to_shader(shader);
        }
    }

    pub fn get_program_name(&self) -> &str {
        &self.shader_program
    }

    pub fn get_mat_properties<T: 'static>(&mut self) -> Option<&mut T> {
        match self.properties.get_mut(&TypeId::of::<T>()) {
            Some(prop) => prop.as_any_mut().downcast_mut(),
            None => None,
        }
    }

}

pub trait MaterialProperties {
    fn set_properties_to_shader(&self, shader: &ShaderProgram);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}



pub mod material_presets;
pub mod texture;
