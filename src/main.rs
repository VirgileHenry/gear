mod gear_core;

use foundry::{create_entity, ecs::system::Updatable, iterate_over_component_mut};
use gear_core::{engine::Engine, rendering::{mesh::{MeshRenderer, MeshType}, material::{Material, MonochromeMaterialProperties}, shaders::ShaderProgram, renderer::DefaultOpenGlRenderer}};

use crate::gear_core::{rendering::{camera::CameraComponent, mesh::Mesh, opengl::color::Color, lighting::light::MainLight}, geometry::transform::Transform};

fn main() {
    // create the engine with the window
    let mut engine = Engine::new() // creates the engine
        .with_gl_window(None, None); // with a window

    // create a renderer and give shaders to it
    let mut renderer = DefaultOpenGlRenderer::new();
    let program = ShaderProgram::simple_program(
        crate::gear_core::rendering::shaders_files::shaders::MONOCHROME_LIT_FRAG_SHADER, 
        crate::gear_core::rendering::shaders_files::shaders::DEFAULT_VERT_SHADER
        ).expect("Unable to compile shaders !");
    
    // create a mesh renderer from the shader program
    let mesh = MeshType::Owned(Mesh::cube(1.0));
    let material = Material::from_program(&program, Box::new(MonochromeMaterialProperties{color: Color::from_rgb(0.2, 0.4, 0.5)}));
    let mesh_renderer = MeshRenderer::new(mesh, material);

    // register the shader program in the renderer
    renderer.register_shader_program(program);
    // assign the renderer to the window
    let mut aspect_ratio = 1.0;
    match engine.get_gl_window_mut() {
        Some(window) => {
            window.set_new_renderer(Box::new(renderer));
            aspect_ratio = window.aspect_ratio();
        },
        None => {},
    }

    // create cube and camera entity
    let world = engine.get_world();

    let rotater = RotatingSystem{timer:0.0};
    let system = foundry::ecs::system::System::new(Box::new(rotater), foundry::ecs::system::UpdateFrequency::PerFrame);

    let cube = create_entity!(world.components; Transform::origin(), mesh_renderer);
    let mut camera_component = CameraComponent::new_perspective_camera(80.0, aspect_ratio, 0.1, 100.0);
    camera_component.set_as_main(&mut world.components);
    let camera = create_entity!(world.components; Transform::origin().translated(0.0, 1.5, -5.0).euler(0.0, 3.1415, 0.0), camera_component);
    let sun = create_entity!(world.components; Transform::origin().translated(-4.0, 4.0, -6.0), MainLight::new(Color::from_rgb(1.0, 0.8, 0.7), Color::from_rgb(0.2, 0.2, 0.2)));

    world.register_system(system, 10);
    // start main loop
    engine.main_loop();

}

struct RotatingSystem {
    timer: f32,
}

impl Updatable for RotatingSystem {
    fn update(&mut self, components: &mut foundry::ecs::component_table::ComponentTable, delta: f32, _user_data: &mut dyn std::any::Any) {
        self.timer += delta;
        for (transform, _other) in iterate_over_component_mut!(components; Transform, MeshRenderer) {
            transform.rotate(cgmath::Vector3::new(0.0, 1.0, 0.0), 1.0 * delta);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}




