#version 330 core

layout(location=0) in vec2 inPos;
layout(location=1) in vec2 inUv;

uniform mat3 modelMat;
uniform int layer;

out VS_OUTPUT {
    vec3 Position;
    vec2 UV;
} OUT;

void main()
{
    vec3 position = modelMat * vec3(inPos, 1.0);
    position.z = -0.95 + 0.9 / (1 + layer);
    gl_Position = vec4(position, 1.0);
    OUT.Position = position;
    OUT.UV = inUv;
}
