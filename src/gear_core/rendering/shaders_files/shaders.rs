

pub static DEFAULT_VERT_SHADER: &str = "#version 330 core

layout(location=0) in vec3 inPos;
layout(location=1) in vec3 inNormal;
layout(location=2) in vec2 inUv;

uniform mat4 projectionMat;
uniform mat4 cameraWorldPos;
uniform mat4 modelWorldPos;

out VS_OUTPUT {
    vec3 Normal;
    vec2 UV;
} OUT;
  
void main()
{
    gl_Position = projectionMat * cameraWorldPos * modelWorldPos * vec4(inPos, 1.0);
    OUT.Normal = inNormal;
    OUT.UV = inUv;
}
";

pub static DEFAULT_FRAG_SHADER: &str = "#version 330 core

in VS_OUTPUT {
    vec3 Normal;
    vec2 UV;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(1.0f, 1.0f, 1.0f, 1.0f); // vec4(Color, 1.0f)
}";

pub static MISSING_FRAG_SHADER: &str = "#version 330 core

in VS_OUTPUT {
    vec3 Normal;
    vec2 UV;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(1.0f, 0.0f, 1.0f, 1.0f);
}";

