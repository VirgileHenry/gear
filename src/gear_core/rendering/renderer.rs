use crate::gear_core::{
    geometry::transform::Transform,
    rendering::{
        camera::CameraComponent,
        geometry::mesh_renderer::MeshRenderer,
        lighting::light::MainLight,
        shaders::{Shader, ShaderProgram, ShaderProgramRef},
    },
};
use crate::{COPY_FRAG_SHADER, Material, Mesh, NoParamMaterialProperties, UI_DEFAULT_FRAG_SHADER, UI_DEFAULT_VERT_SHADER, UI_UNLIT_UV_FRAG_SHADER};
use cgmath::{SquareMatrix, Vector3};
use foundry::*;
use gl::types::*;
use std::collections::HashMap;

pub trait Renderer {
    fn render(&self, components: &mut ComponentTable);
}

pub struct DefaultOpenGlRenderer {
    shader_programs: HashMap<String, ShaderProgram>,
    missing_shader_program: ShaderProgram,

    render_quad: MeshRenderer,
    copy_shader: ShaderProgram,
}

impl DefaultOpenGlRenderer {
    pub fn new() -> DefaultOpenGlRenderer {

        let copy_shader = ShaderProgram::simple_program(COPY_FRAG_SHADER, UI_DEFAULT_VERT_SHADER)
            .expect("Error while generating UI shader");
        let mesh = Mesh::plane(Vector3::unit_x()*2., Vector3::unit_y()*2.);
        let material = Material::from_program("copy_shader", Box::new(NoParamMaterialProperties{}));
        let mesh_renderer = MeshRenderer::new(mesh, material);

        use super::shaders::shaders_files::{MISSING_FRAG_SHADER, DEFAULT_VERT_SHADER};
        DefaultOpenGlRenderer {
            shader_programs: HashMap::new(),
            missing_shader_program: ShaderProgram::simple_program(MISSING_FRAG_SHADER, DEFAULT_VERT_SHADER)
                .expect("[GEAR ENGINE] -> [RENDERER] -> Unable to compile default shaders : "),
            render_quad: mesh_renderer,
            copy_shader,
        }
    }

    pub fn register_shader_program(&mut self, name: &str, program: ShaderProgram) {
        self.shader_programs.insert(name.to_string(), program);
    }
}

impl Renderer for DefaultOpenGlRenderer {

    fn render(&self, components: &mut ComponentTable) {

        self.render_camera_buffers(components);
        for (camera, cam_transform) in iterate_over_component!(&components; CameraComponent, Transform) {
            if camera.is_main() {
                unsafe {
                    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    gl::ActiveTexture(gl::TEXTURE0);
                    camera.get_color_attachment().bind();
                    self.copy_shader.set_used();
                    self.copy_shader.set_mat4("modelMat", Transform::origin().world_pos());
                    self.copy_shader.set_int("tex", 0);
                    self.render_quad.draw(&self.copy_shader);
                }
            }
        }

    }

}

impl DefaultOpenGlRenderer {
    fn render_camera_buffers(&self, components: &mut ComponentTable) {
        // found main camera

        for (camera, cam_transform) in iterate_over_component!(&components; CameraComponent, Transform) {
            // todo: render only if needed
            // todo: render main at the end ?
            // todo: sort once ?
            // sort elements to render by shader to minimise the change of shader program
            let mut rendering_map: HashMap<&str, Vec<(&Transform, &MeshRenderer)>> = HashMap::new();

            for (transform, mesh) in iterate_over_component!(&components; Transform, MeshRenderer) {
                match rendering_map.get_mut(&mesh.material.get_program_name()) {
                    Some(vec) => vec.push((transform, mesh)),
                    None => {rendering_map.insert(mesh.material.get_program_name(), vec![(transform, mesh)]);},
                }
            }

            // Set front cull faces on
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Enable(gl::CULL_FACE);
                gl::CullFace(gl::FRONT);
                gl::Enable(gl::DEPTH_TEST);
                gl::DepthFunc(gl::LESS);
                gl::Viewport(0, 0, camera.get_dimensions().0 as GLsizei, camera.get_dimensions().1 as GLsizei);
                camera.bind();
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            let view_mat = cam_transform.world_pos().invert();

            for (id, vec) in rendering_map.into_iter() {
                // switch to render program
                let current_program = match self.shader_programs.get(id) {
                    Some(shader_program) => shader_program,
                    None => &self.missing_shader_program,
                };
                current_program.set_used();
                // set camera uniform
                current_program.set_mat4("viewMat", view_mat.unwrap());
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
            unsafe {
                camera.unbind();
            }
            break; // render only once in case there are multiple main camera component (and avoid useless shooting)
        }

        // todo render UI !


    }
}

