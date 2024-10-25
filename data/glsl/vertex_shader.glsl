#version 330 core
in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;
uniform vec3 u_light_direction;

out vec3 v_normal;
out vec2 v_tex_coords;
out vec3 v_light_direction;

void main() {
    gl_Position = perspective * view * model * vec4(position, 1.0);
    
    v_tex_coords = tex_coords;

    v_normal = mat3(transpose(inverse(model))) * normal;
    v_light_direction = normalize(u_light_direction);
}