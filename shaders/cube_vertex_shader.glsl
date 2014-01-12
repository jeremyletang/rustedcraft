#version 330 core

layout(location = 0) in vec3 vertexPosition_modelspace;
layout(location = 1) in vec2 vertexUV;

out vec2 texcoord;
uniform mat4 mvp;

void main(void) {
    gl_Position = mvp * vec4(vertexPosition_modelspace, 1.0);
    texcoord = vertexUV;
}