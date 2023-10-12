#version 330 core
layout (location = 0) in vec3 aPos;

out vec2 TexCoords;

void main() {
    TexCoords = vec2(min(0.0, aPos.x), min(0.0, aPos.y));
    gl_Position = vec4(aPos, 1.0);
}