#version 330 core

in vec3 position;

out vec4 fragcolor;

uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;

void main() {
    gl_Position = proj * view * model * vec4(position, 1.0);
    fragcolor = gl_Position;
}