uniform vec3 octree_origin;
uniform float octree_size;

layout(std430) buffer OctreeBuffer {
    // 0-23: child pointer (leaf node if zero)
    // 24-31: child mask
    uint Octree[];
};

struct Attribute {
    uint rgb;
};

layout (std430) buffer AttributeBuffer {
    Attribute Attributes[];
};