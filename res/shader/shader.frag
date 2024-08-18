#version 450

#define INFINITY 1e10
#define MAX_NODES 1024
#define STACK_DEPTH 512

layout(location = 0) out vec4 outColor;

// Camera
uniform ivec2 screen_size;
uniform mat4x4 view_inverse;
uniform mat4x4 proj_inverse;

// Octree
uniform vec3 octree_origin = vec3(-0.5, -0.5, -0.5);
uniform float octree_size = 1;

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
    Node nodes[MAX_NODES];
};

struct Ray {
    vec3 pos;
    vec3 dir;
    vec3 dir_inv;
    vec3 color;
    float near_hit;
};

struct CastStack {
    uint node_index;
    vec3 min;
    float size;
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
    ray.near_hit = INFINITY;

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

Box CreateBox(vec3 min, float size) {
    return Box (min, min + size);
}

bool intersection(Box box, Ray ray, out float t) {
    // https://tavianator.com/2022/ray_box_boundary.html
    float tmax = INFINITY;
    float tmin = 0;

    for (int d = 0; d < 3; ++d) {
        float t1 = (box.min[d] - ray.pos[d]) * ray.dir_inv[d];
        float t2 = (box.max[d] - ray.pos[d]) * ray.dir_inv[d];

        tmin = max(tmin, min(t1, t2));
        tmax = min(tmax, max(t1, t2));
    }

    return tmin < tmax;
}

void hit(inout Ray ray) {
    // stack of nodes to visit
    uint stack_ptr = 0;
    CastStack stack[STACK_DEPTH];

    // push the root node
    stack[stack_ptr++] = CastStack(
        0, 
        octree_origin,
        octree_size
    );

    uint i = 0;
    while (stack_ptr > 0) {
        i++;
        if (i > 1000) {
            return;
        }

        if (stack_ptr > STACK_DEPTH) {
            return;
        }

        // fetch node from the stack
        CastStack item = stack[--stack_ptr];
        
        Node node = nodes[item.node_index];
        Box box = CreateBox(item.min, item.size);

        // check if the ray intersects the node
        float tmin;
        if (!intersection(box, ray, tmin)) {
            continue;
        }

        if (tmin > ray.near_hit) {
            continue;
        }

        // if a leaf node, update the color
        if (node.material != 0) {
            ray.color = vec3(1, 0, 0);
            ray.near_hit = tmin;
        }

        // if it has no children, continue
        if (node.child_start == 0) {
            continue;
        }

        // push children to the stack
        float child_size = item.size * 0.5;
        stack[stack_ptr++] = CastStack(node.child_start + 0, item.min + vec3(0, 0, 0) * child_size, child_size);
        stack[stack_ptr++] = CastStack(node.child_start + 1, item.min + vec3(1, 0, 0) * child_size, child_size);
        stack[stack_ptr++] = CastStack(node.child_start + 2, item.min + vec3(0, 1, 0) * child_size, child_size);
        stack[stack_ptr++] = CastStack(node.child_start + 3, item.min + vec3(1, 1, 0) * child_size, child_size);
        stack[stack_ptr++] = CastStack(node.child_start + 4, item.min + vec3(0, 0, 1) * child_size, child_size);
        stack[stack_ptr++] = CastStack(node.child_start + 5, item.min + vec3(1, 0, 1) * child_size, child_size);
        stack[stack_ptr++] = CastStack(node.child_start + 6, item.min + vec3(0, 1, 1) * child_size, child_size);
        stack[stack_ptr++] = CastStack(node.child_start + 7, item.min + vec3(1, 1, 1) * child_size, child_size);
    }

    if (ray.near_hit < INFINITY) {
        return;
    }

    ray.color = vec3(1, 1, 1) * i / 200.0;
}

void main() {
    Ray ray = CreateCameraRay();

    hit(ray);

    outColor = vec4(ray.color, 1.0);
}