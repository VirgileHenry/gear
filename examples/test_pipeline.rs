use gl::TEXTURE_2D;
use glfw::ffi::glfwGetTime;

use Gear::*;

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

    pub static PERLIN_FRAG: &str = include_str!("test_pipeline/perlin.frag.glsl");
    let perlin_shader = ShaderProgram::simple_program(PERLIN_FRAG, PIPELINE_DEFAULT_VERT).unwrap();
    pipeline.add_render_node("perlin", (1000, 1000), perlin_shader);
    pipeline.set_float("perlin", "time", 1.0);

    pub static ISLAND_MASK_FRAG: &str = include_str!("test_pipeline/island_mask.frag.glsl");
    let island_mask_shader = ShaderProgram::simple_program(ISLAND_MASK_FRAG, PIPELINE_DEFAULT_VERT).unwrap();
    pipeline.add_render_node("island_mask", (1000, 1000), island_mask_shader);
    pipeline.set_float("island_mask", "global_falloff", 0.8);
    pipeline.set_float("island_mask", "falloff_speed", 16.0);
    pipeline.set_vec3("island_mask", "islands", Vector3::<f32>::new(0.5, 0.5, 0.2));

    pub static MULTIPLIER_FRAG: &str = include_str!("test_pipeline/multiplier.frag.glsl");
    let multiplier_shader = ShaderProgram::simple_program(MULTIPLIER_FRAG, PIPELINE_DEFAULT_VERT).unwrap();
    pipeline.add_render_node("multiplier", (1000, 1000), multiplier_shader);
    pipeline.link_render_to_node("perlin", "height_map", "multiplier");
    pipeline.link_render_to_node("island_mask", "mask_tex", "multiplier");
    pipeline.set_float("multiplier", "a", 0.45);
    pipeline.set_float("multiplier", "b", 0.505);
    pipeline.set_int("multiplier", "shape", 8);

    pub static NORMAL_FRAG: &str = include_str!("test_pipeline/computeNormal.frag.glsl");
    let normal_shader = ShaderProgram::simple_program(NORMAL_FRAG, PIPELINE_DEFAULT_VERT).unwrap();
    pipeline.add_render_node("normal", (1000, 1000), normal_shader);
    pipeline.link_render_to_node("multiplier", "heightMap", "normal");

    unsafe {
        pipeline.compute("normal");
    }

    let mut material = Material::from_program("copyShader");
    unsafe {
        if let Some(texture_attachment) = material.get_mat_properties::<TextureAttachmentProp>() {
            texture_attachment.attach_texture("copy_tex", pipeline.get_texture("normal", &None));

        }
    }
    let mesh_renderer = MeshRenderer::new(&mesh, material);

    let timer = TimingSystem{timer:0.0, pipeline};
    let system = System::new(Box::new(timer), UpdateFrequency::PerFrame);


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
    let mut camera_component = CameraComponent::new_perspective_camera(None, 80.0, 0.1, 100.0);
    camera_component.set_as_main(&mut world.components);
    let _camera = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(0.0, 0.0, 1.0)), camera_component);

    // start main loop
    world.register_system(system, 10);

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
        self.pipeline.set_float("island_mask", "time", self.timer);
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