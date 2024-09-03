struct Ray {
    vec3 pos;
    vec3 dir;
    vec3 dir_inv;
};

Ray CreateRay(vec3 pos, vec3 dir)
{
    Ray ray;
    ray.pos = pos;
    ray.dir = dir;
    ray.dir_inv = 1.0 / ray.dir;

    return ray;
}

Ray CreateCameraRay(vec2 frag_coord, vec2 screen_size, mat4x4 view_inverse, mat4x4 proj_inverse) {
    vec2 screen_space = (frag_coord / screen_size) * 2 - 1;
    vec3 pos = (view_inverse * vec4(0, 0, 0, 1)).xyz;
    vec3 dir = (proj_inverse * vec4(screen_space, 0, 1)).xyz;
    dir = (view_inverse * vec4(dir, 0)).xyz;
    dir = normalize(dir);

    return CreateRay(pos, dir);
}

vec3 at(Ray ray, float t) {
    return ray.pos + t * ray.dir;
}

// advance returns the ray's position advanced by t units
vec3 advance(Ray ray, float t) {
    return ray.pos + t * ray.dir;
}