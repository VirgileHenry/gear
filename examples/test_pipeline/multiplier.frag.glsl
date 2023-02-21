#version 330 core

uniform sampler2D height_map;
uniform sampler2D mask_tex;

uniform float time;
uniform vec3 offset;
uniform float a;
uniform float b;
uniform int shape;


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

float reshapefunc(float v) {
    float mountain = pow(v, shape)*(1.-b);
    float ocean = smoothstep(0., a, v)*b;
    return ocean+mountain;
}

void main() {
    float final_height = texture(height_map, IN.UV).r;
    float fact = 0.;
    float noise_mask = texture(mask_tex, IN.UV).r;

    FragColor = vec4(vec3(reshapefunc(final_height*noise_mask*20.)), 1.);
}