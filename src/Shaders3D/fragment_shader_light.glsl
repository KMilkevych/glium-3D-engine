#version 150

in vec3 v_normal;
in vec3 v_position;
in vec2 v_texture;

out vec4 color;

const vec3 v_color = vec3(1.0, 1.0, 1.0);

void main() {
    color = vec4(v_color, 1.0);
}