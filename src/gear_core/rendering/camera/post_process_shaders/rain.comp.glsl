#version 430 core

#define PI 3.141592

layout (local_size_x = 16, local_size_y = 16) in;

layout (rgba32f) writeonly uniform image2D rain_out;
uniform sampler2D rain_in;

uniform float horizontal_speed;
uniform float look_dir_y;

uniform float droplet_speed;
uniform float droplet_length;
uniform float droplet_density;
uniform float alpha;

uniform float time;

float ihash(ivec2 x) {
    x-=x/10000*10000;
    return fract(0.1267+sin(float(x.x*386+863))+sin(float(x.y*398+583)));
}

float rain_int(ivec2 coord) {
    ivec2 rain_x = ivec2(coord.x-int(float(coord.y)*horizontal_speed/100.), coord.y);
    rain_x.y += int(time*1000.*droplet_speed);
    rain_x.y -= int(mod(float(rain_x.y)+sin(float(rain_x.x)*50.), 30.*droplet_length));

    return 1.-smoothstep(ihash(rain_x), 0., 0.0001*droplet_density);
}

void main(void)
{
    ivec2 texcoord = ivec2(gl_GlobalInvocationID.xy);

    ivec2 tex_size = textureSize(rain_in, 0);
    vec4 pixel_val = texture(rain_in, vec2(texcoord)/vec2(tex_size));
    if (tex_size.x <= texcoord.x || tex_size.y <= texcoord.y) {
        return;
    }
    float deformation_fact = sqrt(1.-look_dir_y*look_dir_y);
    imageStore(rain_out, ivec2(gl_GlobalInvocationID.xy), mix(pixel_val, vec4(vec3(rain_int(ivec2(texcoord.xy))), 0.), alpha*deformation_fact));
}