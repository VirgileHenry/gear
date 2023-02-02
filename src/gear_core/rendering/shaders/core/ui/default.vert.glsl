#version 330 core

layout(location=0) in vec2 inPos;
layout(location=1) in vec2 inUv;
layout(location=2) in int inDepth;

uniform mat3 modelMat;

out VS_OUTPUT {
    vec3 Position;
    vec2 UV;
} OUT;

void main()
{
    vec3 position = vec3((modelMat * vec3(inPos, 1.0)).xy, inDepth);
    gl_Position = vec4(position, 1.0);
    OUT.Position = vec3(modelMat * vec3(inPos, 1.0));
    OUT.UV = inUv;
}
