extern crate gl;
// import namespace to avoid repeating `std::ffi` everywhere
use std::ffi::{CString, CStr};



pub struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram {
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
    pub fn from_source(source: &CStr, shader_type: gl::types::GLuint) -> Result<Shader, String> {
        let id_ = match shader_from_source(source, shader_type) {
            Ok(id) => id,
            Err(error) => return Err(error.into()),
        };
        Ok(Shader { id: id_, })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
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

fn shader_from_source(source: &CStr, shader_type: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    // compiles and link a shader from the shader name
    // create a new shader, openGL returns it's id
    let id = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        // set the shader source
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        // compile it
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
    Ok(id)
}


fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}