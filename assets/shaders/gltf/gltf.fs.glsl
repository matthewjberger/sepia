#version 330 core

in vec2 texCoords;
out vec4 color;

uniform sampler2D texture_diffuse1;
uniform vec4 base_color;

void main()
{
  vec4 texture_color = texture(texture_diffuse1, texCoords);
  // if(texture_color.a < 0.5) {
  //   discard;
  // }
  color = vec4(texture_color);
  // color = base_color;
  // color = texture(texture_diffuse1, texCoords);
}
