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

layout (location = 0) in vec2 vUV;
layout (location = 1) in vec3 vFragPos;
layout (location = 2) in vec3 vNormal;

out vec4 FragColor;

layout (location = 5) uniform sampler2D uTexture;

#define GAMMA_CORRECTION 1.1

void main() {
    vec3 normal = normalize(vNormal);
    vec3 lightColor = vec3(1., 1., 1.);
    vec3 lightDir = normalize(vec3(1., 2., 3.));

    // Ambient lighting
    float ambientStrength = 0.2;
    vec3 ambient = ambientStrength * lightColor;

    // Diffuse lighting
    float diff = max(dot(vNormal, lightDir) * 0.9, 0.0);
    vec3 diffuse = diff * lightColor;

    // Specular lighting will come soon, maybe :D
    // I like the comic style
    FragColor = vec4((ambient + diffuse) * texture(uTexture, vUV).rgb, 1.0);
    FragColor.rgb = pow(FragColor.rgb, vec3(1.0/GAMMA_CORRECTION));
}