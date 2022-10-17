#version 330 core
// object color
uniform vec3 color;
// light values
uniform vec3 ambientColor;
uniform vec3 mainLightPos;
uniform vec3 mainLightColor;
// user values
uniform vec3 camPos;

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
    vec3 lightDir = normalize(mainLightPos - IN.Position);
    vec3 diffuse = max(dot(normal, lightDir), 0.0) * mainLightColor;

    // specular lighting
    float specularStrength = 0.5f;
    vec3 viewDir = normalize(camPos - IN.Position);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * mainLightColor;

    // final color in linear space
    vec3 result = (ambient + diffuse + specular) * color;

    // gamma correction
    vec3 result_srgb = pow(result, vec3(1./2.2));
    Color = vec4(result_srgb, 1.0f);
}