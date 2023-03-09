extern crate cgmath;
extern crate gl;


use std::collections::HashMap;

use gl::types::GLuint;

use crate::{ShaderProgram, Texture2D};

pub struct ComputeShader {
    program: ShaderProgram,

    read_write_textures: HashMap<String, Texture2D>,
    read_textures: HashMap<String, Texture2D>,
    write_textures: HashMap<String, Texture2D>,

    dispatch_dimensions: (i32, i32, i32),
}

impl ComputeShader {
    pub fn new(compute_src: &str, dispatch_dimensions: (i32, i32, i32)) -> Self {

        Self {
            program: ShaderProgram::compute_program(compute_src).expect("Could not compile compute shader"),

            read_write_textures: Default::default(),
            write_textures: Default::default(),
            read_textures: Default::default(),

            dispatch_dimensions,
        }
    }

    pub fn set_dispatch_dimensions(&mut self, dimensions: (i32, i32, i32)) {
        self.dispatch_dimensions = dimensions;
    }

    pub fn add_read_texture(&mut self, name: &str, texture: Texture2D) {
        self.read_textures.insert(name.to_string(), texture);
    }
    pub fn add_write_texture(&mut self, name: &str, texture: Texture2D) {
        self.write_textures.insert(name.to_string(), texture);
    }
    pub fn add_read_write_texture(&mut self, name: &str, texture: Texture2D) {
        self.read_write_textures.insert(name.to_string(), texture);
    }

    pub fn get_texture(&self, name: &str) -> Texture2D {
        if let Some(tex) = self.write_textures.get(name) {
            return tex.clone();
        }
        if let Some(tex) = self.read_write_textures.get(name) {
            return tex.clone();
        }
        if let Some(tex) = self.read_textures.get(name) {
            return tex.clone();
        }
        panic!("Texture {name} not found");
    }

    pub fn set_used(&self) -> &ShaderProgram {
        self.program.set_used();

        let mut location = 0;

        for (name, texture) in &self.read_textures {
            self.program.set_int(name, location);
            self.program.set_image2d_read(texture, location as u32);
            location += 1;
        }
        for (name, texture) in &self.write_textures {
            self.program.set_int(name, location);
            self.program.set_image2d_write(texture, location as u32);
            location += 1;
        }
        for (name, texture) in &self.read_write_textures {
            self.program.set_int(name, location);
            self.program.set_image2d_read_write(texture, location as u32);
            location += 1;
        }

        &self.program
    }

    pub fn get_texture_count(&self) -> u32 {
        (self.read_textures.len() + self.write_textures.len() + self.read_write_textures.len()) as u32
    }

    /// /!\ The compute shader must be set used before
    pub fn begin_computation_with_dimensions(&self, dispatch_dimensions: (u32, u32, u32)) {
        unsafe { gl::DispatchCompute(dispatch_dimensions.0 as GLuint, dispatch_dimensions.1 as GLuint, dispatch_dimensions.2 as GLuint); }
    }

    /// /!\ The compute shader must be set used before
    pub fn begin_computation(&self) {
        unsafe { gl::DispatchCompute(self.dispatch_dimensions.0 as GLuint, self.dispatch_dimensions.1 as GLuint, self.dispatch_dimensions.2 as GLuint); }
    }

    pub fn wait_for_result(&self) {
        unsafe { gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT) }
    }
}

impl Clone for ComputeShader {
    /// Clone the compute shader program and dimensions
    /// ! not the textures
    fn clone(&self) -> Self {
        Self {
            program: self.program.clone(),

            read_write_textures: Default::default(),
            write_textures: Default::default(),
            read_textures: Default::default(),

            dispatch_dimensions: self.dispatch_dimensions,
        }
    }
}
