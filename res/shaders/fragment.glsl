#version 330

in vec3 frag_pos;
in vec3 frag_normal;
in vec2 frag_tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform vec4 light_pos;

void main() {
    float ambient = 0.1;

    vec3 norm = normalize(frag_normal);
    vec3 light_dir = normalize(light_pos.xyz - frag_pos);
    float diffuse = max(dot(norm, light_dir), 0.0);

    color = vec4(texture(tex, frag_tex_coords).xyz * (ambient + diffuse), 1.0);
}