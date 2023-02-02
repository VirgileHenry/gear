#version 330 core

uniform sampler2D input_tex;

in VS_OUTPUT {
    vec2 UV;
} IN;

out vec4 Color;

out vec4 FragColor;


void main() {

    FragColor = vec4(texture(input_tex, IN.UV).rrr, 1.);
}