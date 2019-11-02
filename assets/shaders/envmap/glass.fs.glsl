#version 330 core

out vec4 color;

in vec3 normal;
in vec3 position;

uniform vec3 camera_pos;
uniform samplerCube skybox;

void main()
{
  vec3 I = normalize(position - camera_pos);
  float refractive_index = 1.00 / 1.52;
  vec3 R = refract(I, normalize(normal), refractive_index);
  color = vec4(texture(skybox, R).rgb, 1.0);
}
