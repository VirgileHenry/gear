use crate::{Material, ShaderProgram};


/// Component to render ui. 
pub struct UIRenderer {
    material: Material,
}

impl UIRenderer {
    pub fn new(material: Material) -> UIRenderer {
        UIRenderer { material }
    }

    pub fn material_name(&self) -> &str {
        self.material.get_program_name()
    }

    pub fn set_mat_to_shader(&self, program: &ShaderProgram) {
        self.material.set_properties_to_shader(program);
    }
}