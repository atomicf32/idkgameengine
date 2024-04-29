#version 330

in vec2 frag_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, frag_tex_coords);
}