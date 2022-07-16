#version 140
in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform vec3 transform;
uniform mat4 pers_mat;
uniform mat4 mod_matrix;
uniform mat4 view_matrix;
out vec3 v_normal;
out vec2 v_tex_coords;

void main() {
    v_normal = transpose(inverse(mat3(mod_matrix))) * normal;
    gl_Position = pers_mat * view_matrix * mod_matrix * vec4(position + transform, 1.0);
}