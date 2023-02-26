use gl::TEXTURE_2D;
use glfw::ffi::glfwGetTime;

use gear::*;

fn main() {
    let window_size = (1200, 800);

    // create the engine with the window
    let mut engine = Engine::new() // creates the engine
        .with_gl_window(None, window_size); // with a window

    // create a renderer and give shaders to it
    let mut renderer = DefaultOpenGlRenderer::new(window_size);
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

    pub static COMPUTE_PERLIN: &str = include_str!("test_pipeline_with_compute/compute_perlin.comp.glsl");
    static PERLIN_NODE: &str = "perlin";
    let mut compute_perlin = ComputeShader::new(COMPUTE_PERLIN, (1000, 1000, 1));
    compute_perlin.add_read_write_texture("perlin_texture", Texture2D::new_from_presets((1000, 1000), TexturePresets::pipeline_default(), None));
    compute_perlin.add_write_texture("uv_texture", Texture2D::new_from_presets((1000, 1000), TexturePresets::pipeline_default(), None));
    pipeline.add_compute_node(PERLIN_NODE, compute_perlin);
    pipeline.set_float(PERLIN_NODE, "time", 1.0);

    pub static COMPUTE_COPY: &str = include_str!("test_pipeline_with_compute/copy.comp.glsl");
    static COPY_NODE: &str = "copy";
    let mut compute_copy = ComputeShader::new(COMPUTE_COPY, (1000, 1000, 1));
    compute_copy.add_write_texture("copy_texture", Texture2D::new_from_presets((1000, 1000), TexturePresets::pipeline_default(), None));
    pipeline.add_compute_node(COPY_NODE, compute_copy);

    pipeline.link_compute_to_node(PERLIN_NODE, "perlin_texture", "copied_texture", COPY_NODE);

    unsafe {
        pipeline.compute(COPY_NODE);
    }

    let mut material = Material::from_program("copyShader");
    unsafe {
        material.attach_texture(pipeline.get_texture(COPY_NODE, &Some(String::from("copy_texture"))));
    }
    let mesh_renderer = MeshRenderer::new(&mesh, material);

    //let timer = TimingSystem{timer:0.0, pipeline};
    //let system = System::new(Box::new(timer), UpdateFrequency::PerFrame);


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
    let _camera = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(0.0, 0.0, 1.0)), camera_component);

    // start main loop
    //world.register_system(system, 10);

    engine.main_loop();
}


struct TimingSystem {
    timer: f32,
    pipeline: ShaderPipeline,
}

impl Updatable for TimingSystem {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, _user_data: &mut dyn std::any::Any) {
        self.timer += delta*0.00;
        self.pipeline.set_float("perlin", "time", self.timer);
        unsafe {
            self.pipeline.compute("normal");
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}