#include <common.glsl>

#ifdef VERTEX_SHADER
uniform float u_size;
uniform float u_radius;
uniform float u_padding = 0.0;
uniform float u_toughness;
uniform vec2 u_position;
uniform vec2 u_position_2;
const float THICKNESS = 0.05;

out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;

void main() {
    vec2 direction = u_position_2 - u_position;
    v_quad_pos = a_pos * (1.0 + u_padding);
    vec2 pos = u_position + direction * step(0, a_pos.y);
    pos += rotateCW(normalize(direction), PI / 2.) * a_pos.x * THICKNESS * (1. - length(direction) / u_toughness);
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.0);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 v_quad_pos;

void main() {
    float len = length(v_quad_pos);
    gl_FragColor = mix(u_color_1, u_color_2, len);
}
#endif
