use cgmath::num_traits::clamp;
use std::ffi::c_void;

use gl::types::{GLenum, GLint};
use image::RgbaImage;

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
            internal_format: gl::RGBA,
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
}

pub struct Texture2D {
    id: u32,
    dimensions: (i32, i32),
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
            gl::BindTexture(gl::TEXTURE_2D, id);
            // set the texture wrapping/filtering options (on the currently bound texture object)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, presets.wrap_t as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, presets.wrap_s as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                presets.min_filter as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                presets.mag_filter as GLint,
            );

            match initial_value {
                Some(buffer) => {
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        presets.internal_format as GLint,
                        dimensions.0,
                        dimensions.1,
                        0,
                        presets.format,
                        gl::UNSIGNED_BYTE,
                        buffer.as_raw().as_ptr() as *const c_void,
                    );
                }
                None => {
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        presets.internal_format as GLint,
                        dimensions.0,
                        dimensions.1,
                        0,
                        presets.format,
                        gl::UNSIGNED_BYTE,
                        std::ptr::null(),
                    );
                }
            }
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0)
        }
        Self { id, dimensions }
    }

    pub fn load_from(file_name: &str) -> Self {
        let buffer = image::open(format!("textures/{}", file_name))
            .unwrap()
            .flipv()
            .to_rgba8();

        let dimensions = buffer.dimensions();
        let dimensions: (i32, i32) = (dimensions.0 as i32, dimensions.1 as i32);
        println!("dimensions = {dimensions:?}");
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
}
