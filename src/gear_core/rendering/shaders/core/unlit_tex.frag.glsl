#version 330 core
// object color
uniform vec3 color;
// light values
uniform vec3 ambientColor;
uniform vec3 mainLightPos;
uniform vec3 mainLightColor;
// user values
uniform vec3 camPos;

uniform sampler2D u_tex_sampler;

in VS_OUTPUT {
    vec3 Position;
    vec3 Normal;
    vec2 UV;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(texture(u_tex_sampler, IN.UV).xyz, 1.);
}