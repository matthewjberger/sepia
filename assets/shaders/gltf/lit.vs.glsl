#version 330 core
layout (location = 0) in vec3 v_position;
layout (location = 1) in vec3 v_normal;
layout (location = 2) in vec2 v_texCoords;

out vec3 position;
out vec3 normal;
out vec2 texCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    texCoords = v_texCoords;
    position = vec3(model * vec4(v_position, 1.0));

    // TODO: compute this on the cpu and pass it along
    normal = mat3(transpose(inverse(model))) * v_normal;

    gl_Position = projection * view * vec4(position, 1.0);
}
