#version 450

layout(location = 0) out vec4 outColor;

layout(std430) buffer floatBuffer {
    float data[6];
};

uniform ivec2 screenSize;

void main() {
    float r = data[0];
    float g = data[1];
    float b = data[2];
    
    outColor = vec4(r, g, b, 1.0);
}