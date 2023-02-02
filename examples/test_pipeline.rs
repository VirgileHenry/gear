use gl::TEXTURE_2D;
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

    let node = create();

    let mut material = Material::from_program("copyShader", Box::new(NoParamMaterialProperties{}));
    unsafe {
        let a = vec![];
        node.compute(&a);
        material.attach_texture(node.get_texture());
    }
    let mesh_renderer = MeshRenderer::new(mesh, material);


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
    let _camera = create_entity!(&mut world.components; Transform::origin().translated(Vector3::new(0.0, 0.0, 1.3)), camera_component);

    // start main loop
    engine.main_loop();

}


pub fn create() -> ShaderPipelineNode {
    pub static PERLIN_FRAG: &str = include_str!("test_pipeline/perlin.frag.glsl");
    let perlin_shader = ShaderProgram::simple_program(PERLIN_FRAG, PIPELINE_DEFAULT_VERT).unwrap();
    let perlin_node = ShaderPipelineNode::new((1000, 1000), vec![], perlin_shader, None);


    //pub static GRAYSCALE_FRAG: &str = include_str!("test_pipeline/gray_scale.frag.glsl");
    //let gray_shader = ShaderProgram::simple_program(GRAYSCALE_FRAG, PIPELINE_DEFAULT_VERT).unwrap();
    //let gray_node = ShaderPipelineNode::new((1000, 1000), vec![perlin_node], gray_shader, None);

    perlin_node
}
