#version 330 core

layout(location=0) in vec3 inPos;
layout(location=1) in vec3 inNormal;
layout(location=2) in vec2 inUv;

uniform mat4 projectionMat;
uniform mat4 cameraWorldPos;
uniform mat4 modelWorldPos;

out VS_OUTPUT {
    vec3 Normal;
    vec3 Position;
} OUT;
  
void main()
{
    gl_Position = vec4(inPos, 1.0); // projectionMat * cameraWorldPos * modelWorldPos * 
    OUT.Normal = inNormal;
}