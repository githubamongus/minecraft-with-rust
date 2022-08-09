#version 460 core

in vec2 UV;
in vec3 FragPos;
in vec3 Normal;
out vec4 OutColor;

uniform sampler2D tex;

vec3 lightPos = vec3(0.0, 1000.0, 0.0);

void main() {
    float ambientStrength = 0.4f;
    vec3 ambient = ambientStrength * vec3(1.0, 1.0, 1.0);

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);

    float diff = max(dot(norm, lightDir), 0.0);

    vec4 diffuse = vec4(ambient + diff, 1.0) * texture(tex, UV);

    if (diffuse.a <= 0.1) {
        discard;
    }

    OutColor = diffuse;
}