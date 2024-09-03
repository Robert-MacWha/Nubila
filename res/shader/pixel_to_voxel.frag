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

    uint parent;
    float t;
    vec3 debug = vec3(0);
    bool hit = intersect_octree(ray, parent, t, debug);

    if (!hit) {
        outColor = uvec4(200, 200, 255, 1);
        return;
    }

    Attribute voxel = Attributes[parent];
    vec3 voxel_color = vec3(u32_to_u8x4(voxel.rgb).yzw);
    outColor = uvec4(voxel_color, 1);
    return;
}