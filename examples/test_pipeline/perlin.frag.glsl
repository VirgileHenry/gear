#version 330 core

uniform sampler2D tex;
uniform float time;

in VS_OUTPUT {
    vec2 UV;
} IN;

out vec4 Color;

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

    float val = mix
    (
    mix
    (
    mix
    (
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

vec3 colormap1(float val) {
    vec3 a = vec3(.65, .0, .0);
    vec3 b = vec3(.75, .75, .65);
    vec3 c = vec3(.35, .82, .98);
    float da = .2;
    float db = .4;
    float dc = .1;
    float wa = .02;
    float wb = .03;
    float wc = .001;

    return a * exp(-pow(val-da, 2.)/wa) + b * exp(-pow(val-db, 2.)/wb) + c * exp(-pow(val-dc, 2.)/wc);
}

vec3 colormap2(float val) {
    vec3 a = vec3(178., 145., 124.)/256.;
    vec3 b = vec3(.9, .9, .9)*.7;
    vec3 c = vec3(.5, .3, .0);
    float da = 0.5;
    float db = 1.;
    float dc = .5;
    float wa = 10.;
    float wb = .01;
    float wc = .1;

    return a * exp(-pow(val-da, 2.)/wa) + b * exp(-pow(val-db, 2.)/wb) + c * exp(-pow(val-dc, 2.)/wc);
}

void main() {
    float val = (fractal(vec3(IN.UV*10., time))+1.)*.5;
    FragColor = vec4(vec3(val), 1.);
}