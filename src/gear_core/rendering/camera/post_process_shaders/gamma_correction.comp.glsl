#version 430 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (rgba32f) writeonly uniform image2D result;
uniform sampler2D input_tex;


const mat3 ACESInputMat = mat3(
0.59719, 0.35458, 0.04823,
0.07600, 0.90834, 0.01566,
0.02840, 0.13383, 0.83777
);

const mat3 ACESOutputMat = mat3(
1.60475, -0.53108, -0.07367,
-0.10208, 1.10813, -0.00605,
-0.00327, -0.07276, 1.07602
);

vec3 RRTAndODTFit(vec3 v)
{
    vec3 a = v * (v + 0.0245786) - 0.000090537;
    vec3 b = v * (0.983729 * v + 0.4329510) + 0.238081;
    return a / b;
}

vec3 ACESFitted(vec3 color)
{
    color = transpose(ACESInputMat) * color;

    // Apply RRT and ODT
    color = RRTAndODTFit(color);

    color = transpose(ACESOutputMat) * color;

    // Clamp to [0, 1]
    color = clamp(color, 0.0, 1.0);

    return color;
}


void main(void)
{

    ivec2 texcoord = ivec2(gl_GlobalInvocationID.xy);
    ivec2 tex_size = textureSize(input_tex, 0);
    if (tex_size.x <= texcoord.x || tex_size.y <= texcoord.y) {
        return;
    }

    vec2 uv = vec2(gl_GlobalInvocationID.xy)/gl_NumWorkGroups.xy/gl_WorkGroupSize.xy;
    vec4 input_color = texture(input_tex, uv);
    vec4 output_color = vec4(pow(ACESFitted(input_color.xyz), vec3(1./2.2)), 1.);
    imageStore(result, ivec2(gl_GlobalInvocationID.xy), output_color);
}