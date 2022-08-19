#version 460 core

out vec4 OutColor;
in vec2 UV;

uniform sampler2D tex;

void main() {
    OutColor = texture(tex, UV);
}
