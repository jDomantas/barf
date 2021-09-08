#version 450

layout(location=0) in vec2 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=2) in vec4 inst_pos;
layout(location=3) in vec2 inst_scale;
layout(location=4) in vec2 tex_source_pos;
layout(location=5) in vec2 tex_source_size_x;
layout(location=6) in vec2 tex_source_size_y;
layout(location=7) in vec4 inst_color;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec4 v_color;

void main() {
    v_tex_coords = tex_source_pos
        + a_tex_coords.x * tex_source_size_x
        + a_tex_coords.y * tex_source_size_y;
    v_color = inst_color;
    mat4 inst_transform = mat4(
        vec4(inst_scale.x, 0, 0, 0),
        vec4(0, inst_scale.y, 0, 0),
        vec4(0, 0, 1, 0),
        vec4(inst_pos.x, inst_pos.y, inst_pos.z, 1)
    );
    mat4 vertical_flip = mat4(
        1, 0, 0, 0,
        0, -1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    );
    gl_Position = vertical_flip * inst_transform * vec4(a_position, 0.0, 1.0);
}
