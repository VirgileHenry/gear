#version 430 core

#define PI 3.141592

layout (local_size_x = 16, local_size_y = 16) in;

layout (rgba32f) writeonly uniform image2D fog_tex;
layout (rgba32f) readonly uniform image2D color_out;
uniform sampler2D input_tex;

uniform float a;
uniform float b;

uniform mat4 projectionMat;
uniform mat4 viewMat;

uniform float aspect;
uniform float z_near;
uniform float half_fov;

uniform vec3 camPos;
uniform vec3 mainLightDir;
uniform vec3 mainLightColor;
uniform vec3 ambientColor;

vec3 applyFog( in vec3  rgb, in float distance ) // camera to point distance
{
    float fogAmount = 1.0 - exp( -distance*b );
    vec3  fogColor  = vec3(0.5,0.6,0.7);
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

    vec3  fogColor  = mix( vec3(0.7,0.8,0.9)*ambientColor*3., // bluish
    mainLightColor/2., // yellowish
    pow(sunAmount,8.0) );
    return mix( rgb, fogColor, clamp(fogAmount, 0., 1.) );
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
    vec2 uv = vec2(gl_GlobalInvocationID.xy)/gl_NumWorkGroups.xy/gl_WorkGroupSize.xy;

    float h = tan(half_fov*PI/180.)*z_near;
    float x = h * aspect;

    vec4 rayb = projectionMat * vec4((uv-vec2(.5))*vec2(x, h)*2., -z_near, 1.);
    vec4 front = normalize(inverse(viewMat) * vec4(rayb.xyz, 0.));

    float d = texture(input_tex, uv).r;
    float ndc = d * 2.0 - 1.0;
    float linearDepth = (2.0 * near * far) / (far + near - ndc * (far - near));
    vec3 color = imageLoad(color_out, ivec2(gl_GlobalInvocationID.xy)).xyz;
    imageStore(fog_tex, ivec2(gl_GlobalInvocationID.xy), vec4(applyFog3(color, linearDepth, camPos, front.xyz, -mainLightDir), 0.));
    imageStore(fog_tex, ivec2(gl_GlobalInvocationID.xy), vec4(color, 1.));
}