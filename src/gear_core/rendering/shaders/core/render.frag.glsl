#version 330 core

uniform sampler2D tex;

out vec4 FragColor;

in VS_OUTPUT {
    vec3 Position;
    vec3 Normal;
    vec2 UV;
} IN;

out vec4 Color;

void main() {
    Color = vec4(pow(texture(tex, IN.UV).xyz, vec3(1./2.2)), 1.);
}