struct Box {
    vec3 min;
    vec3 max;
};

Box CreateBox(vec3 min, float size) {
    return Box (min, min + size);
}