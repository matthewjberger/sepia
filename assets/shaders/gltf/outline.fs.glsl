#version 330 core

out vec4 color;

uniform vec3 highlight;

void main()
{
    color = vec4(highlight, 1.0);
}