use std::collections::HashMap;

use crate::{ShaderProgram, Texture2D};
use crate::material::MaterialProperties;

pub struct TextureAttachmentProp {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureAttachmentProp {
    pub fn new() -> Self {
        Self{
            textures: HashMap::new(),
        }
    }
    pub fn attach_texture(&mut self, name: &str, texture: Texture2D) {
        self.textures.insert(name.to_string(), texture);
    }
    pub fn remove_texture(&mut self, name: &str) -> Option<Texture2D> {
        self.textures.remove(name)
    }
    pub unsafe fn bind_textures(&self, shader: &ShaderProgram) {
        let mut texture_index_offset = 0;
        for (name, texture) in &self.textures {
            shader.set_int(name, texture_index_offset);
            gl::ActiveTexture(gl::TEXTURE0 + texture_index_offset as u32);
            texture.bind();
            texture_index_offset += 1;
        }
    }
}

impl MaterialProperties for TextureAttachmentProp {
    fn set_properties_to_shader(&self, shader: &ShaderProgram) {
        unsafe { self.bind_textures(shader); }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
