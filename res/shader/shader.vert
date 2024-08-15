#version 450

out vec4 vertColor;

void main() {
    vec2 positions[3] = vec2[3](
        vec2(0.0,  0.5),  // Top vertex
        vec2(-0.5, -0.5),  // Bottom-left vertex
        vec2(0.5, -0.5)  // Bottom-right vertex
    );

    vec3 colors[3] = vec3[3](
        vec3(1.0, 0.0, 0.0),  // Red
        vec3(0.0, 1.0, 0.0),  // Green
        vec3(0.0, 0.0, 1.0)  // Blue
    );

    vertColor = vec4(colors[gl_VertexIndex], 1.0);
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
}