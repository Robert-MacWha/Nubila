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
    // depending on the top bit of this field.
    // If the top bit is set, this is a leaf node and the data contains the material.
    // If the top bit is not set, this is an internal node and the data contains the start index
    uint data;
};

layout(std430) buffer Nodes {
    //? For whatever reason, defining this as a fixed size array causes the shader 
    //? to take *forever* to compile for larger MAX_NODES values.  I'm not sure why.
    //? Seems to be an ongoing issue. 
    Node nodes[];
};