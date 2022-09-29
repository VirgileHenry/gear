use super::shaders::{ShaderProgramRef, ShaderProgram};

#[derive(Clone, Copy)]
pub struct Material {
    /// shader program to use
    pub program_ref: ShaderProgramRef,
    // needs params depending on the program. Generics ?
}

impl Material {
    pub fn from_program(program: &ShaderProgram) -> Material {
        Material { program_ref: ShaderProgramRef::new(program) }
    }

    pub fn from_ref(program_ref: ShaderProgramRef) -> Material {
        Material { program_ref: program_ref }
    }
}