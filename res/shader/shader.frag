#version 450
#extension GL_ARB_gpu_shader_int64 : enable

#define INFINITY 1e10
#define EPSILON 1e-3
#define MAX_NODES 8092
#define MAX_STEPS 1024

layout(location = 0) out vec4 outColor;

// Camera
uniform ivec2 screen_size;
uniform mat4x4 view_inverse;
uniform mat4x4 proj_inverse;

// Octree
uniform vec3 octree_origin;
uniform float octree_size;

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
    // parent is the index of this node's parent
    uint parent;

    // data contains either the material of the voxel or the start index of the children
    // depending on the first bit of this field.
    // If the first bit is set, this is a leaf node and the data contains the material.
    // If the first bit is not set, this is an internal node and the data contains the start index
    uint data;
};

layout(std430) buffer Nodes {
    Node nodes[MAX_NODES];
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

//* Ray
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

// advance returns the ray's position advanced by t units
vec3 advance(Ray ray, float t) {
    return ray.pos + t * ray.dir;
}

//* Box
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

bool box_contains(Box box, vec3 point) {
    return all(lessThanEqual(box.min, point)) && all(lessThanEqual(point, box.max));
}

//* Bitwise hacks
vec3 color_from_material(uint material) {
    return vec3(
        (material >> 16) & 0xff,
        (material >> 8) & 0xff,
        material & 0xff
    ) / 255.0;
}

void push(inout uint stack_ptr, inout uint64_t stack, uint value) {
    stack |= uint64_t(value) << (stack_ptr * 4);
    stack_ptr++;
}

uint pop(inout uint stack_ptr, inout uint64_t stack) {
    stack_ptr--;
    uint val = uint((stack >> (stack_ptr * 4)) & 0xf);
    stack &= ~(uint64_t(0xf) << (stack_ptr * 4));
    return val;
}

// interesct_octree returns the index of the node that the ray intersects with
// and advances the ray's position to the intersection point.
uint intersect_octree(inout Ray ray) {
    uint64_t rel_child_stack;
    uint stack_ptr = 0;

    uint current_node = 0;
    vec3 origin = octree_origin;
    float size = octree_size;
    int i = 0;
    for (i = 0; i < MAX_STEPS; i ++) {
        Node current = nodes[current_node];

        ray.color = vec3(float(current_node) / 10.0);

        // if this node is a leaf, return the index
        if ((current.data & 0x80000000) != 0) {
            ray.color = color_from_material(current.data);
            return current_node;
        }

        // if the ray is past the current node, ascend to the parent
        float tmin, tmax;
        Box box = CreateBox(origin, size);

        bool ascend = !ray_box_intersection(box, ray, tmin, tmax) || current.data == 0;
        if (ascend) {
            // if the ray is past the root node, return 0
            if (current_node == 0) {
                ray.color = vec3(1, 0, 1);
                break;
            }
            
            float t = EPSILON;
            ray.pos = advance(ray, tmax + t);
            current_node = current.parent;

            // undo the change made to origin when decending to the child
            // uint rel_child_index = rel_child_stack[--stack_ptr];
            uint rel_child_index = pop(stack_ptr, rel_child_stack);
            origin -= offset_lookup[rel_child_index] * size;
            size *= 2;
            continue;
        }

        //* this node has children
        // advance the ray to the intersection point with this node
        
        // select the correct child node
        vec3 oriented_pos = (ray.pos - origin) * 2 / size;
        ivec3 test_pos = ivec3(floor(oriented_pos));
        test_pos = clamp(test_pos, ivec3(0), ivec3(1));

        // decend to the child 
        uint rel_child_index = test_pos.x | (test_pos.y << 1) | (test_pos.z << 2);
        uint child_index = current.data + rel_child_index;
        current_node = child_index;
        size /= 2;
        origin += offset_lookup[rel_child_index] * size;

        // store the rel child index for later
        // rel_child_stack[stack_ptr++] = rel_child_index;
        push(stack_ptr, rel_child_stack, rel_child_index);
    }

    ray.color = vec3(i) / (MAX_STEPS / 2);
    return 0;
}

void main() {
    Ray ray = CreateCameraRay();

    uint intersection = intersect_octree(ray);

    outColor = vec4(ray.color, 1.0);
}