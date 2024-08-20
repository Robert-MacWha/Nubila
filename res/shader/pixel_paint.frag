#version 450
#extension GL_ARB_shading_language_include : require

// Pixel Paint manages the second rendering pass - painting in pixel color 
// values with the result of the voxel raycasts. This shader reads from the 
// voxel_map texture, which contains the voxel IDs of the first voxel hit by
// each pixel raycast. It then reads the render_data buffer to get the color
// of the voxel, and paints the pixel with that color.

#include "common/define.glsl"

layout(location = 0) out vec4 outColor;

layout(std430, binding=0) buffer NodeRender {
    // render_data contains the render data for this node.  The top bit is the render 
    // flag, set if the node was seen in the first fragment pass.  The bottom 24 
    // bits are the render color.
    uint render_data[MAX_NODES];
};

uniform usampler2D voxel_map;

uniform uvec2 screen_size;

uvec4 u32_to_u8x4(uint value) {
    return uvec4(
        (value >> 24) & 0xff,
        (value >> 16) & 0xff,
        (value >> 8) & 0xff,
        value & 0xff
    );
}

uint u8x4_to_u32(uvec4 value) {
    return  (uint(value.r) << 24) | 
            (uint(value.g) << 16) | 
            (uint(value.b) << 8) | 
            uint(value.a);
}

void main() {
    vec2 uv = (gl_FragCoord.xy / screen_size);
    uvec4 tex_color = texture(voxel_map, uv);
    
    uint node_id = u8x4_to_u32(tex_color);
    if (node_id == 0) {
        outColor = vec4(0.4, 0.5, 0.9, 1.0);
        return;
    }

    uvec4 data = u32_to_u8x4(render_data[node_id]);
    outColor = vec4(data.xyz / 255.0, 1.0);
}