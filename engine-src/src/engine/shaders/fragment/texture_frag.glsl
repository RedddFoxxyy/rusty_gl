#version 330 core

in vec2 TexCoords;

uniform vec4 color;
uniform sampler2D image;

out vec4 fragColor;

void main() {
    fragColor = texture(image, TexCoords) * color;
}
