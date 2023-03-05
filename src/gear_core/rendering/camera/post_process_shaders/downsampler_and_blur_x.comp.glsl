#version 430 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (location = 0, rgba32f) writeonly uniform image2D downsampled_tex;
uniform sampler2D input_tex;

uniform int blur_size;
uniform float sigma;

float gauss(int x) {
    return exp(-float(x*x)/sigma);
}

void main(void)
{
    vec2 texcoord = vec2(gl_GlobalInvocationID.xy)+1.;

    float total_w = 1;
    vec4 sum = texture(input_tex, texcoord/vec2(gl_NumWorkGroups.xy*gl_WorkGroupSize.xy));
    for (int i = 1; i < blur_size; i++) {
        float w = gauss(i);
        sum += w * texture(input_tex, (texcoord+vec2(i, 0.))/vec2(gl_NumWorkGroups.xy*gl_WorkGroupSize.xy));
        sum += w * texture(input_tex, (texcoord-vec2(i, 0.))/vec2(gl_NumWorkGroups.xy*gl_WorkGroupSize.xy));
        total_w += 2.*w;
    }

    imageStore(downsampled_tex, ivec2(gl_GlobalInvocationID.xy), sum/total_w);
}