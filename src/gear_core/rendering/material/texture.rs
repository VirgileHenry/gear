use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Mutex;

use cgmath::Vector2;
use gl::types::{GLenum, GLint, GLuint};
use image::RgbaImage;

pub use pipeline::*;

static mut AVAILABLE_GEAR_ID: u32 = 0;
lazy_static! {

    static ref TEXTURE_REGISTER: Mutex<HashMap<u32, u32>> = { Mutex::new(HashMap::new()) };
    // HashMap<gear_id, opengl_id>
    // gear id is to make sure a texture is not replaced by an other when deleted
}

#[derive(Copy, Clone)]
struct TextureId {
    opengl_id: u32,
    gear_id: u32,
}

fn register_texture(opengl_id: u32) -> TextureId {
    unsafe {
        AVAILABLE_GEAR_ID += 1;
        TEXTURE_REGISTER.lock()
            .unwrap()
            .insert(AVAILABLE_GEAR_ID, opengl_id);
        TextureId {
            gear_id: AVAILABLE_GEAR_ID,
            opengl_id,
        }
    }
}

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

    pub fn ocean_final() -> Self {
        Self {
            wrap_s: gl::CLAMP_TO_EDGE,
            wrap_t: gl::CLAMP_TO_EDGE,
            min_filter: gl::LINEAR,
            mag_filter: gl::LINEAR,
            internal_format: gl::RGBA32F,
            format: gl::RGBA,
        }
    }

    pub fn ocean_default() -> Self {
        Self {
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            min_filter: gl::LINEAR,
            mag_filter: gl::LINEAR,
            internal_format: gl::RG32F,
            format: gl::RG
        }
    }

    pub fn ocean_init() -> Self {
        Self {
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            min_filter: gl::NEAREST,
            mag_filter: gl::NEAREST,
            internal_format: gl::RG32F,
            format: gl::RG,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Texture2D {
    id: TextureId,
    dimensions: (i32, i32),
    presets: TexturePresets,
}

impl Texture2D {
    pub fn new(dimensions: (i32, i32)) -> Self {
        Self::new_from_presets(dimensions, TexturePresets::default(), None)
    }

    pub fn new_from_presets(
        dimensions: (i32, i32),
        presets: TexturePresets,
        initial_value: Option<RgbaImage>,
    ) -> Self {
        let mut opengl_id = 0;
        unsafe {
            gl::GenTextures(1, &mut opengl_id);
        }

        let tex = Self {
            id: register_texture(opengl_id),
            dimensions,
            presets
        };
        tex.refresh(initial_value);
        tex
    }

    /// This id shouldn't be stored because the getter checks if the texture still exists
    pub fn get_opengl_id(&self) -> Result<u32, String> {
        unsafe {
            match TEXTURE_REGISTER.lock().unwrap().get(&self.id.gear_id) {
                Some(registered_id) => {
                    if *registered_id == self.id.opengl_id {
                        Ok(self.id.opengl_id)
                    } else {
                        Err(String::from("This texture has been deleted."))
                    }
                }
                None => {
                    Err(String::from("This texture has been deleted."))
                }
            }
        }
    }

    pub fn unwrap_id(&self) -> u32 {
        self.get_opengl_id().unwrap()
    }

    pub fn load_from_vec(data:  Vec<Vector2<f32>>, dimensions: (i32, i32), presets: TexturePresets) -> Self {
        let mut opengl_id = 0;
        unsafe {
            gl::GenTextures(1, &mut opengl_id);
        }
        let tex =  Self {
            id: register_texture(opengl_id),
            dimensions,
            presets
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, opengl_id);
            // set the texture wrapping/filtering options (on the currently bound texture object)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, tex.presets.wrap_t as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, tex.presets.wrap_s as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                tex.presets.min_filter as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                tex.presets.mag_filter as GLint,
            );
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                tex.presets.internal_format as GLint,
                tex.dimensions.0,
                tex.dimensions.1,
                0,
                tex.presets.format,
                gl::FLOAT,
                data.as_ptr() as *const c_void,
            );
        }
        tex
    }

    pub fn CS_new_storage2D_text (
        dimensions: (i32, i32),
        presets: TexturePresets
    ) -> Self {
        let mut opengl_id = 0;
        unsafe {
            gl::GenTextures(1, &mut opengl_id);
        }
        let tex = Self { id: register_texture(opengl_id), dimensions, presets};
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, tex.get_opengl_id().expect("error: texture id broken"));
            // set the texture wrapping/filtering options (on the currently bound texture object)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, tex.presets.wrap_t as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, tex.presets.wrap_s as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                tex.presets.min_filter as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                tex.presets.mag_filter as GLint,
            );

            gl::TexStorage2D(gl::TEXTURE_2D, 1, tex.presets.internal_format, dimensions.0, dimensions.1);

            gl::BindTexture(gl::TEXTURE_2D, 0);

        }
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

    pub fn get_dimensions(&self) -> (i32, i32) {
        self.dimensions
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.get_opengl_id().unwrap());
    }

    pub unsafe fn unbind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    pub fn resize(&mut self, dimensions: (i32, i32)) {
        self.dimensions = dimensions;
        self.refresh(None);
    }

    pub fn refresh(
        &self,
        initial_value: Option<RgbaImage>,
    ) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.get_opengl_id().unwrap());
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

            gl::BindTexture(gl::TEXTURE_2D, 0)
        }
    }

    pub fn get_presets(&self) -> TexturePresets {
        self.presets
    }

    pub unsafe fn generate_mipmap(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.get_opengl_id().unwrap());
        gl::GenerateMipmap(gl::TEXTURE_2D); // TODO brice: faut-il le faire
        gl::BindTexture(gl::TEXTURE_2D, 0);

    }

    pub unsafe fn copy(&self, texture_to_copy: &Texture2D) {
        gl::CopyImageSubData(
            texture_to_copy.get_opengl_id().unwrap(),
            gl::TEXTURE_2D,
            0,
            0,
            0,
            0,
            self.get_opengl_id().unwrap(),
            gl::TEXTURE_2D,
            0,
            0,
            0,
            0,
            self.dimensions.0,
            self.dimensions.1,
            1,
        );
    }

    pub fn delete(texture: Texture2D) -> Result<(), String> {
        match texture.get_opengl_id() {
            Ok(id) => unsafe {
                gl::DeleteTextures(1, &id);
                TEXTURE_REGISTER.lock().unwrap().remove(&id);
                Ok(())
            }
            Err(msg) => Err(msg)
        }
    }
}

pub mod pipeline;
