#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec4 v_color;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    vec4 color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    if (color.w != 0) {
        f_color = vec4(
            color.x * v_color.x,
            color.y * v_color.y,
            color.z * v_color.z,
            1.0
        );
    } else {
        discard;
    }
}
