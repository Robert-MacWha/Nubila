#version 450

layout(location = 0) out vec4 outColor;

struct Ray {
    vec3 pos;
    vec3 dir;
};

uniform ivec2 screenSize;

void main() {
    vec2 pos = (gl_FragCoord.xy / screenSize) * 2 - 1;

    Ray ray = Ray(vec3(pos, 0), vec3(0, 0, 1));

    outColor = vec4(ray.pos, 1.0);
}