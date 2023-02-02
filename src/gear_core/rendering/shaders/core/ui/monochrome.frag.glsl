#version 330 core
// object color
uniform vec3 color;

in VS_OUTPUT {
    vec3 Position;
    vec2 UV;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(color, 1.0f);
}