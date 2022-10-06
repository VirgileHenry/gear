extern crate gl;
extern crate cgmath;
use std::ffi::{CString, CStr};
use std::fs;


pub struct ShaderProgram {
    id: gl::types::GLuint,
}

// used as a ref to a shader program.
#[derive(Clone, Copy)]
pub struct ShaderProgramRef {
    id: u32,
}

impl ShaderProgramRef {
    pub fn new(from: &ShaderProgram) -> ShaderProgramRef {
        return ShaderProgramRef { id: from.id() }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl ShaderProgram {
    pub fn simple_program(frag_source: &str, vert_source: &str) -> Result<ShaderProgram, String> {
        // create a shader program
        let program_id = unsafe { gl::CreateProgram() };

        let frag_shader = match Shader::from_frag_string(frag_source) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        let vert_shader = match Shader::from_vert_string(vert_source) {
            Ok(shader) => shader,
            Err(error) => return Err(error),
        };
        unsafe { 
            gl::AttachShader(program_id, frag_shader.id());
            gl::AttachShader(program_id, vert_shader.id());
        }

        // link the prgram
        unsafe { gl::LinkProgram(program_id); }

        // error handling (same as shaders)
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }
        
            let error = create_whitespace_cstring_with_len(len as usize);
        
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
        
            return Err(error.to_string_lossy().into_owned());
        }

        // now the program is linked, we can unbind shaders
        unsafe { 
            gl::DetachShader(program_id, frag_shader.id());
            gl::DetachShader(program_id, vert_shader.id());
        }

        // return the program
        Ok(ShaderProgram { id: program_id })
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        // create a shader program
        let program_id = unsafe { gl::CreateProgram() };
        
        // bind given shaders to it
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        // link the prgram
        unsafe { gl::LinkProgram(program_id); }

        // error handling (same as shaders)
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }
        
            let error = create_whitespace_cstring_with_len(len as usize);
        
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
        
            return Err(error.to_string_lossy().into_owned());
        }

        // now the program is linked, we can unbind shaders
        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        // return the program
        Ok(ShaderProgram { id: program_id })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /// Set a matrix4 uniform.
    /// Will fail silently, so a same renderer can be adapted to different shaders without requirements.
    pub fn set_mat4(&self, name: &str, val: cgmath::Matrix4<f32>) {
        unsafe {
            let c_name = CString::new(name)
            .unwrap()
            .into_bytes_with_nul();
            let loc = gl::GetUniformLocation(self.id, c_name.as_ptr().cast());
            if loc != -1 {
                gl::UniformMatrix4fv(
                    loc,
                    1, 
                    gl::FALSE, 
                    &val[0][0] as *const f32,
                )
            }
        }
    }

    /// Set a float uniform.
    /// Will fail silently, so a same renderer can be adapted to different shaders without requirements.
    pub fn set_float(&self, name: &str, val: f32) {
        unsafe {
            let c_name = CString::new(name)
                .unwrap()
                .into_bytes_with_nul();
            let loc = gl::GetUniformLocation(self.id, c_name.as_ptr().cast());
            if loc != -1 {
                gl::Uniform1f(
                    loc, 
                    val,
                )
            }            
        }
    }

    /// Set a vector3 uniform.
    /// Will fail silently, so a same renderer can be adapted to different shaders without requirements.
    pub fn set_vec3(&self, name: &str, val: cgmath::Vector3<f32>) {
        unsafe {
            let c_name = CString::new(name)
            .unwrap()
            .into_bytes_with_nul();
            let loc = gl::GetUniformLocation(self.id, c_name.as_ptr().cast());
            if loc != -1 {
                gl::Uniform3fv(
                    loc, 
                    1,
                    &val[0] as *const f32
                )
            }
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}


pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &str, shader_type: gl::types::GLuint) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(shader_type) };

        let source = match fs::read_to_string(&source) {
            Ok(shader_code) => shader_code,
            Err(_error) => return Err(format!("Unable to read file at : {}", source)), 
        };

        unsafe {
            gl::ShaderSource(id,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap())
            );

            gl::CompileShader(id);
        }

        // get the compile error message
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        // check for shader compile errors
        if success == 0 {
            // compile error: get the error log message
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            // create a empty buffer for the error
            let error = create_whitespace_cstring_with_len(len as usize);
            // get the error from opengl
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            // return the error
            return Err(error.to_string_lossy().into_owned());
        }
        //returns the shader id
        Ok( Shader{id: id} )
    }

    pub fn from_vert_source(source: &str) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &str) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn from_string(string: &str, shader_type: gl::types::GLuint) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {        
            gl::ShaderSource(id,
                1,
                &(string.as_bytes().as_ptr().cast()),
                &(string.len().try_into().unwrap())
            );

            gl::CompileShader(id);
        }

        // get the compile error message
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        // check for shader compile errors
        if success == 0 {
            // compile error: get the error log message
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            // create a empty buffer for the error
            let error = create_whitespace_cstring_with_len(len as usize);
            // get the error from opengl
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            // return the error
            return Err(error.to_string_lossy().into_owned());
        }
        //returns the shader id
        Ok( Shader{id: id} )
    }

    pub fn from_vert_string(source: &str) -> Result<Shader, String> {
        Shader::from_string(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_string(source: &str) -> Result<Shader, String> {
        Shader::from_string(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            // clean the shader id
            gl::DeleteShader(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

