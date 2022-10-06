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
    vec3 normal = normalize(IN.Normal);
    vec3 ambient = ambientColor * color;
    vec3 lightDir = normalize(mainLightPos - IN.Position);
    vec3 diffuse = max(dot(normal, lightDir), 0.0) * mainLightColor;
    float specularStrength = 0.5f;
    vec3 viewDir = normalize(camPos - IN.Position);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * mainLightColor; 
    vec3 result = (ambient + diffuse + specular) * color;
    Color = vec4(result * color, 1.0f);
}