#version 330 core
#define MAX_CHAR_COUNT 256
#define CHAR_SDF_SIZE 64

uniform int char_count;

uniform int char_index[MAX_CHAR_COUNT];
uniform vec2 char_pos[MAX_CHAR_COUNT];

in VS_OUTPUT {
    vec3 Position;
    vec3 Normal;
    vec2 UV;
} IN;

out vec4 Color;

void main() {
    for (int index = 0; index < char_count; char_index++) {
        if 
    }
}