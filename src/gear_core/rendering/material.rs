
// OpenGL dependency !!

use gl::DeleteTextures;

use super::shaders::{ShaderProgramRef, ShaderProgram};
use crate::gear_core::rendering::opengl::color::Color;
use crate::material::texture::Texture;


pub struct Material {
    /// shader program to use
    pub program_ref: ShaderProgramRef,
    // needs params depending on the program. Generics ?
    properties: Box<dyn MaterialProperties>,
    // textures id
    textures: Vec<Texture>,
}

impl Material {
    pub fn from_program(program: &ShaderProgram, properties: Box<dyn MaterialProperties>) -> Material {
        Material {
            program_ref: ShaderProgramRef::new(program),
            properties: properties,
            textures: Vec::new(),
        }
    }

    pub fn from_ref(program_ref: ShaderProgramRef, properties: Box<dyn MaterialProperties>) -> Material {
        Material {
            program_ref: program_ref,
            properties: properties,
            textures: Vec::new(),
        }
    }

    pub fn attach_texture(&mut self, file_name: &str) {
        self.textures.push(Texture::new(file_name));
    }

    #[inline]
    pub fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        self.properties.set_properties_to_shader(shader);
    }

    pub unsafe fn bind_textures(&self, mut texture_index: u32) -> u32 {
        for texture in &self.textures {
            gl::ActiveTexture(texture_index);
            gl::BindTexture(gl::TEXTURE_2D, texture.get_id());
            texture_index += 1;
        }
        texture_index + 1
    }
}

pub trait MaterialProperties {
    fn set_properties_to_shader(&self, shader: &ShaderProgram);
}



pub mod material_presets;
mod texture;
