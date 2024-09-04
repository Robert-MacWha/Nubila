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

uvec2 u64_to_uvec2(uint64_t value) {
    uint low = uint(value & 0xFFFFFFFFu);  // Lower 32 bits
    uint high = uint((value >> 32) & 0xFFFFFFFFu);  // Upper 32 bits
    return uvec2(low, high);
}

uint64_t uvec2_to_u64(uvec2 value) {
    return uint64_t(value.y) << 32 | uint64_t(value.x);
}