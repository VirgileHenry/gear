
#version 330 core

#define texture_count 20

out vec4 FragColor;

uniform sampler2D u_textures[texture_count];
uniform ivec2 u_texture_pos[texture_count];

uniform ivec2 u_resolution;

in VS_OUTPUT {
    vec3 Position;
    vec3 Normal;
    vec2 UV;
} IN;

out vec4 Color;

void main() {
    Color = vec4(0.);
    ivec2 iCoords = ivec2(u_resolution * IN.UV);
    for (int i = 0; i < texture_count; i++) {
        ivec2 tex_size = textureSize(u_textures[i], 0);
        ivec2 delta_coords = (iCoords - u_texture_pos[i]);
        if (delta_coords/tex_size == ivec2(0)) {
            Color = texelFetch(u_textures[i], delta_coords, 0);
        }
    }
    Color = vec4(1., 1., 1., .5);
}