#version 330
#extension GL_ARB_separate_shader_objects : enable

in vec3 aPosition;
in vec2 aUV;

out vec2 vUV;

uniform mat4 uMVP;

void main() {
    vUV = aUV;
    gl_Position = uMVP * vec4(aPosition, 1.);
}