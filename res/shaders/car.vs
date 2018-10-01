#version 330 core
#extension GL_ARB_explicit_uniform_location : enable
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec2 aUV;

layout (location = 0) out vec2 vUV;

layout (location = 0) uniform mat4 uMVP;

void main() {
    vUV = aUV;
    gl_Position = uMVP * vec4(aPosition, 1.);
}