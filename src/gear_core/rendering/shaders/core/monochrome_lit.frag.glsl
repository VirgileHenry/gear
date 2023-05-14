#version 330 core
#define MAX_POINT_LIGHT_COUNT 10

// object color
uniform vec3 color;
// light values
uniform vec3 ambientColor;
uniform vec3 mainLightDir;
uniform vec3 mainLightColor;
// user values
uniform vec3 camPos;

uniform int lightCount;
uniform vec3 lightCol[MAX_POINT_LIGHT_COUNT];
uniform vec3 lightPos[MAX_POINT_LIGHT_COUNT];


in VS_OUTPUT {
    vec3 Position;
    vec3 Normal;
    vec2 UV;
} IN;

out vec4 Color;

void main()
{
    // ambiant lighting
    vec3 ambient = ambientColor;

    // direct lighting
    vec3 normal = normalize(IN.Normal);
    vec3 diffuse = max(dot(normal, mainLightDir), 0.0) * mainLightColor;
    for (int i = 0; i < lightCount; ++i) {
        vec3 dir = lightPos[i]-IN.Position;
        float dist = length(dir);
        dir/=dist;
        float fact = min(3., 1. / dist / dist);
        diffuse += max(dot(normal, dir), 0.0) * fact * lightCol[i];
    }

    // specular lighting
    float specularStrength = 0.5f;
    vec3 viewDir = normalize(camPos - IN.Position);
    vec3 reflectDir = reflect(-mainLightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * mainLightColor;

    // final color in linear space
    vec3 result = (ambient + diffuse + specular) * color;

    // gamma correction
    Color = vec4(result, 1.0f);
}