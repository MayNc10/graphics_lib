#version 330 core

out vec4 color;

in vec2 TexCoords;

uniform sampler2D gColorEmission;

void main() {
    color = vec4(texture(gColorEmission, TexCoords).rgb, 1.0);
}
