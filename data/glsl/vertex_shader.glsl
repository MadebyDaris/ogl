#version 140
in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 pers_mat;
uniform mat4 matrix;
uniform mat4 view_matrix;
out vec3 v_normal;
out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = pers_mat * view_matrix * matrix * vec4(position, 1.0);
}