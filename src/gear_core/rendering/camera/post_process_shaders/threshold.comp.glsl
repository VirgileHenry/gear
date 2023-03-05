#version 430 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (location = 0, rgba32f) writeonly uniform image2D processed_image;
uniform sampler2D image_to_process;

uniform float threshold;

void main(void)
{
    vec2 uv = vec2(gl_GlobalInvocationID.xy+1.)/gl_NumWorkGroups.xy/gl_WorkGroupSize.xy;
    vec4 input_color = texture(image_to_process, uv);
    vec3 above_threshold_color = max(vec3(0), input_color.xyz-threshold);
    imageStore(processed_image, ivec2(gl_GlobalInvocationID.xy), vec4(above_threshold_color, 1.));
}