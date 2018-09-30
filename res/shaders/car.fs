#version 330
#extension GL_ARB_separate_shader_objects : enable

in vec2 vUV;

uniform sampler2D uTexture;

void main() {
    gl_FragColor = texture(uTexture, vUV).rgba;
}