#version 330 core
layout (location = 0) in vec3 v_position;
layout (location = 1) in vec3 v_normal;
layout (location = 2) in vec2 v_texCoords;

uniform mat4 mvp_matrix;

void main()
{
    gl_Position = mvp_matrix * vec4(v_position, 1.0);
}
