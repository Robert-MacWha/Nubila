#version 450

layout(location = 0) out vec4 outColor;

uniform ivec2 screen_size;
uniform mat4x4 view_inverse;
uniform mat4x4 proj_inverse;

float focal_length = 1;
float viewport_height = 2;

struct Ray {
    vec3 pos;
    vec3 dir;
    vec3 color;
};

Ray CreateRay(vec3 pos, vec3 dir)
{
    Ray ray;
    ray.pos = pos;
    ray.dir = dir;
    ray.color = vec3(0, 0, 0);

    return ray;
}

Ray CreateCameraRay() {
    vec2 uv = (gl_FragCoord.xy / screen_size) * 2 - 1;

    vec3 pos = (view_inverse * vec4(0, 0, 0, 1)).xyz;
    vec3 dir = (proj_inverse * vec4(uv, 0, 1)).xyz;
    dir = (view_inverse * vec4(dir, 0)).xyz;
    dir = normalize(dir);

    return CreateRay(pos, dir);
}

vec3 at(Ray ray, float t) {
    return ray.pos + t * ray.dir;
}

bool hit_sphere(vec3 center, float radius, inout Ray ray) {
    vec3 oc = center - ray.pos;
    float a = dot(ray.dir, ray.dir);
    float b = -2 * dot(ray.dir, oc);
    float c = dot(oc, oc) - radius * radius;
    float discriminant = b * b - 4 * a * c;

    if (discriminant < 0) {
        return false;
    }

    float t = (-b - sqrt(discriminant)) / (2 * a);
    if (t < 0) {
        return false;
    }

    vec3 n = normalize(at(ray, t) - vec3(0, 0, -1));
    ray.color = (n + 1) * 0.5;
    return true;
}

void hit(inout Ray ray) {
    if (hit_sphere(vec3(0, 0, 2), 0.5, ray)) {
        return;
    }

    vec3 unit_dir = normalize(ray.dir);
    float a = 0.5*(unit_dir.y + 1.0);
    ray.color = (1.0-a)*vec3(1.0, 1.0, 1.0) + a*vec3(0.5, 0.7, 1.0);
}

void main() {
    Ray ray = CreateCameraRay();

    hit(ray);

    outColor = vec4(ray.color, 1.0);
}