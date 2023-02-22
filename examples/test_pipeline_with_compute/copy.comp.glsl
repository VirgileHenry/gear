#version 430 core
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(rgba32f, binding = 0) uniform image2D copy_texture;

uniform sampler2D copied_texture;

void main() {
    ivec2 tex_coord = ivec2(gl_GlobalInvocationID.xy);
    vec2 uv = vec2(tex_coord)/vec2(gl_NumWorkGroups.xy);
    imageStore(copy_texture, tex_coord, texture(copied_texture, uv));
}
