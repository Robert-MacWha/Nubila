#version 430 core
layout (location = 0) in vec2 aPosition;
out vec2 pos;    

void main() 
{
    gl_Position = vec4(aPosition.xy, 0, 1.0);
    pos = vec2(aPosition.xy);
}