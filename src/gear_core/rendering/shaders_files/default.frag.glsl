#version 330 core

in VS_OUTPUT {
    vec3 Normal;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(1.0f, 1.0f, 1.0f, 1.0f); // vec4(Color, 1.0f)
}