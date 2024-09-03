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
bool intersect_octree(Ray ray, out uint parent, out float t, out vec3 debug) {
    debug = vec3(1, 0, 0);

    float ray_scale = 0;
    float hit_t = 0;
    vec3 hit_pos = vec3(0);
    int hit_idx = 0;
    int hit_scale = 0;

    const int max_scale = 23; // Maximum scale (number of float mantissa bits).
    const float epsilon = exp2(-max_scale);

    ivec2 stack[max_scale + 1]; // parent voxel stack

    // Remove small directions from the ray
    vec3 ray_pos = (ray.pos - octree_origin) / octree_size + vec3(1);
    vec3 ray_dir = normalize(ray.dir);
    if (abs(ray_dir.x) < epsilon) ray_dir.x = epsilon;
    if (abs(ray_dir.y) < epsilon) ray_dir.y = epsilon;
    if (abs(ray_dir.z) < epsilon) ray_dir.z = epsilon;

    // pre-compute ray traversal coeffcients
    float tx_coef = 1.0 / -abs(ray_dir.x);
    float ty_coef = 1.0 / -abs(ray_dir.y);
    float tz_coef = 1.0 / -abs(ray_dir.z);

    float tx_bias = tx_coef * ray_pos.x;
    float ty_bias = ty_coef * ray_pos.y;
    float tz_bias = tz_coef * ray_pos.z;
    
    // select octant mask to mirror the coordinate system.
    int octant_mask = 7;
    if (ray_dir.x > 0) { octant_mask ^= 1; tx_bias = 3.0 * tx_coef - tx_bias; }
    if (ray_dir.y > 0) { octant_mask ^= 2; ty_bias = 3.0 * ty_coef - ty_bias; }
    if (ray_dir.z > 0) { octant_mask ^= 4; tz_bias = 3.0 * tz_coef - tz_bias; }

    // initialize the active span of t-values
    float t_min = max(max(2.0 * tx_coef - tx_bias, 2.0 * ty_coef - ty_bias), 2.0 * tz_coef - tz_bias);
    float t_max = min(min(      tx_coef - tx_bias,       ty_coef - ty_bias),       tz_coef - tz_bias);
    t_min = max(t_min, 0.0);

    // initialize the current voxel to the first child of the root node
    uint current = 0;
    parent = 0;
    uint idx = 0;
    vec3 pos = vec3(1, 1, 1);
    int scale = max_scale - 1;
    float scale_exp2 = 0.5;

    // determin the first child voxel
    if (1.5 * tx_coef - tx_bias > t_min) { idx ^= 1; pos.x = 1.5; }
    if (1.5 * ty_coef - ty_bias > t_min) { idx ^= 2; pos.y = 1.5; }
    if (1.5 * tz_coef - tz_bias > t_min) { idx ^= 4; pos.z = 1.5; }

    // traverse the octree
    int i = 0;
    while (scale < max_scale && i < MAX_STEPS) {
        i += 1;
        debug.x = i;

        // fetch child descriptor if invalid
        if (current == 0) {
            current = Octree[parent];
        }

        // Terminate if the voxel is a leaf.
        if ((current & 0x00FFFFFF) == 0) {
            break;
        }

        // Determine maximum t-value of the cube by evaluating tx(), ty(), and tz() at its corner.
        float tx_corner = pos.x * tx_coef - tx_bias;
        float ty_corner = pos.y * ty_coef - ty_bias;
        float tz_corner = pos.z * tz_coef - tz_bias;
        float tc_max = min(min(tx_corner, ty_corner), tz_corner);

        // Check if the corresponding bit in the valid mask is set and the active t-span is non-empty.
        uint child_shift = idx ^ octant_mask; // permute child slots based on mirroring of the coordinate system.
        // Extract the valid mask (bits 24-31)
        uint child_masks = ((current >> 24) & 0xFF);

        if (((child_masks & (1 << child_shift)) != 0) && (t_min <= t_max)) { // ? check top bit
            // Terminate if the voxel is small enough.
            if (tc_max * ray_scale >= scale_exp2) {
                return true;
            }

            // INTERSECT
            // Intersect active t-span with the cube and evaluate
            // tx(), ty(), and tz() at the center of the voxel.

            float tv_max = min(t_max, tc_max);
            float half_scale = scale_exp2 * 0.5;
            float tx_center = half_scale * tx_coef + tx_corner;
            float ty_center = half_scale * ty_coef + ty_corner;
            float tz_center = half_scale * tz_coef + tz_corner;

            // Descend to the first child if the resulting t-span is non-empty.
            if (t_min <= tv_max) {
                uint child_offset = current & 0x00FFFFFF; // get bits 0-23

                // PUSH
                // Write current parent to the stack.
                stack[scale] = ivec2(parent, floatBitsToInt(t_max));

                // Find the child descriptor for the current voxel.
                uint mask = (1u << child_shift) - 1u;
                uint masked_child_mask = child_masks & mask;
                uint preceeding_children = bitCount(masked_child_mask);
                uint p = parent;
                parent += child_offset;
                parent += preceeding_children;

                idx = 0;
                scale -= 1;
                scale_exp2 = half_scale;

                if (tx_center > t_min) {idx ^= 1; pos.x += scale_exp2; }
                if (ty_center > t_min) {idx ^= 2; pos.y += scale_exp2; }
                if (tz_center > t_min) {idx ^= 4; pos.z += scale_exp2; }

                t_max = tv_max;
                current = 0;

                continue;
            }
        }

        // ADVANCE
        // Step along the ray
        int step_mask = 0;
        if (tx_corner <= tc_max) { step_mask ^= 1; pos.x -= scale_exp2; }
        if (ty_corner <= tc_max) { step_mask ^= 2; pos.y -= scale_exp2; }
        if (tz_corner <= tc_max) { step_mask ^= 4; pos.z -= scale_exp2; }

        // Update active t-span and flip bits of the child slot index
        t_min = tc_max;
        idx ^= step_mask;

        // Proceed with pop if the bit flips disagree with the ray direction.
        if ((idx & step_mask) != 0) {
            // POP
            // find the highest different bit between the two positions

            int differing_bits = 0;
            if ((step_mask & 1) != 0) { differing_bits |= floatBitsToInt(pos.x) ^ floatBitsToInt(pos.x + scale_exp2); }
            if ((step_mask & 2) != 0) { differing_bits |= floatBitsToInt(pos.y) ^ floatBitsToInt(pos.y + scale_exp2); }
            if ((step_mask & 4) != 0) { differing_bits |= floatBitsToInt(pos.z) ^ floatBitsToInt(pos.z + scale_exp2); }
            scale = (floatBitsToInt(float(differing_bits)) >> 23) - 127;
            scale_exp2 = uintBitsToFloat((scale - max_scale + 127) << 23);

            // Restore parent voxel from the stack.
            ivec2 stack_entry = stack[scale];
            parent = stack_entry.x;
            t_max = intBitsToFloat(stack_entry.y);

            // Round cube position and extract child slot
            int shx = floatBitsToInt(pos.x) >> scale;
            int shy = floatBitsToInt(pos.y) >> scale;
            int shz = floatBitsToInt(pos.z) >> scale;
            pos.x = intBitsToFloat(shx << scale);
            pos.y = intBitsToFloat(shy << scale);
            pos.z = intBitsToFloat(shz << scale);
            idx = (shx & 1) | ((shy & 1) << 1) | ((shz & 1) << 2);
            
            // prevent the same parent from being stored again and invalidating cached child descriptors.
            current = 0;
            continue;
        }
    }

    // Return
    if (scale >= max_scale) {
        return false;
    }

    t = t_min;
    return true;
}