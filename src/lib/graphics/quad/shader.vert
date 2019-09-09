#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 2) in vec2 a_offset;
layout(location = 3) in vec2 a_origin;
layout(location = 4) in vec2 a_scale;
layout(location = 5) in vec3 a_color;

layout(location = 0) out vec2 v_tex_coord;
layout(location = 1) out vec3 v_color;

void main() {
    v_tex_coord = a_tex_coord;
    v_color = a_color;
    gl_Position = vec4((a_pos - a_origin) * a_scale + a_offset, 0, 1);
}