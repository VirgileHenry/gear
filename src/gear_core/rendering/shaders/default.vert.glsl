#version 330 core

layout(location=0) in vec3 inPos;
layout(location=1) in vec3 inColor;

uniform mat4 projectionMat;
uniform mat4 cameraWorldPos;
uniform mat4 modelWorldPos;

out VS_OUTPUT {
    vec3 Color;
} OUT;
  
void main()
{
    gl_Position = projectionMat * cameraWorldPos * modelWorldPos * vec4(inPos, 1.0);
    OUT.Color = inColor;
}