#version 450
#extension GL_ARB_gpu_shader_int64 : enable
#extension GL_ARB_shading_language_include : require

#include "common/define.glsl"
#include "common/bitwise.glsl"
#include "common/raycast.glsl"
#include "common/node_buffer.glsl"

layout(location = 0) out vec4 outColor;

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
    bool hit = raymarch(ray_pos, ray_dir, 0, parent, t, path, debug);

    if (!hit) {
        outColor = vec4(0.3, 0.4, 0.7, 1);
        return;
    }

    uint encoded_rgb = Attributes[parent].rgb;
    uvec3 base_rgb = u32_to_u8x4(encoded_rgb).yzw;
    outColor = vec4(base_rgb / 255.0, 1);
    return;
}