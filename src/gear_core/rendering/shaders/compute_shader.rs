extern crate cgmath;
extern crate gl;


use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs;

use gl::types::{GLsizei, GLuint};

use crate::{ShaderProgram, Texture2D};

pub struct ComputeShader {
    program: ShaderProgram,
    output_texture: Texture2D,
    input_textures: HashMap<String, Texture2D>,

    dispatch_dimensions: (i32, i32, i32),
}

impl ComputeShader {
    pub fn new(compute_src: &str, dimensions: (i32, i32), dispatch_dimensions: (i32, i32, i32)) -> Self {
        Self {
            program: ShaderProgram::compute_program(compute_src).expect("Could not compile compute shader"),
            output_texture: Texture2D::new(dimensions),
            input_textures: Default::default(),

            dispatch_dimensions,
        }
    }

    pub fn add_input_texture(&mut self, name: &str, texture: Texture2D) {
        self.input_textures.insert(name.to_string(), texture);
    }

    /// /!\ The compute shader must be set used before
    pub fn begin_computation(&self, dispatch_dimensions: (u32, u32, u32)) {
        unsafe { gl::DispatchCompute(dispatch_dimensions.0 as GLuint, dispatch_dimensions.1 as GLuint, dispatch_dimensions.2 as GLuint); }
    }

    pub fn retrieve_result(&self) {
        unsafe { gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT) }
    }
}
