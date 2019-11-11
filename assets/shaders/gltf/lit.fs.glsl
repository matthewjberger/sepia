#version 330 core

in vec3 position;
in vec3 normal;
in vec2 texCoords;
out vec4 color;

uniform vec3 view_pos;

struct Light {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

struct Material {
  sampler2D diffuse_texture;
  // sampler2D specular_texture;  
  float shininess;
};

uniform Material material;
uniform Light light;

void main()
{
  // ambient
  vec3 ambient = light.ambient * texture(material.diffuse_texture, texCoords).rgb;

  // diffuse
  vec3 norm = normalize(normal);
  vec3 light_dir = normalize(light.position - position);
  float diff = max(dot(norm, light_dir), 0.0);
  vec3 diffuse = light.diffuse * diff * texture(material.diffuse_texture, texCoords).rgb;

  // specular
  vec3 view_dir = normalize(view_pos - position);
  vec3 reflect_dir = reflect(-light_dir, norm);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);
  vec3 specular = spec * light.specular; // * texture(material.specular_texture, texCoords).rgb;

  vec3 result = ambient + diffuse + specular;
  color = vec4(result, 1.0);
}
