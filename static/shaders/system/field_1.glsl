#ifdef VERTEX_SHADER
out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;

void main() {
    v_quad_pos = a_pos * 100.0;
    vec2 pos = v_quad_pos;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.0);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 v_quad_pos;

void main() {
    vec3 color = vec3(0.24f, 0.29f, 0.58f) * (1.0 - smoothstep(0.0, 20.0, length(v_quad_pos)));
    gl_FragColor = vec4(color, 1.f);
}
#endif
