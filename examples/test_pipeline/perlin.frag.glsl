#version 330 core

uniform sampler2D tex;
uniform float time;
uniform vec3 offset;

in VS_OUTPUT {
    vec2 UV;
} IN;

out vec4 FragColor;

vec3 hash(vec3 v) {
    return -1.+2.*vec3
    (
        fract(sin(dot(v, vec3(763., 827., 244.97))+26.)*9283.),
        fract(sin(dot(v, vec3(135., 236., 652.783))+145.)*422.),
        fract(sin(dot(v, vec3(1387., 249., 1376.21))+1246.)*1896.)
    );
}

float perlin(vec3 coords) {
    vec3 cube_coords = floor(coords);
    coords -= cube_coords;

    vec3 smooth_coords = smoothstep(0., 1., coords);

    float val = mix(
        mix(
        mix(
        dot(hash(cube_coords), coords),
        dot(hash(cube_coords+vec3(1., 0., 0.)), coords-vec3(1., 0., 0.)),
        smooth_coords.x
        )
        ,
        mix
        (
        dot(hash(cube_coords+vec3(0., 1., 0.)), coords-vec3(0., 1., 0.)),
        dot(hash(cube_coords+vec3(1., 1., 0.)), coords-vec3(1., 1., 0.)),
        smooth_coords.x
        )
        ,
        smooth_coords.y
        )
        ,
        mix
        (
        mix
        (
        dot(hash(cube_coords+vec3(0., 0., 1.)), coords-vec3(0., 0., 1.)),
        dot(hash(cube_coords+vec3(1., 0., 1.)), coords-vec3(1., 0., 1.)),
        smooth_coords.x
        )
        ,
        mix
        (
        dot(hash(cube_coords+vec3(0., 1., 1.)), coords-vec3(0., 1., 1.)),
        dot(hash(cube_coords+vec3(1., 1., 1.)), coords-vec3(1., 1., 1.)),
        smooth_coords.x
        )
        ,
        smooth_coords.y
        )
        ,
        smooth_coords.z
    );

    return val;
}

float fractal(vec3 pos) {
    float val = 0.;
    float pers = .5;
    float amp = 1.;
    float lac = 1.5;
    float freq = 1.;
    for (int i = 0; i < 16; i++) {
        val += perlin(pos*freq)*amp;
        amp*=pers;
        freq*=lac;
    }
    return val;
}

float ridges(vec3 pos, float shape) {
    return pow(clamp(1. - abs(fractal(pos)), 0., 1.), shape);
}

float turb(vec3 pos) {
    return (1.+sin(.1*pos.y+4.*ridges(pos/20., 1.)))*.5;
}


void main() {
    float mountain_scale = 6.;
    float terrain_scale = 17.;
    float terrain_amp = .5;

    vec2 uv = IN.UV + offset.xy;

    float terrain_detail = (fractal(vec3(IN.UV*terrain_scale, time))+1.)*.5*terrain_amp;
    float mountain = (ridges(vec3(IN.UV*mountain_scale, time), 10.));

    FragColor = vec4(vec3(terrain_detail*mountain+terrain_detail), 1.);
}
