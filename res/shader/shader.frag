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

// Lookup table for the order of children to visit based on the ray direction.
// Optimized to visit closer children first.
const uint order_lookup[8][8] = {
    // +x +y +z
    {0, 1, 2, 3, 4, 5, 6, 7}, 
    // -x +y +z
    {1, 0, 3, 2, 5, 4, 7, 6}, 
    // +x -y +z
    {2, 3, 0, 1, 6, 7, 4, 5}, 
    // -x -y +z
    {3, 2, 1, 0, 7, 6, 5, 4}, 
    // +x +y -z
    {4, 5, 6, 7, 0, 1, 2, 3}, 
    // -x +y -z
    {5, 4, 7, 6, 1, 0, 3, 2}, 
    // +x -y -z
    {6, 7, 4, 5, 2, 3, 0, 1}, 
    // -x -y -z
    {7, 6, 5, 4, 3, 2, 1, 0}  
};

const vec3 offset_lookup[8] = {
    vec3(0, 0, 0),
    vec3(1, 0, 0),
    vec3(0, 1, 0),
    vec3(1, 1, 0),
    vec3(0, 0, 1),
    vec3(1, 0, 1),
    vec3(0, 1, 1),
    vec3(1, 1, 1)
};

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

    // material is packed with the rgb color components.  r<<16 | g<<8 | b
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

bool intersection(Box box, Ray ray, out float tmin) {
    // https://tavianator.com/2022/ray_box_boundary.html
    float tmax = INFINITY;
    tmin = 0;

    for (int d = 0; d < 3; ++d) {
        float t1 = (box.min[d] - ray.pos[d]) * ray.dir_inv[d];
        float t2 = (box.max[d] - ray.pos[d]) * ray.dir_inv[d];

        tmin = min(max(t1, tmin), max(t2, tmin));
        tmax = max(min(t1, tmax), min(t2, tmax));
    }

    return tmin < tmax;
}

int signbit(float x) {
    return int(floatBitsToInt(x) >> 31) & 1;
}

vec3 color_from_material(uint material) {
    return vec3(
        (material >> 16) & 0xff,
        (material >> 8) & 0xff,
        material & 0xff
    ) / 255.0;
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
    uint max_stack = 0;
    uint color_updates = 0;
    while (stack_ptr > 0) {
        max_stack = max(max_stack, stack_ptr);
        i++;
        if (i > 1000) {
            return;
        }

        if (stack_ptr > STACK_DEPTH) {
            ray.color = vec3(1, 0.5, 1);
            return;
        }

        // fetch node from the stack
        CastStack item = stack[--stack_ptr];
        Node node = nodes[item.node_index];
        Box box = CreateBox(item.min, item.size);

        float tmin;
        if (!intersection(box, ray, tmin)) {
            // discard missed nodes
            continue;
        }

        if (tmin >= ray.near_hit) {
            continue;
        }

        if (node.material != 0) {
            // if a leaf node, update the color
            ray.color = color_from_material(node.material);
            ray.near_hit = tmin;
            color_updates += 1;
        }

        if (node.child_start == 0) {
            // if it has no children, continue
            continue;
        }

        // push non-empty children to the stack
        // int sign_x = signbit(ray.dir.x);
        // int sign_y = signbit(ray.dir.y);
        // int sign_z = signbit(ray.dir.z);
        // int index = sign_x << 2 | sign_y << 1 | sign_z;
        // const uint order[8] = order_lookup[index];

        float child_size = item.size * 0.5;
        for (uint j = 0; j < 8; j++) {
            uint i = j;
            uint child_index = node.child_start + i;
            Node child_node = nodes[child_index];

            // only push non-empty nodes
            if (child_node.material == 0 && child_node.child_start == 0) {
                continue;
            }

            vec3 offset = offset_lookup[i];
            stack[stack_ptr++] = CastStack(child_index, item.min + offset * child_size, child_size);
        }
    }

    ray.color = vec3(1, 1, 1) * (float(color_updates) / 2.0);
}

void main() {
    Ray ray = CreateCameraRay();

    hit(ray);

    outColor = vec4(ray.color, 1.0);
}