#include "common/box.glsl"
#include "common/ray.glsl"
#include "common/octree.glsl"

bool ray_box_intersection(Box box, Ray ray, out float tmin, out float tmax) {
    // https://tavianator.com/2022/ray_box_boundary.html
    tmin = 0;
    tmax = INFINITY;

    for (int d = 0; d < 3; ++d) {
        float t1 = (box.min[d] - ray.pos[d]) * ray.dir_inv[d];
        float t2 = (box.max[d] - ray.pos[d]) * ray.dir_inv[d];

        tmin = min(max(t1, tmin), max(t2, tmin));
        tmax = max(min(t1, tmax), min(t2, tmax));
    }

    return tmin < tmax;
}

bool ray_in_box(Box box, vec3 point) {
    return all(lessThanEqual(box.min, point)) && all(lessThanEqual(point, box.max));
}

//* Rendering
// interesct_octree returns the index of the node that the ray intersects with
// and advances the ray's position to the intersection point.
uint intersect_octree(inout Ray ray) {
    vec3 origin = octree_origin;
    float size = octree_size;

    // advance the ray to the first intersection point
    Box box = CreateBox(origin, size);
    float tmin, tmax;
    if (!ray_box_intersection(box, ray, tmin, tmax)) {
        return 0;
    }

    ray.pos = advance(ray, tmin + EPSILON);

    // march the ray
    uint64_t rel_child_stack;
    uint stack_ptr = 0;

    uint current_node = 0;
    int i = 0;
    for (i = 0; i < MAX_STEPS; i ++) {
        Node current = nodes[current_node];

        // if this node is a leaf, return the index
        bool is_leaf = (current.data & 0x80000000) != 0;
        if (is_leaf) {
            return current_node;
        }

        // if the ray is past the current node, ascend to the parent
        box = CreateBox(origin, size);
    
        if (current.data == 0) {
            float tmin, tmax;
            ray_box_intersection(box, ray, tmin, tmax);
            ray.pos = advance(ray, tmax + EPSILON);

            // ascend to the parent 
            current_node = current.parent;
            uint rel_child_index = pop(stack_ptr, rel_child_stack);
            origin -= offset_lookup[rel_child_index] * size;
            size *= 2;
            continue;
        }

        if (!ray_in_box(box, ray.pos)) {
            // if the ray is past the root node, return 0
            if (current_node == 0) {
                break;
            }

            // ascend to the parent 
            current_node = current.parent;
            uint rel_child_index = pop(stack_ptr, rel_child_stack);
            origin -= offset_lookup[rel_child_index] * size;
            size *= 2;
            continue;
        }

        //* this node has children
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
        push(stack_ptr, rel_child_stack, rel_child_index);
    }

    return 0;
}