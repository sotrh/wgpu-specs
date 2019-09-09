#version 450

layout(location = 0) in vec2 v_tex_coord;
layout(location = 1) in vec3 v_color;

layout(location = 0) out vec4 o_target;

layout(set = 0, binding = 1) uniform texture2D t_color;
layout(set = 0, binding = 2) uniform sampler s_color;

void main() {
    vec4 tex = texture(sampler2D(t_color, s_color), v_tex_coord);
    o_target = tex * vec4(v_color, 1);
}