#version 430 core

#define PI 3.141592

layout (local_size_x = 16, local_size_y = 16) in;

layout (rgba32f) writeonly uniform image2D result;
layout (rgba32f) readonly uniform image2D color_out;
uniform sampler2D input_tex;

uniform float a;
uniform float b;
uniform float time;

uniform mat4 projectionMat;
uniform mat4 viewMat;

uniform float aspect;
uniform float z_near;
uniform float half_fov;

uniform vec3 camPos;
uniform vec3 mainLightDir;
uniform vec3 mainLightColor;
uniform vec3 ambientColor;


vec3 hash(vec3 v) {
    return -1.+2.*vec3
    (
    fract(sin(dot(v, vec3(763., 827., 244.97))+26.)*9283.),
    fract(sin(dot(v, vec3(135., 236., 652.783))+145.)*422.),
    fract(sin(dot(v, vec3(1387., 249., 1376.21))+1246.)*1896.)
    );
}


vec3 ihash(ivec3 v) {
    return vec3
    (
    fract(sin(float(dot(v, ivec3(763, 647, 294))+256))*283.),
    fract(sin(float(dot(v, ivec3(135, 836, 652))+145))*422.),
    fract(sin(float(dot(v, ivec3(387, 249, 176))+1246))*196.)
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


vec3 applyFog( in vec3  rgb, in float distance ) // camera to point distance
{
    float fogAmount = 1.0 - exp( -distance*b );
    vec3  fogColor  = vec3(0.5,0.7,0.8);
    return mix( rgb, fogColor, fogAmount );
}

vec3 applyFog2( in vec3  rgb,      // original color of the pixel
in float distance, // camera to point distance
in vec3  rayDir,   // camera to point vector
in vec3  sunDir )  // sun light direction
{
    float fogAmount = 1.0 - exp( -distance*b );
    float sunAmount = max( dot( rayDir, sunDir ), 0.0 );
    vec3  fogColor  = mix( vec3(0.5,0.6,0.7)*ambientColor, // bluish
    mainLightColor/2., // yellowish
    pow(sunAmount,8.0) );
    return mix( rgb, fogColor, fogAmount );
}

vec3 applyFog3( in vec3  rgb,      // original color of the pixel
in float distance, // camera to point distance
in vec3  rayOri,   // camera position
in vec3  rayDir , // camera to point vector
in vec3  sunDir )
{
    float fogAmount = (a/b) * exp(-rayOri.y*b) * (1.0-exp( -distance*rayDir.y*b ))/rayDir.y;
    float sunAmount = max( dot( rayDir, sunDir ), 0.0 );

    vec3  fogColor  = vec3(0.7,0.8,0.9)*ambientColor*3.+mainLightColor/2. * pow(sunAmount,8.0);
    return mix( rgb, fogColor, clamp(fogAmount, 0., 1.) );
}

bool box_intersect(vec3 box_center, vec3 box_radius, vec3 ray_ogn, vec3 ray_dir, out float d_min, out float d_max) {
    vec3 invDir = 1./ray_dir;

    vec3 tbot = invDir * (box_center-box_radius - ray_ogn);
    vec3 ttop = invDir * (box_center+box_radius - ray_ogn);
    vec3 tmin = min(ttop, tbot);
    vec3 tmax = max(ttop, tbot);
    vec2 t = max(tmin.xx, tmin.yz);
    float t0 = max(t.x, t.y);
    t = min(tmax.xx, tmax.yz);
    float t1 = min(t.x, t.y);
    d_min = t0;
    d_max = t1;
    return t1 > max(t0, 0.0);
}

float HenyeyGreenstein(float g, float costh) {
    return (1. / (4.*PI)) * ((1.-g*g)/pow(1.+g*g-2.*g*costh, 1.5));
}

float phase_func(float g, float costh) {
    return mix(HenyeyGreenstein(-g, costh), HenyeyGreenstein(g, costh), 0.7);
}


float density(vec3 p) {
    float freq = 1.;
    float amp = 1.;
    float v = 0.;
    for (int i = 0; i < 7; ++i) {
        v+=(perlin(p*freq)+.1)*amp;
        amp*=.5;
        freq*=2.;
    }
    return max(0.,v);
}

#define EXT_MULT 1.
float scattering(float density, float mu) {
    float attenuation = 0.2;
    float contribution = 0.4;
    float phase_attenuation = 0.1;
    const float scattering_octaves = 4.0;

    float a = 1.0;
    float b = 1.0;
    float c = 1.0;
    float g = 0.85;

    float luminance = 0.0;

    for (float i = 0.; i < scattering_octaves; ++i) {
        float phase_f = phase_func(0.3 * c, mu);
        float beer = exp(-attenuation*EXT_MULT*a);
        luminance+=b*phase_f*beer;
        a*=attenuation;
        b*=contribution;
        c*=(1.-phase_attenuation);
    }
    return luminance;

}

void main(void)
{
    ivec2 texcoord = ivec2(gl_GlobalInvocationID.xy);
    ivec2 tex_size = textureSize(input_tex, 0);
    if (tex_size.x <= texcoord.x || tex_size.y <= texcoord.y) {
        return;
    }

    float near = 0.3;
    float far = 20000.;
    vec2 uv = vec2(gl_GlobalInvocationID.xy+ivec2(1))/tex_size;

    float h = tan(half_fov*PI/180.)*z_near;
    float x = h * aspect;

    vec4 raya = inverse(projectionMat) * vec4((uv-.5)*2., 0., 1.);
    vec3 front = normalize(inverse(mat3(viewMat)) * raya.xyz);

    float d = texture(input_tex, uv).r;
    float ndc = d * 2.0 - 1.0;
    float linearDepth = (2.0 * near * far) / (far + near - ndc * (far - near));
    vec3 color = imageLoad(color_out, ivec2(gl_GlobalInvocationID.xy)).xyz;


    float to_water_fact = clamp(-camPos.y/front.y/linearDepth, 0., 1.);

    float sun_alignment = clamp(dot(front, -mainLightDir), 0., 1.);

    vec3 final_fog = color;
    float underwater_dst = linearDepth;
    if (front.y > 0.) {
        underwater_dst = min(linearDepth * to_water_fact, underwater_dst);
    }
    if (camPos.y < 0.) {
        vec3 light_blue = vec3(0.05, 0.1, 0.5);
        vec3 deep_blue = vec3(0.003, 0.009, 0.03);
        vec3 blue = mix(light_blue, deep_blue, smoothstep(0., -300., camPos.y + front.y * 10.));
        final_fog = mix(final_fog, blue, 1.-exp(- .02 * underwater_dst));
    }
    if (camPos.y > -20.) {
        final_fog = applyFog3(final_fog, linearDepth, camPos, front.xyz, -mainLightDir);
        float d_min, d_max;
        float attenuation = 0.03;
        float cloud_height = 700.;
        float cloud_scale = 2000.;

        vec3 lightning_col = vec3(2., 2., 0.4);
        float ligthning_time = fract(time);
        vec3 ligthning_pos = (ihash(ivec3(floor(time)))-.5)*vec3(400, 100, 400);

        if (box_intersect(vec3(camPos.x, cloud_height, camPos.z), vec3(20000., 500., 20000.), camPos, front, d_min, d_max)) {
            d_min = max(0., d_min);
            d_max = min(linearDepth, d_max);
            float distance = 0.;
            vec3 cloud_col = max(mainLightColor, vec3(.3));
            float t = d_min;
            float dt = min(30., (d_max-d_min)/40.);
            vec3 point = vec3(camPos+d_min*front);
            for (int i = 0; i < 40; ++i) {
                if (t > d_max) { break; }
                float perlin_density = perlin(point/cloud_scale);
                t += dt+cloud_scale*(perlin_density*.5+.5)*.02;
                point = camPos + t * front;
                float step_distance = 0.;
                //cloud_col += lightning_col*smoothstep(100., 0., length(point-ligthning_pos));
                distance += dt*max(0., density(point/300.)*perlin_density*(1.-exp(-(point.y-cloud_height)*0.001)));
            }
            final_fog = mix(cloud_col, final_fog, exp(-distance*attenuation));
        }
    }



    imageStore(result, ivec2(gl_GlobalInvocationID.xy), vec4(final_fog, 0.));
}