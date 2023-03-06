use gear::*;

fn main() {
    let window_size = (1024, 512);

    // create the engine with the window
    let mut engine = Engine::new() // creates the engine
        .with_gl_window(None, window_size); // with a window

    // create a renderer and give shaders to it
    let mut renderer = DefaultOpenGlRenderer::new(window_size);
    let program = ShaderProgram::simple_program(
        MONOCHROME_LIT_FRAG_SHADER,
        DEFAULT_VERT_SHADER
    ).expect("Unable to compile shaders !");
    // register the shader program in the renderer
    renderer.register_shader_program("defaultShader", program);

    // create a mesh renderer from the shader program
    let mesh = Mesh::sphere(1.0, 40);
    let mesh2 = Mesh::cube(2.0);
    let material = Material::from_program("defaultShader")
        .with_property(MonochromeMaterialProperties{color: Color::from_rgb(0.4, 0.8, 1.0)});
    let material2 = Material::from_program("defaultShader")
        .with_property(MonochromeMaterialProperties{color: Color::from_rgb(0.4, 0.8, 1.0)});
    let mesh_renderer = MeshRenderer::new(&mesh, material);
    let mesh_renderer2 = MeshRenderer::new(&mesh2, material2);


    // assign the renderer to the window
    match engine.get_gl_window_mut() {
        Some(window) => {
            window.set_new_renderer(Box::new(renderer));
        },
        None => {},
    }

    // create cube and camera entity
    let world = engine.get_world();

    let rotater = RotatingSystem{timer:0.0};
    let system = System::new(Box::new(rotater), UpdateFrequency::PerFrame);

    let _sphere = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(0.0, 1.8, 0.0)), mesh_renderer);
    let cube = create_entity!(&mut world.components; Transform::origin(), mesh_renderer2);
    let mut camera_component = CameraComponent::new_perspective_camera(None, 80.0, 0.1, 100.0);
    camera_component.set_as_main(&mut world.components);
    let _camera = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(0.0, 1.5, 5.0)), camera_component);
    let _sun = create_entity!(&mut world.components; Transform::origin().rotated(Euler::new(Rad(-1.4), Rad(0.75), Rad(0.0))), MainLight::new(Color::from_rgb(8.0, 5.8, 4.7), Color::from_rgb(0.2, 0.2, 0.2)));
    
    // world.set_entity_active(sun, false);
    let (cube_tf, cube_mr) = get_components!(&world.components; cube; mut Transform, MeshRenderer);

    world.register_system(system, 10);
    // start main loop
    engine.main_loop();

}

struct RotatingSystem {
    timer: f32,
}

impl Updatable for RotatingSystem {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, _user_data: &mut dyn std::any::Any) {
        self.timer += delta;
        for (transform, _other) in iterate_over_component_mut!(components; Transform, MeshRenderer) {
            transform.rotate_around(cgmath::Vector3::new(0.0, 1.0, 0.0), Rad(1.0 * delta));
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}




