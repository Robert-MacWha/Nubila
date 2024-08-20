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

void push(inout uint stack_ptr, inout uint64_t stack, uint value) {
    stack |= uint64_t(value) << (stack_ptr * 4);
    stack_ptr++;
}

uint pop(inout uint stack_ptr, inout uint64_t stack) {
    stack_ptr--;
    uint val = uint((stack >> (stack_ptr * 4)) & 0xf);
    stack &= ~(uint64_t(0xf) << (stack_ptr * 4));
    return val;
}