#version 450

in vec4 vertColor;

// Output color
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vertColor;
}
