#version 430 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (rgba32f) writeonly uniform image2D downsampled_tex;
uniform sampler2D input_tex;

uniform int blur_size;
uniform float sigma;

float gauss(int x) {
    return exp(-float(x*x)/sigma);
}

void main(void)
{
    ivec2 texcoord = ivec2(gl_GlobalInvocationID.xy);
    ivec2 tex_size = textureSize(input_tex, 0);
    if (tex_size.x <= texcoord.x || tex_size.y <= texcoord.y) {
        return;
    }

    float total_w = 1;
    vec4 sum = texelFetch(input_tex, texcoord*2, 0);
    for (int i = 1; i < blur_size; i++) {
        float w = gauss(i);
        sum += w * texelFetch(input_tex, texcoord*2+2*ivec2(i, 0.), 0);
        sum += w * texelFetch(input_tex, texcoord*2-2*ivec2(i, 0.), 0);
        total_w += 2.*w;
    }

    imageStore(downsampled_tex, ivec2(gl_GlobalInvocationID.xy), sum/total_w);
}