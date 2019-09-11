#version 450

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 2) in vec2 a_offset;
layout(location = 3) in vec2 a_origin;
layout(location = 4) in vec2 a_scale;
layout(location = 5) in float a_rotation;
layout(location = 6) in vec3 a_color;

layout(location = 0) out vec2 v_tex_coord;
layout(location = 1) out vec3 v_color;

void main() {
    v_tex_coord = a_tex_coord;
    v_color = a_color;
    vec2 pos = (a_pos - a_origin) * a_scale;
    pos = vec2(
        pos.x * cos(a_rotation) + pos.y * sin(a_rotation), 
        -pos.x * sin(a_rotation) + pos.y * cos(a_rotation)
    );
    gl_Position = vec4(pos + a_offset, 0, 1);
}