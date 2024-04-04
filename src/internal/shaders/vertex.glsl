#version 330

in vec3 position;
in vec2 tex_coords;
out vec2 frag_tex_coords;

uniform mat4 matrix;

void main() {
    gl_Position = matrix * vec4(position, 1.0);
    frag_tex_coords = tex_coords;
}