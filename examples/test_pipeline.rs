use gl::TEXTURE_2D;
use glfw::ffi::glfwGetTime;
use Gear::*;

fn main() {
    let window_size = (1200, 800);

    // create the engine with the window
    let mut engine = Engine::new() // creates the engine
        .with_gl_window(None, window_size); // with a window

    // create a renderer and give shaders to it
    let mut renderer = DefaultOpenGlRenderer::new();
    let program = ShaderProgram::simple_program(
        COPY_FRAG_SHADER,
        DEFAULT_VERT_SHADER
    ).expect("Unable to compile shaders !");
    // register the shader program in the renderer
    renderer.register_shader_program("copyShader", program);

    // create a mesh renderer from the shader program
    let mesh = Mesh::plane(Vector3::unit_x()*2., Vector3::unit_y()*2.);


    /* Pipeline set up */
    let mut pipeline = ShaderPipeline::new();

    pub static PERLIN_FRAG: &str = include_str!("test_pipeline/perlin.frag.glsl");
    let perlin_shader = ShaderProgram::simple_program(PERLIN_FRAG, PIPELINE_DEFAULT_VERT).unwrap();
    pipeline.add_node("perlin", (1000, 1000), perlin_shader);
    pipeline.set_float("perlin", "time", 1.0);

    pub static NORMAL_FRAG: &str = include_str!("test_pipeline/computeNormal.frag.glsl");
    let normal_shader = ShaderProgram::simple_program(NORMAL_FRAG, PIPELINE_DEFAULT_VERT).unwrap();
    pipeline.add_node("normal", (1000, 1000), normal_shader);
    pipeline.link_nodes("perlin", "heightMap", "normal");
    //normal_node.add_input_node("heightMap", perlin_node);

    unsafe {
        pipeline.compute("normal");
    }

    let mut material = Material::from_program("copyShader", Box::new(NoParamMaterialProperties{}));
    unsafe {
        material.attach_texture(pipeline.get_texture("normal"));
    }
    //let timer = TimingSystem{timer:0.0, perlin_node};
    //let system = System::new(Box::new(timer), UpdateFrequency::PerFrame);


    let mesh_renderer = MeshRenderer::new(&mesh, material);

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

    let _plane = create_entity!(&mut world.components; Transform::origin(), mesh_renderer);
    let mut camera_component = CameraComponent::new_perspective_camera(window_size, 80.0, aspect_ratio, 0.1, 100.0);
    camera_component.set_as_main(&mut world.components);
    let _camera = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(0.0, -0.2, 1.0)), camera_component);

    // start main loop
    //world.register_system(system, 10);

    engine.main_loop();

}


//struct TimingSystem {
//    timer: f32,
//}
//
//impl Updatable for TimingSystem {
//    fn update(&mut self, components: &mut ComponentTable, delta: f32, _user_data: &mut dyn std::any::Any) {
//        self.timer += delta;
//        self.node.set_float("time", self.timer);
//        unsafe {
//            self.node.compute();
//        }
//    }
//
//    fn as_any(&self) -> &dyn std::any::Any {
//        self
//    }
//
//    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//        self
//    }
//}