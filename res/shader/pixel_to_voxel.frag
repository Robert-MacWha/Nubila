#version 450
#extension GL_ARB_gpu_shader_int64 : enable
#extension GL_ARB_shading_language_include : require

#include "common/define.glsl"
#include "common/bitwise.glsl"
#include "common/raycast.glsl"
#include "common/node_buffer.glsl"

// Pixel to Voxel manages the first rendering pass - determining which voxels
// corespond to which pixels.  The shader casts rays from each pixel, colours
// in the output texture with the voxel ID of the first voxel hit, and flags
// the voxel as seen in the render_data buffer.

layout(location = 0) out uvec4 outColor;

// Camera
uniform uvec2 screen_size;
uniform mat4x4 view_inverse;
uniform mat4x4 proj_inverse;

void main() {
    Ray ray = CreateCameraRay(gl_FragCoord.xy, screen_size, view_inverse, proj_inverse);

    uint skips = 0;
    uint node_id = intersect_octree(ray, skips);
    if (node_id > MAX_NODES || node_id == 0) {
        outColor = uvec4(0, 0, 0, 0);
        return;
    }

    // get the hit node's color
    Node node = nodes[node_id];
    uvec3 node_color = u32_to_u8x4(node.data).yzw;
    
    // pack render_data
    uint data = u8x4_to_u32(uvec4(node_color, 1));
    node_buffer[node_id] = data;

    // assign the pixel to the encoded hit_id ID
    uvec4 encoded = u32_to_u8x4(node_id);
    outColor = encoded;
}