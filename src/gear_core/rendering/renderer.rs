use std::{collections::HashMap};
use cgmath::SquareMatrix;
use foundry::*;
use crate::gear_core::{
    rendering::{
        geometry::mesh_renderer::MeshRenderer,
        camera::CameraComponent,
        lighting::light::MainLight,
        shaders::{Shader, ShaderProgram, ShaderProgramRef},
    },
    geometry::transform::Transform,
};


pub trait Renderer {
    fn render(&self, components: &mut ComponentTable);
}

pub struct DefaultOpenGlRenderer {
    shader_programs: HashMap<u32, ShaderProgram>,
    missing_shader_program: ShaderProgram,
}

impl DefaultOpenGlRenderer {
    pub fn new() -> DefaultOpenGlRenderer {
        use super::shaders::shaders_files::{MISSING_FRAG_SHADER, DEFAULT_VERT_SHADER};
        DefaultOpenGlRenderer {
            shader_programs: HashMap::new(),
            missing_shader_program: ShaderProgram::simple_program(MISSING_FRAG_SHADER, DEFAULT_VERT_SHADER)
                .expect("[GEAR ENGINE] -> [RENDERER] -> Unable to compile default shaders : "),
        }
    }

    pub fn register_shader_program(&mut self, program: ShaderProgram) {
        self.shader_programs.insert(program.id() as u32, program);
    }
}

impl Renderer for DefaultOpenGlRenderer {
    fn render(&self, components: &mut ComponentTable) {
        // find main camera

        for (camera, cam_transform) in iterate_over_component!(&components; CameraComponent, Transform) {
            if !camera.is_main() { continue; } // check we have the main camera
            // sort elements to render by shader to minimise the change of shader program
            let mut rendering_map: HashMap<u32, Vec<(&Transform, &MeshRenderer)>> = HashMap::new();

            for (transform, mesh) in iterate_over_component!(&components; Transform, MeshRenderer) {
                match rendering_map.get_mut(&mesh.material.program_ref.id()) {
                    Some(vec) => vec.push((transform, mesh)),
                    None => {rendering_map.insert(mesh.material.program_ref.id(), vec![(transform, mesh)]);},
                }
            }

            // Set gl defaults params
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Enable(gl::CULL_FACE);
                gl::CullFace(gl::FRONT);
                gl::Enable(gl::DEPTH_TEST);
                gl::DepthFunc(gl::LESS);
            }

            for (id, vec) in rendering_map.into_iter() {
                // switch to render program
                let current_program = match self.shader_programs.get(&id) {
                    Some(shader_program) => shader_program,
                    None => &self.missing_shader_program,
                };
                current_program.set_used();
                // set camera uniform
                current_program.set_mat4("viewMat", cam_transform.world_pos().invert().unwrap());
                current_program.set_mat4("projectionMat", camera.get_perspective_mat());
                current_program.set_vec3("camPos", cam_transform.position());
                // set main light scene
                for (light, light_tf) in iterate_over_component!(components; MainLight, Transform) {
                    current_program.set_vec3("mainLightPos", light_tf.position());
                    current_program.set_vec3("mainLightColor", light.color_as_vec());
                    current_program.set_vec3("ambientColor", light.ambient_as_vec());
                    break; // only first main light taken into account, the others would override the first one so let's avoid useless code
                }

                for (transform, mesh_renderer) in vec.into_iter() {
                    // todo !
                    // set model uniform
                    current_program.set_mat4("modelMat", transform.world_pos());
                    unsafe {
                        mesh_renderer.draw(current_program);
                    }
                }
            }

            break; // render only once in case there are multiple main camera component (and avoid useless shooting)
        }

        // todo render UI ! 

        
    }
}

