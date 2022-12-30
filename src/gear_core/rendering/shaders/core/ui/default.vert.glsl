#version 330 core

layout(location=0) in vec3 inPos;
layout(location=1) in vec3 inNormal;
layout(location=2) in vec2 inUv;

uniform mat4 projectionMat;     // not used but is ok
uniform mat4 viewMat;    // not used but is ok
uniform mat4 modelMat;

out VS_OUTPUT {
    vec3 Position;
    vec3 Normal;
    vec2 UV;
} OUT;

void main()
{
    gl_Position = modelMat * vec4(inPos, 1.0);
    OUT.Position = vec3(modelMat * vec4(inPos, 1.0));
    OUT.Normal = mat3(transpose(inverse(modelMat))) * inNormal; // todo : compute once in cpu
    OUT.UV = inUv;
}
