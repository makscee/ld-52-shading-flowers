#include <common.glsl>

uniform float u_radius;

#ifdef VERTEX_SHADER
uniform float u_padding = 0.0;
uniform vec2 u_position;

out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;

void main() {
    v_quad_pos = a_pos * (1.0 + u_padding);
    vec2 pos = v_quad_pos * u_radius + u_position;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.0);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
const float THICKNESS = 0.05;

in vec2 v_quad_pos;

void main() {
    float len = length(v_quad_pos);
    if(len > 1.0) {
        discard;
    }
    vec4 color = u_color_2;
    if(len < 1.0 - THICKNESS / u_radius) {
        color.a = 0.1;
    } else {
        if(fract((vecAngle(v_quad_pos) + PI * 2 + u_time * .2) / PI * 8) < 0.5) {
            discard;
        }
    }
    gl_FragColor = color;
}
#endif
