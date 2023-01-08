#include <common.glsl>

uniform float u_mut_1 = 0.1;
uniform float u_mut_2 = 0.9;
uniform float u_mut_3 = 0.3;
uniform float u_mut_4 = 0.4;
uniform float u_mut_5 = 0.9;

#ifdef VERTEX_SHADER
uniform float u_size;
uniform float u_radius;
uniform float u_padding = 0.0;

uniform vec2 u_position;

out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
flat out int p_index;
flat out float p_t;

void main() {
    float t = 1 - u_spawn;
    p_index = gl_InstanceID;
    v_quad_pos = a_pos * (1.0 + u_padding);
    p_t = rand(p_index + 4) + u_time;
    vec2 p = randCircle(p_index) * rand(p_index + 1);
    vec2 v = vec2(sin(u_time * (u_mut_2 * 3.) * rand(p_index + 5)), cos(u_time * (u_mut_1 * 15. + 0.1 * rand(p_index + int(u_mut_4 * 4))))) * .6 * cos(u_time * (0.6 + u_mut_5));
    v += rand(p_index + int(u_mut_5 * 5)) * vec2(0, 1 + sin(u_time * 1.3 * u_mut_1)) * .5;
    vec2 pos = v + p + v_quad_pos * (u_size * sin(p_t * PI * 2 * (0.8 + u_mut_2))) + u_position;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.0);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 v_quad_pos;
flat in float p_t;
flat in int p_index;

void main() {
    float len = length(v_quad_pos);
    if(length(v_quad_pos) > 1.0) {
        discard;
    }
    vec4 color = mix(u_color_1, u_color_2, fract(rand(p_index + int(u_mut_3 * 4)) + u_time * .3));
    gl_FragColor = vec4(color.rgb, 1.3 - len);
}
#endif
