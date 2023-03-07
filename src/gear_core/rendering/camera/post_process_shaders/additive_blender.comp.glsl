#version 430 core
#define DOWN_SAMPLING_STEPS 4

layout (local_size_x = 16, local_size_y = 16) in;

layout (binding = 0, rgba32f) writeonly uniform image2D result;
layout (binding = 1, rgba32f) readonly uniform image2D tex_before_threshold;
layout (location = 2) uniform sampler2D blurred[DOWN_SAMPLING_STEPS];

void main(void)
{
    vec2 uv = vec2(gl_GlobalInvocationID.xy)/gl_NumWorkGroups.xy/gl_WorkGroupSize.xy;
    vec4 sum = imageLoad(tex_before_threshold, ivec2(gl_GlobalInvocationID.xy));
    for (int i = 0; i < DOWN_SAMPLING_STEPS; i++) {
        sum += texture(blurred[i], uv);
    }
    imageStore(result, ivec2(gl_GlobalInvocationID.xy), sum);
}