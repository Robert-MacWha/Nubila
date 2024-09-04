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
    
    vec3 ray_pos;
    vec3 ray_dir;
    ray_to_octree(ray, ray_pos, ray_dir);

    uint parent;
    float t;
    uint64_t path;
    vec3 debug;
    bool hit = raymarch(ray_pos, ray_dir, 0, 0, parent, t, path, debug);

    if (!hit) {
        outColor = uvec4(0, 0, 0, 0);
        return;
    }

    // store the node that was hit and the path to it
    outColor = u32_to_u8x4(parent);
    node_buffer[parent] = u64_to_uvec2(path);  
    return;
}