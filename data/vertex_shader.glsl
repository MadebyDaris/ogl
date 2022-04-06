#version 140
in vec3 position;
in vec3 normal;

uniform mat4 pers_mat;
uniform mat4 matrix;
uniform mat4 rot_mat;
out vec3 v_normal;
void main() {
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = rot_mat* matrix * vec4(position, 1.0);
}