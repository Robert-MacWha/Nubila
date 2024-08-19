#version 450

#define INFINITY 1e10
#define EPSILON 1e-6
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

bool ray_box_intersection(Box box, Ray ray, out float tmin, out float tmax) {
    // https://tavianator.com/2022/ray_box_boundary.html
    tmax = INFINITY;
    tmin = 0;

    for (int d = 0; d < 3; ++d) {
        float t1 = (box.min[d] - ray.pos[d]) * ray.dir_inv[d];
        float t2 = (box.max[d] - ray.pos[d]) * ray.dir_inv[d];

        tmin = min(max(t1, tmin), max(t2, tmin));
        tmax = max(min(t1, tmax), min(t2, tmax));
    }

    return tmin < tmax;
}

float ray_plane_intersection(vec3 rayOrigin, vec3 rayDir, vec3 normal) {
    float t = -dot(rayOrigin, normal) / dot(rayDir, normal);
    if (t < 0.0) {
        return INFINITY; // No hit
    }
    return t;
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

    uint tmp_stack_ptr = 0;
    CastStack tmp_stack[4];

    // push the root node
    stack[stack_ptr++] = CastStack(
        0, 
        octree_origin,
        octree_size
    );

    uint i = 0;
    uint max_stack = 0;
    uint color_updates = 0;
    while (stack_ptr > 0 && stack_ptr < STACK_DEPTH && i < 1000) {
        i++;
        max_stack = max(max_stack, stack_ptr);
        
        // fetch node from the stack
        CastStack item = stack[--stack_ptr];
        Node node = nodes[item.node_index];
        Box box = CreateBox(item.min, item.size);

        float tmin;
        float tmax;
        if (!ray_box_intersection(box, ray, tmin, tmax)) {
            // discard non-intersecting nodes
            continue;
        }

        if (node.material != 0) {
            // if a leaf node, update the color
            ray.color = color_from_material(node.material);
            return;
        }

        if (node.child_start == 0) {
            // if a empty leaf, continue
            continue;
        }

        // push the children to the stack with the closest on top
        float t = tmin;
        float child_size = item.size * 0.5;
        for (int i = 0; i < 3; i ++) {
            // calculate the id for the closest child
            vec3 ray_pos = at(ray, t);
            vec3 oriented_point = (ray_pos - item.min) * 2 / item.size; // 0 <= x < 2, with 0,0,0 being the min and 2,2,2 being the max
            uvec3 test = uvec3(floor(oriented_point));
            test = min(test, uvec3(1));
            
            uint index = test.x | test.y << 1 | test.z << 2;

            // get the far intersection point for the child and update t
            float tmin;
            float tmax;
            Box child_box = CreateBox(item.min + child_size * offset_lookup[index], child_size);
            if (!ray_box_intersection(child_box, ray, tmin, tmax)) {
                // non-intersection should never happen
                ray.color = vec3(0, 1, 0);
                return;
            }

            t = tmax + EPSILON;

            // push the child to the temp stack
            tmp_stack[tmp_stack_ptr++] = CastStack(
                node.child_start + index,
                item.min + child_size * offset_lookup[index],
                child_size
            );
        }

        // push the children to the stack in reverse order
        while (tmp_stack_ptr > 0) {
            stack[stack_ptr++] = tmp_stack[--tmp_stack_ptr];
        }
    }

    ray.color = vec3(1, 0, 0) * (float(i) / 50.0);
}

void main() {
    Ray ray = CreateCameraRay();

    hit(ray);

    outColor = vec4(ray.color, 1.0);
}