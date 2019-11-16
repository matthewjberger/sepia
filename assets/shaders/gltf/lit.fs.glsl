#version 330 core

struct Material {
  sampler2D diffuse_texture;
  // sampler2D specular_texture;
  float shininess;
};

struct DirectionalLight {
  vec3 direction;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

struct PointLight {
  vec3 position;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float constant;
  float linear;
  float quadratic;
};

struct SpotLight {
  vec3 position;
  vec3 direction;
  float cutOff;
  float outerCutOff;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float constant;
  float linear;
  float quadratic;
};

in vec3 position;
in vec3 normal;
in vec2 texCoords;
out vec4 color;

uniform Material material;
uniform vec3 view_pos;
uniform DirectionalLight directional_light;
#define NUMBER_OF_POINT_LIGHTS 2
uniform PointLight point_lights[NUMBER_OF_POINT_LIGHTS];
uniform SpotLight spotlight;

vec3 calculate_directional_light(DirectionalLight light, vec3 normal, vec3 view_direction);
vec3 calculate_point_light(PointLight light, vec3 normal, vec3 frag_position, vec3 view_direction);
vec3 calculate_spot_light(SpotLight light, vec3 normal, vec3 frag_position, vec3 view_direction);

void main()
{
  vec3 norm = normalize(normal);
  vec3 view_dir = normalize(view_pos - position);

  vec3 result = calculate_directional_light(directional_light, norm, view_dir);
  for (int i = 0; i < NUMBER_OF_POINT_LIGHTS; i++)
    result += calculate_point_light(point_lights[i], norm, position, view_dir);
  result += calculate_spot_light(spotlight, norm, position, view_dir);

  color = vec4(result, 1.0);
}

vec3 calculate_directional_light(DirectionalLight light, vec3 normal, vec3 view_direction)
{
  vec3 light_dir = normalize(-light.direction);
  vec3 halfway_dir = normalize(light_dir + view_direction);

  // diffuse
  float diff = max(dot(normal, light_dir), 0.0);

  // specular
  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(normal, halfway_dir), 0.0), material.shininess);

  vec3 ambient = light.ambient * vec3(texture(material.diffuse_texture, texCoords));
  vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse_texture, texCoords));
  vec3 specular = light.specular * spec; // * vec3(texture(material.specular_texture, texCoords));

  return (ambient + diffuse + specular);
}


vec3 calculate_point_light(PointLight light, vec3 normal, vec3 frag_position, vec3 view_direction)
{
  vec3 light_dir = normalize(light.position - frag_position);
  vec3 halfway_dir = normalize(light_dir + view_direction);

  // diffuse
  float diff = max(dot(normal, light_dir), 0.0);

  // specular
  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(normal, halfway_dir), 0.0), material.shininess);

  // attenuation
  float distance = length(light.position.xyz - frag_position);
  float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

  vec3 ambient = light.ambient * vec3(texture(material.diffuse_texture, texCoords));
  vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse_texture, texCoords));
  vec3 specular = light.specular * spec; // * vec3(texture(material.specular_texture, texCoords));
  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  return (ambient + diffuse + specular);
}


vec3 calculate_spot_light(SpotLight light, vec3 normal, vec3 frag_position, vec3 view_direction)
{
  vec3 light_dir = normalize(light.position - frag_position);
  vec3 halfway_dir = normalize(light_dir + view_direction);

  // diffuse
  float diff = max(dot(normal, light_dir), 0.0);

  // specular
  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(normal, halfway_dir), 0.0), material.shininess);

  // attenuation
  float distance = length(light.position - frag_position);
  float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

  // spotlight
  float theta = dot(light_dir, normalize(-light.direction));
  float epsilon = light.cutOff - light.outerCutOff;
  float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

  vec3 ambient = light.ambient * vec3(texture(material.diffuse_texture, texCoords));
  vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse_texture, texCoords));
  vec3 specular = spec * light.specular; // * vec3(texture(material.specular_texture, texCoords));

  ambient *= attenuation * intensity;
  diffuse *= attenuation * intensity;
  specular *= attenuation * intensity;

  return (ambient + diffuse + specular);
}