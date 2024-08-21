#version 450
#extension GL_ARB_gpu_shader_int64 : enable
#extension GL_ARB_shading_language_include : require

#include "common/define.glsl"
#include "common/bitwise.glsl"
#include "common/node_buffer.glsl"

// Pixel Paint manages the second rendering pass - painting in pixel color 
// values with the result of the voxel raycasts. This shader reads from the 
// voxel_map texture, which contains the voxel IDs of the first voxel hit by
// each pixel raycast. It then reads the render_data buffer to get the color
// of the voxel, and paints the pixel with that color.

layout(location = 0) out vec4 outColor;

uniform uvec2 screen_size;
uniform usampler2D voxel_map;

void main() {
    vec2 uv = (gl_FragCoord.xy / screen_size);
    uvec4 tex_color = texture(voxel_map, uv);
    
    uint node_id = u8x4_to_u32(tex_color);
    if (node_id == 0) {
        outColor = vec4(0.4, 0.5, 0.9, 1.0);
        return;
    }

    uvec4 node_data = u32_to_u8x4(node_buffer[node_id]);
    outColor = vec4(node_data.xyz / 255.0, 1.0);
}