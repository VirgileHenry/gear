#version 430 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (location = 0, rgba32f) writeonly uniform image2D processed_image;
uniform sampler2D input_tex;


void main(void)
{
    vec2 uv = vec2(gl_GlobalInvocationID.xy)/gl_NumWorkGroups.xy/gl_WorkGroupSize.xy;
    vec4 input_color = texture(input_tex, uv);
    vec4 output_color = vec4(pow(input_color.xyz, vec3(1./2.4)), 1.);
    imageStore(processed_image, ivec2(gl_GlobalInvocationID.xy), output_color);
}