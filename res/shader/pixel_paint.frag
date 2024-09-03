#version 450
#extension GL_ARB_gpu_shader_int64 : enable
#extension GL_ARB_shading_language_include : require

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
    
    outColor = tex_color / 255.0;
}