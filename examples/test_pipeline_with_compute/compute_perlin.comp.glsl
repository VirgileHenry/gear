#version 430 core
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(rgba32f, binding = 0) uniform image2D perlin_texture;
layout(rgba32f, binding = 1) uniform image2D uv_texture;

uniform float time;

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

    ivec2 tex_coord = ivec2(gl_GlobalInvocationID.xy);
    vec2 uv = vec2(tex_coord)/vec2(gl_NumWorkGroups.xy);

    float terrain_detail = (fractal(vec3(uv*terrain_scale, time))+1.)*.5*terrain_amp;
    float mountain = (ridges(vec3(uv*mountain_scale, time), 10.));

    vec4 color = vec4(vec3(terrain_detail*mountain+terrain_detail), 1.);
    imageStore(perlin_texture, tex_coord, color);
    imageStore(uv_texture, tex_coord, vec4(uv, 0, 1));
}
