layout(std430, binding = 0) buffer NodeBuffer {
    // voxel_buffer contains the render data for each voxel. The top bit is the render 
    // flag, set if the voxel was seen in the first fragment pass. The bottom 24 
    // bits are the rendered color.
    uvec2 node_buffer[];
};