
// OpenGL dependency !!

use gl::DeleteTextures;

use super::shaders::{ShaderProgramRef, ShaderProgram};
use crate::gear_core::rendering::opengl::color::Color;
use crate::material::texture::Texture2D;


pub struct Material {
    /// shader program to use
    shader_program: String,
    // needs params depending on the program.
    properties: Box<dyn MaterialProperties>,
    // textures id
    textures: Vec<Texture2D>, // todo brice : ça dégage dans les mat prop ça
}

impl Material {
    pub fn from_program(program: &str, properties: Box<dyn MaterialProperties>) -> Material {
        Material {
            shader_program: program.to_string(),
            properties: properties,
            textures: Vec::new(),
        }
    }

    pub fn attach_texture(&mut self, texture: Texture2D) {
        self.textures.push(texture);
    }

    #[inline]
    pub fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        self.properties.set_properties_to_shader(shader);
    }

    pub unsafe fn bind_textures(&self, mut texture_index: u32) -> u32 {
        for texture in &self.textures {
            gl::ActiveTexture(texture_index);
            texture.bind();
            texture_index += 1;
        }
        texture_index + 1
    }

    pub fn get_program_name(&self) -> &str {
        &self.shader_program
    }

    pub fn get_mat_properties<T: 'static>(&mut self) -> Option<&mut T> {
        self.properties.as_any_mut().downcast_mut()
    }
}

pub trait MaterialProperties {
    fn set_properties_to_shader(&self, shader: &ShaderProgram);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}



pub mod material_presets;
pub mod texture;
