#version 330 core

uniform sampler2D heightMap;

in VS_OUTPUT {
    vec2 UV;
} IN;

out vec4 FragColor;

float height(vec2 p) {
    return texture(heightMap, p).x;
}

void main() {
    vec2 ps = 1./textureSize(heightMap, 0);

    vec2 ofx = vec2(ps.x, 0.);
    vec2 ofy = vec2(0., ps.y);

    float l = height(IN.UV-ofx);
    float r = height(IN.UV+ofx);
    float t = height(IN.UV+ofy);
    float b = height(IN.UV-ofy);

    vec3 n = normalize(vec3((l-r)/(2.*ps.x), (b-t)/(2.*ps.y), 1.));
    FragColor = vec4(n*.5+.5, 1.);
}
