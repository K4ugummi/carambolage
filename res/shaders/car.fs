#version 330 core
#extension GL_ARB_explicit_uniform_location : enable
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec2 vUV;

layout (location = 5) uniform sampler2D uTexture;

void main() {
    gl_FragColor = texture(uTexture, vUV).rgba;
}