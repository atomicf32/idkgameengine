#version 330 core

out vec4 color;

in vec4 fragcolor;

void main() {
    color = vec4(fragcolor.xyz, 1.0);
}