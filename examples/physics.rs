use Gear::*;
use cgmath::Vector3;

fn main() {
    // create the engine with the window
    let mut engine = Engine::new() // creates the engine
        .with_gl_window(None, None); // with a window

    // create a renderer and give shaders to it
    let mut renderer = DefaultOpenGlRenderer::new();
    let program = ShaderProgram::simple_program(
        MONOCHROME_LIT_FRAG_SHADER,
        DEFAULT_VERT_SHADER
    ).expect("Unable to compile shaders !");

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

    // let's do everything here !



    let mut camera_component = CameraComponent::new_perspective_camera(80.0, aspect_ratio, 0.1, 100.0);
    camera_component.set_as_main(&mut world.components);
    let _camera = create_entity!(&mut world.components; Transform::origin().translated(0.0, 1.5, -5.0).euler(0.0, 3.1415, 0.0), camera_component);
    let sun = create_entity!(&mut world.components; Transform::origin().translated(-4.0, -4.0, -6.0), MainLight::new(Color::from_rgb(1.0, 0.8, 0.7), Color::from_rgb(0.2, 0.2, 0.2)));
    
    
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

struct SphereColliderBody {
    radius: f32,
    velocity: Vector3<f32>,
}

struct Collision<'a> {
    obj1: &'a mut SphereColliderBody,
    obj2: &'a mut SphereColliderBody,
    t: f32,
}

fn create_sphere(components: &mut ComponentTable, program: &ShaderProgram, position: Vector3<f32>, radius: f32) -> EntityRef {
    let mesh = MeshType::Owned(Mesh::sphere(1.0, 40));
    let material = Material::from_program(&program, Box::new(MonochromeMaterialProperties{color: Color::from_rgb(0.4, 0.8, 1.0)}));
    let mesh_renderer = MeshRenderer::new(mesh, material);
    let transform = Transform::origin().translated(position.x, position.y, position.z);
    let sphere_collider = SphereColliderBody{radius: radius, velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 }}; // no initial velocity
    create_entity!(components; mesh_renderer, transform, sphere_collider)
}

struct SpherePhysics {
    gravity: f32,
}

impl Updatable for SpherePhysics {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, user_data: &mut dyn std::any::Any) {
        // loop through every sphere
        // n² for now but let's try it out fiiiirst
        let mut allCollisions: Vec<Collision> = Vec::new();

        for (col1, tf1) in iterate_over_component_mut!(components; SphereColliderBody, Transform) {
            for (col2, tf2) in iterate_over_component_mut!(components; SphereColliderBody, Transform) {
                // this is unsafe at this point ! we should check col1 and col2 are different
                if std::ptr::eq(col1, col2) {continue;}
                // allCollisions.push(Collision { obj1: col1, obj2: col2, t: 0.0 })
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}