use std::collections::HashMap;

use crate::{Color, ShaderProgram, Texture2D};
use crate::material::MaterialProperties;

pub struct TextureAttachmentProp {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureAttachmentProp {
    pub fn attach_texture(&mut self, name: &str, texture: Texture2D) {
        self.textures.insert(name.to_string(), texture);
    }
    pub fn pop_texture(&mut self, name: &str) -> Texture2D {
        self.textures.remove(name)
            .expect(&*format!("Texture not {name} found"))
    }
    pub unsafe fn bind_textures(&self, mut texture_index: u32) -> u32 {
        for texture in &self.textures {
            gl::ActiveTexture(texture_index);
            texture.bind();
            texture_index += 1;
        }
        texture_index + 1
    }
}

impl MaterialProperties for TextureAttachmentProp {
    fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        unsafe { self.bind_textures(gl::TEXTURE0); }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
