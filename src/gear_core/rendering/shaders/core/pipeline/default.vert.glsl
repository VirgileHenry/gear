#version 330 core

layout(location=0) in vec3 inPos;
layout(location=1) in vec3 inNormal;
layout(location=2) in vec2 inUv;

out VS_OUTPUT {
    vec2 UV;
} OUT;

void main()
{
    gl_Position = vec4(inPos, 1.0);
    OUT.UV = inUv;
}
