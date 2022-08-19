#version 460 core

in vec2 UV;
out vec4 OutColor;

uniform sampler2D tex;

void main() {
    vec4 diffuse = texture(tex, UV);

    if (diffuse.a <= 0.1) {
        discard;
    }

    OutColor = diffuse;
}