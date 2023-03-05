use crate::{ComputeShader, ShaderPipeline, Texture2D, TexturePresets};

static DISPATCH_GROUP_SIZE: (i32, i32) = (16, 16);
static DOWN_SAMPLING_STEPS: i32 = 4;

pub fn create_post_processing_pipeline(processed_image: &Texture2D) -> ShaderPipeline {
    let mut pipeline = ShaderPipeline::new();

    let tex_dim = processed_image.get_dimensions();

    let threshold_compute_source: &str = include_str!("post_process_shaders/threshold.comp.glsl");
    let threshold_output_tex = Texture2D::new_from_presets((tex_dim.0/2, tex_dim.1/2), TexturePresets::pipeline_default(), None);
    let mut threshold_compute_shader = ComputeShader::new(threshold_compute_source, (tex_dim.0/2/DISPATCH_GROUP_SIZE.0, tex_dim.1/2/DISPATCH_GROUP_SIZE.1, 1));
    threshold_compute_shader.add_write_texture("processed_image", threshold_output_tex);
    pipeline.add_compute_node("threshold", threshold_compute_shader);
    pipeline.set_input_texture("image_to_process", processed_image.clone(), "threshold");
    pipeline.set_float("threshold", "threshold",1.0);

    let downsampler_and_blur_x_source = include_str!("post_process_shaders/downsampler_and_blur_x.comp.glsl");
    let blur_y_source = include_str!("post_process_shaders/blur_y.comp.glsl");
    let mut downsampler_and_blur_x_compute = ComputeShader::new(downsampler_and_blur_x_source, (tex_dim.0/DISPATCH_GROUP_SIZE.0, tex_dim.1/DISPATCH_GROUP_SIZE.1, 1));
    let mut blur_y_compute = ComputeShader::new(blur_y_source, (tex_dim.0/DISPATCH_GROUP_SIZE.0, tex_dim.1/DISPATCH_GROUP_SIZE.1, 1));

    let mut downsample_dim = (tex_dim.0/4, tex_dim.1/4);
    let mut previous_node = String::from("threshold");
    let mut previous_node_output_tex = String::from("processed_image");

    for mip in 2..(2+DOWN_SAMPLING_STEPS) {
        // downsampling + horizontal blur
        let mut node_downsampler_and_blur_x_compute = downsampler_and_blur_x_compute.clone();
        node_downsampler_and_blur_x_compute.set_dispatch_dimensions((downsample_dim.0/DISPATCH_GROUP_SIZE.0, downsample_dim.1/DISPATCH_GROUP_SIZE.1, 1));
        let downsampled_tex = Texture2D::new_from_presets(downsample_dim, TexturePresets::pipeline_default(), None);
        let downsampled_tex_name = String::from("downsampled_tex");
        node_downsampler_and_blur_x_compute.add_write_texture(&downsampled_tex_name, downsampled_tex);
        let node_downsampler_and_blur_x_name = format!("downsample_blur_x_{mip}");
        pipeline.add_compute_node(&node_downsampler_and_blur_x_name, node_downsampler_and_blur_x_compute);
        pipeline.set_float(&node_downsampler_and_blur_x_name, "sigma", 20.);
        pipeline.set_int(&node_downsampler_and_blur_x_name, "blur_size", 10);
        pipeline.link_compute_to_node(
            &previous_node,
            &previous_node_output_tex,
            "input_tex",
            &node_downsampler_and_blur_x_name,
        );

        previous_node = node_downsampler_and_blur_x_name;
        previous_node_output_tex = downsampled_tex_name;

        // vertical blur
        let mut node_blur_y = blur_y_compute.clone();
        node_blur_y.set_dispatch_dimensions((downsample_dim.0/DISPATCH_GROUP_SIZE.0, downsample_dim.1/DISPATCH_GROUP_SIZE.1, 1));
        let fully_blurred_tex = Texture2D::new_from_presets(downsample_dim, TexturePresets::pipeline_default(), None);
        let fully_blurred_tex_name = String::from("blurred_tex");
        node_blur_y.add_read_write_texture(&fully_blurred_tex_name, fully_blurred_tex);
        let node_blur_y_name = format!("blur_y_{mip}");

        pipeline.add_compute_node(&node_blur_y_name, node_blur_y);
        pipeline.set_float(&node_blur_y_name, "sigma", 20.);
        pipeline.set_int(&node_blur_y_name, "blur_size", 10);
        pipeline.link_compute_to_node(
            &previous_node,
            &previous_node_output_tex,
            "input_tex",
            &node_blur_y_name
        );

        previous_node = node_blur_y_name;
        previous_node_output_tex = fully_blurred_tex_name;
        downsample_dim = (downsample_dim.0/2, downsample_dim.1/2);
    }

    let additive_source: &str = include_str!("post_process_shaders/additive_blender.comp.glsl");
    let additive_output_tex = Texture2D::new_from_presets((tex_dim.0, tex_dim.1), TexturePresets::pipeline_default(), None);
    let mut additive_compute_shader = ComputeShader::new(additive_source, (tex_dim.0/DISPATCH_GROUP_SIZE.0, tex_dim.1/DISPATCH_GROUP_SIZE.1, 1));
    additive_compute_shader.add_read_texture("tex_before_threshold", processed_image.clone());
    additive_compute_shader.add_write_texture("result", additive_output_tex);
    pipeline.add_compute_node("additive_blender", additive_compute_shader);

    for mip in 2..(2+DOWN_SAMPLING_STEPS) {
        let node_output_name = format!("blur_y_{mip}");
        let node_input_tex_name = format!("blurred[{}]", mip-2);
        pipeline.link_compute_to_node(
            &node_output_name,
            "blurred_tex",
            &node_input_tex_name,
            "additive_blender"
        );
    }

    let gamma_correction_source: &str = include_str!("post_process_shaders/gamma_correction.comp.glsl");
    let gamma_output_tex = Texture2D::new_from_presets((tex_dim.0, tex_dim.1), TexturePresets::pipeline_default(), None);
    let mut gamma_correction_compute = ComputeShader::new(gamma_correction_source, (tex_dim.0/DISPATCH_GROUP_SIZE.0, tex_dim.1/DISPATCH_GROUP_SIZE.1, 1));
    gamma_correction_compute.add_write_texture("processed_image", gamma_output_tex);
    pipeline.add_compute_node("gamma_correction", gamma_correction_compute);
    pipeline.link_compute_to_node(
        "additive_blender",
        "result",
        "input_tex",
        "gamma_correction"
    );


    pipeline
}
