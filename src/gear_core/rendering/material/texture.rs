use std::ffi::c_void;

use cgmath::num_traits::clamp;
use gl::types::{GLenum, GLint};
use image::RgbaImage;

pub use pipeline::*;

#[derive(Copy, Clone)]
pub struct TexturePresets {
    pub wrap_s: GLenum,
    pub wrap_t: GLenum,

    pub mag_filter: GLenum,
    pub min_filter: GLenum,

    pub internal_format: GLenum,
    pub format: GLenum,
}

impl TexturePresets {
    pub fn default() -> Self {
        Self {
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            min_filter: gl::LINEAR_MIPMAP_LINEAR,
            mag_filter: gl::LINEAR,
            internal_format: gl::RGBA,
            format: gl::RGBA,
        }
    }

    pub fn color_default() -> Self {
        Self {
            wrap_s: gl::CLAMP_TO_BORDER,
            wrap_t: gl::CLAMP_TO_BORDER,
            min_filter: gl::LINEAR,
            mag_filter: gl::LINEAR,
            internal_format: gl::RGBA32F,
            format: gl::RGBA,
        }
    }

    pub fn depth_default() -> Self {
        Self {
            wrap_s: gl::CLAMP_TO_EDGE,
            wrap_t: gl::CLAMP_TO_EDGE,
            min_filter: gl::NEAREST,
            mag_filter: gl::NEAREST,
            internal_format: gl::DEPTH_COMPONENT,
            format: gl::DEPTH_COMPONENT,
        }
    }

    pub fn pipeline_default() -> Self {
        Self {
            wrap_s: gl::CLAMP_TO_EDGE,
            wrap_t: gl::CLAMP_TO_EDGE,
            min_filter: gl::LINEAR,
            mag_filter: gl::LINEAR,
            internal_format: gl::RGBA32F,
            format: gl::RGBA,
        }
    }
}

// todo brice : implémenter drop pour la texture, histoire de libérer les buffers opengl (cf les buffers)
#[derive(Copy, Clone)]
pub struct Texture2D {
    id: u32,
    dimensions: (i32, i32),
    presets: TexturePresets,
}

impl Texture2D {
    pub fn new(dimensions: (i32, i32)) -> Self {
        let presets = TexturePresets::default();
        Self::new_from_presets(dimensions, presets, None)
    }

    pub fn new_from_presets(
        dimensions: (i32, i32),
        presets: TexturePresets,
        initial_value: Option<RgbaImage>,
    ) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        let tex = Self { id, dimensions, presets };
        tex.refresh(initial_value);
        tex
    }

    pub fn load_from(file_name: &str) -> Self {
        let buffer = image::open(format!("textures/{}", file_name))
            .unwrap()
            .flipv()
            .to_rgba8();

        let dimensions = buffer.dimensions();
        let dimensions: (i32, i32) = (dimensions.0 as i32, dimensions.1 as i32);
        let texture = Self::new_from_presets(dimensions, TexturePresets::default(), Some(buffer));

        texture
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_dimensions(&self) -> (i32, i32) {
        self.dimensions
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub fn resize(&mut self, dimensions: (i32, i32)) {
        self.dimensions = dimensions;
        self.refresh(None);
    }

    pub fn refresh(
        &self,
        initial_value: Option<RgbaImage>,
    ) {
        let mut id = 0;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            // set the texture wrapping/filtering options (on the currently bound texture object)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.presets.wrap_t as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.presets.wrap_s as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                self.presets.min_filter as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                self.presets.mag_filter as GLint,
            );

            match initial_value {
                Some(buffer) => {
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        self.presets.internal_format as GLint,
                        self.dimensions.0,
                        self.dimensions.1,
                        0,
                        self.presets.format,
                        gl::UNSIGNED_BYTE,
                        buffer.as_raw().as_ptr() as *const c_void,
                    );
                }
                None => {
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        self.presets.internal_format as GLint,
                        self.dimensions.0,
                        self.dimensions.1,
                        0,
                        self.presets.format,
                        gl::UNSIGNED_BYTE,
                        std::ptr::null(),
                    );
                }
            }
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0)
        }
    }

    pub fn get_presets(&self) -> TexturePresets {
        self.presets
    }
}

pub mod pipeline;

