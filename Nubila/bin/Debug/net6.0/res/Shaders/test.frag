#version 430 core
out vec4 FragColor;
in vec2 pos;

void main() 
{
	FragColor = vec4(pos.xy * 0.5 + 0.5, 0, 1);
}