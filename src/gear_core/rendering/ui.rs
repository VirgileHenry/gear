use foundry::World;
mod ui_layer;

use ui_layer::UILayer;
use crate::{DefaultOpenGlRenderer, Material, ShaderProgram, ShaderProgramRef, UI_DEFAULT_FRAG_SHADER, UI_DEFAULT_VERT_SHADER, UI_UNLIT_UV_FRAG_SHADER};


pub struct UIManager {
    layers: Vec<UILayer>,
    shader_program_ref: ShaderProgramRef,
}

impl UIManager {
    pub fn new(renderer: &mut DefaultOpenGlRenderer) -> Self {
        let shader_program = ShaderProgram::simple_program(UI_DEFAULT_FRAG_SHADER, UI_DEFAULT_VERT_SHADER)
            .expect("Error while generating UI shader");
        let shader_program_ref = ShaderProgramRef::new(&shader_program);
        renderer.register_shader_program(shader_program);
        UIManager {
            layers: vec![],
            shader_program_ref,
        }
    }

    // add a new layer on top of every layers
    pub fn add_new_layer(&mut self, world: &mut World) {
        self.layers.push(UILayer::new(world,self.shader_program_ref.clone()));
    }
}
