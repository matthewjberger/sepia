#version 330 core

in vec3 position;
in vec3 normal;
in vec2 texCoords;

out vec4 color;

uniform sampler2D texture_diffuse1;
uniform vec4 base_color;

uniform vec3 light_pos;
uniform vec3 view_pos;
uniform vec3 light_color;

void main()
{
  // ambient
  float ambient_strength = 0.1;
  vec3 ambient = ambient_strength * light_color;

  // diffuse
  vec3 norm = normalize(normal);
  vec3 light_dir = normalize(light_pos - position);
  float diff = max(dot(norm, light_dir), 0.0);
  vec3 diffuse = diff * light_color;

  // specular
  float specular_strength = 0.5;
  vec3 view_dir = normalize(view_pos - position);
  vec3 reflect_dir = reflect(-light_dir, norm);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
  vec3 specular = specular_strength * spec * light_color;

  vec4 result = vec4(ambient + diffuse + specular, 1.0) * base_color;

  color = vec4(texture(texture_diffuse1, texCoords)) * result;
}
