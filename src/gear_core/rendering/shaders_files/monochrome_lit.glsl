#version 330 core

uniform vec3 color;
uniform vec3 ambientColor;
uniform vec3 mainLightPos;
uniform vec3 mainLightColor;

in VS_OUTPUT {
    vec3 Position;
    vec3 Normal;
    vec2 UV;
} IN;

out vec4 Color;

void main()
{
    vec3 lightDir = normalize(IN.Position - mainLightPos)
    vec3 mainLightImpact = mainLightColor * dot(IN.Normal, lightDir);
    Color = vec4(color * (ambientColor + mainLightImpact) , 1.0f);
};