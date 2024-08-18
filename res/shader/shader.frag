#version 450

#define INFINITY 1e10

layout(location = 0) out vec4 outColor;

// Camera
uniform ivec2 screen_size;
uniform mat4x4 view_inverse;
uniform mat4x4 proj_inverse;

// Octree
uniform vec3 octree_origin;
uniform float octree_size;

struct Node {
    // todo: Think about using the first bit as a flag for leaf node. Then you could
    // store the material in the same uint as the children.
    // 
    // 0: leaf node, otherwise internal node. If an internal node, the first child 
    // is at child_start and the other children are at child_start + 1, child_start + 2, etc.
    // nodes[0]: x0 y0 z0
    // nodes[1]: x1 y0 z0
    // nodes[2]: x0 y1 z0
    // nodes[3]: x1 y1 z0
    // nodes[4]: x0 y0 z1
    // nodes[5]: x1 y0 z1
    // nodes[6]: x0 y1 z1
    // nodes[7]: x1 y1 z1
    uint child_start;

    uint material;
};

layout(std430) buffer Nodes {
    Node nodes[];
};

struct Ray {
    vec3 pos;
    vec3 dir;
    vec3 dir_inv;
    vec3 color;
};

struct Box {
    vec3 min;
    vec3 max;
};

Ray CreateRay(vec3 pos, vec3 dir)
{
    Ray ray;
    ray.pos = pos;
    ray.dir = dir;
    ray.dir_inv = 1.0 / ray.dir;
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

Box CreateBox(vec3 origin, float size) {
    return Box (origin - size, origin + size);
}

bool intersection(Box box, Ray ray) {
    // https://tavianator.com/2022/ray_box_boundary.html
    float tmin = 0;
    float tmax = INFINITY;

    for (int d = 0; d < 3; ++d) {
        float t1 = (box.min[d] - ray.pos[d]) * ray.dir_inv[d];
        float t2 = (box.max[d] - ray.pos[d]) * ray.dir_inv[d];

        tmin = max(tmin, min(t1, t2));
        tmax = min(tmax, max(t1, t2));
    }

    return tmin < tmax;
}

void hit(inout Ray ray) {
    if (intersection(CreateBox(vec3(0, 0, 1), 0.1), ray)) {
        ray.color = vec3(1, 0, 0);
        return;
    }

    ray.color = vec3(ray.dir.y, ray.dir.y, (ray.dir.y + 0.3) * 4);
}

void main() {
    Ray ray = CreateCameraRay();

    hit(ray);

    outColor = vec4(ray.color, 1.0);
}