use gear::*;

fn main() {
    let window_size = (1200, 800);

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
    let mesh = Mesh::cube(1.0);
    let mesh2 = Mesh::cube(0.5);
    let mesh3 = Mesh::cube(2.0);
    let material = Material::from_program("defaultShader")
        .with_property(MonochromeMaterialProperties{color: Color::from_rgb(0.4, 0.8, 1.0)});
    let material2 = Material::from_program("defaultShader")
        .with_property(MonochromeMaterialProperties{color: Color::from_rgb(0.8, 1.0, 0.4)});
    let material3 = Material::from_program("defaultShader")
        .with_property(MonochromeMaterialProperties{color: Color::from_rgb(1.0, 0.4, 0.8)});
    let mesh_renderer = MeshRenderer::new(&mesh3, material);
    let mesh_renderer2 = MeshRenderer::new(&mesh, material2);
    let mesh_renderer3 = MeshRenderer::new(&mesh2, material3);


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

    let cube_tf1 = Transform::origin();
    let mut cube_tf2 = Transform::origin().translated(Vector3::new(2.0, 0.0, 0.0));
    let mut cube_tf3 = Transform::origin().translated(Vector3::new(1.0, 0.0, 0.0));
    cube_tf2.set_parent(Some(&cube_tf1));
    cube_tf3.set_parent(Some(&cube_tf2));

    let _sphere = create_entity!(&mut world.components; cube_tf1, mesh_renderer);
    let _cube = create_entity!(&mut world.components; cube_tf2, mesh_renderer2);
    let _cube2 = create_entity!(&mut world.components; cube_tf3, mesh_renderer3);
    let mut camera_component = CameraComponent::new_perspective_camera(None, 80.0, 0.1, 100.0);
    camera_component.set_as_main(&mut world.components);
    let _camera = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(0.0, 1.5, 5.0)), camera_component);
    let _sun = create_entity!(&mut world.components; Transform::origin().rotated(Euler::new(Rad(-1.4), Rad(0.75), Rad(0.0))), MainLight::new(Color::from_rgb(1.0, 0.8, 0.7), Color::from_rgb(0.2, 0.2, 0.2)));
    
    // world.set_entity_active(sun, false);

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




