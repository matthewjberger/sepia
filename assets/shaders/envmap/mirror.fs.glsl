#version 330 core

out vec4 color;

in vec3 normal;
in vec3 position;

uniform vec3 camera_pos;
uniform samplerCube skybox;

void main()
{
  vec3 I = normalize(position - camera_pos);
  vec3 R = reflect(I, normalize(normal));
  color = vec4(texture(skybox, R).rgb, 1.0);
}
