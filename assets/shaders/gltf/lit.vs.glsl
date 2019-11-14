#version 330 core
layout (location = 0) in vec3 v_position;
layout (location = 1) in vec3 v_normal;
layout (location = 2) in vec2 v_texCoords;
layout (location = 3) in vec4 a_joint;
layout (location = 4) in vec4 a_weight;

uniform mat4 u_jointMatrix[2];

out vec3 position;
out vec3 normal;
out vec2 texCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    mat4 skinMatrix =
      a_weight.x * u_jointMatrix[int(a_joint.x)] +
      a_weight.y * u_jointMatrix[int(a_joint.y)] +
      a_weight.z * u_jointMatrix[int(a_joint.z)] +
      a_weight.w * u_jointMatrix[int(a_joint.w)];

    texCoords = v_texCoords;

    // TODO: compute this on the cpu and pass it along
    normal = mat3(transpose(inverse(model))) * v_normal;

    gl_Position = projection * view * model * skinMatrix * vec4(v_position, 1.0);
}
