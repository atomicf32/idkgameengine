#version 330 core

out vec4 color;

in vec4 fragcolor;

void main() {
    color = vec4(0.0, 0.0, fragcolor.z, 1.0);
}