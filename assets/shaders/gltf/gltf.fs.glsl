#version 330 core

in vec2 texCoords;
out vec4 color;

uniform sampler2D texture_diffuse1;
uniform vec4 base_color;

void main()
{
  color = vec4(texture(texture_diffuse1, texCoords));
}
