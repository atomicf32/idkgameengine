#version 330

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 frag_pos;
out vec3 frag_normal;
out vec2 frag_tex_coords;

uniform mat4 camera_mat;
uniform mat4 model_mat;

void main() {
    gl_Position = camera_mat * model_mat * vec4(position, 1.0);
    
    frag_tex_coords = tex_coords;
    frag_normal = mat3(transpose(inverse(model_mat))) * normal;
    frag_pos = vec3(model_mat * vec4(position, 1.0));
}