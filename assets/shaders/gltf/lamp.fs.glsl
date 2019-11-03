#version 330 core

uniform vec3 lamp_color;

out vec4 color;

void main()
{
  color = vec4(lamp_color, 1.0);
}
