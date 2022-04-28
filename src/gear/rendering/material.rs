use super::shaders;
use std::ffi::CString;

pub struct Material {
    // shaders used to render this material
    pub shader_program: shaders::ShaderProgram,
    // params and uniforms for this material
}

impl Material {
    pub fn default() -> Material {
        // TERRIBLE : compile a shader program for each use
        // create shaders and set the program
        let vert_shader = shaders::Shader::from_vert_source(
            &CString::new(include_str!("shader_programs/default.vert.glsl")).unwrap()
        ).unwrap();
        
        let frag_shader = shaders::Shader::from_frag_source(
            &CString::new(include_str!("shader_programs/default.frag.glsl")).unwrap()
        ).unwrap();

        let shader_program_: shaders::ShaderProgram = shaders::ShaderProgram::from_shaders(&[vert_shader, frag_shader]).unwrap();
        // tell opengl we want to use the program
            
        return Material {
            shader_program: shader_program_,
        };
    }

    pub fn id(&self) -> u32 {
        return self.shader_program.id() as u32;
    }

}
