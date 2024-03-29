#version 430 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (rgba32f) writeonly uniform image2D processed_image;
uniform sampler2D image_to_process;

uniform float threshold;

void main(void)
{
    ivec2 texcoord = ivec2(gl_GlobalInvocationID.xy);
    ivec2 tex_size = textureSize(image_to_process, 0);
    if (tex_size.x <= texcoord.x || tex_size.y <= texcoord.y) {
        return;
    }

    vec2 uv = vec2(gl_GlobalInvocationID.xy)/gl_NumWorkGroups.xy/gl_WorkGroupSize.xy;
    vec4 input_color = texelFetch(image_to_process, texcoord*2, 0);
    vec3 above_threshold_color = max(vec3(0), input_color.xyz-threshold);
    imageStore(processed_image, ivec2(gl_GlobalInvocationID.xy), vec4(above_threshold_color, 1.));
}