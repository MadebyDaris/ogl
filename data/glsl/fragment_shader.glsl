#version 330 core

in vec2 v_tex_coords;           // Texture coordinates from vertex shader
in vec3 v_normal;               // Normal from vertex shader
in vec3 v_light_direction;      // Light direction from vertex shader

uniform sampler2D tex;          // Texture sampler
uniform vec3 u_light_color;     // Light color

out vec4 frag_color;            // Output color

void main() {
    // Basic ambient light
    vec3 ambient = 0.1 * u_light_color; // Ambient light contribution

    // Calculate the diffuse component
    vec3 norm = normalize(v_normal);     // Normalize the normal
    float diff = max(dot(norm, -v_light_direction), 0.0); // Diffuse lighting factor (light direction inverted)
    vec3 diffuse = diff * u_light_color; // Calculate diffuse color

    vec4 tex_color = texture(tex, v_tex_coords); // Sample the texture
    vec3 result = (ambient + diffuse) * tex_color.rgb; // Combine lighting with texture color

    frag_color = vec4(result, tex_color.a); // Final color output
}
