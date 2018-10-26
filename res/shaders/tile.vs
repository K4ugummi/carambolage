// This file is part of Carambolage.

// Carambolage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Carambolage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Carambolage.  If not, see <http://www.gnu.org/licenses/>.
#version 330 core
#extension GL_ARB_explicit_uniform_location : enable
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUV;
layout (location = 3) in mat4 aInstanceMatrix;

layout (location = 0) out vec2 vUV;
layout (location = 1) out vec3 vFragPos;
layout (location = 2) out vec3 vNormal;

layout (location = 0) uniform mat4 uView;
layout (location = 1) uniform mat4 uProjection;

void main() {
    mat4 modelViewProj = uProjection * uView * aInstanceMatrix;
    vUV = aUV;
    vFragPos = vec3(aInstanceMatrix * vec4(aPosition, 1.0));
    vNormal = normalize(vec3(transpose(inverse(aInstanceMatrix * uView)) * vec4(aNormal, 0.)));
    gl_Position = modelViewProj * vec4(aPosition, 1.);
}