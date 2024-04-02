#version 330 core

in vec3 position;

out vec4 fragcolor;

uniform mat4 matrix;

void main() {
    gl_Position = matrix * vec4(position, 1.0);
    fragcolor = gl_Position;
}