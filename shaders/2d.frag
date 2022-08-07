#version 460 core

in vec2 UV;
out vec4 OutColor;

uniform sampler2D tex;

void main() {
    OutColor = texture(tex, UV);
}