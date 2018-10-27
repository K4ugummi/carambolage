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

out vec4 FragColor;

layout (location = 0) uniform int postProcessingEffect;
layout (location = 5) uniform sampler2D screen;

const float offset = 1.0 / 1000.0; 

vec4 use_kernel(float kernel[9]) {
    vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right    
    );

    vec3 sampleTex[9];
    for(int i = 0; i < 9; i++) {
        sampleTex[i] = vec3(texture(screen, vUV.st + offsets[i]));
    }
    vec3 color = vec3(0.0);
    for(int i = 0; i < 9; i++) {
        color += sampleTex[i] * kernel[i];
    }
    return vec4(color, 1.0);
}

void main() {
    if (postProcessingEffect == 1) {
        FragColor = texture(screen, vUV);
    }
    else if (postProcessingEffect == 2) {
        FragColor = vec4(vec3(1.0 - texture(screen, vUV)), 1.0);
    }
    else if (postProcessingEffect == 3) {
        FragColor = texture(screen, vUV);
        float average = 0.2126 * FragColor.r + 0.7152 * FragColor.g + 0.0722 * FragColor.b;
        FragColor = vec4(average, average, average, 1.0);
    }
    else if (postProcessingEffect == 4) {
        float kernel[9] = float[](
            -1, -1, -1,
            -1,  9, -1,
            -1, -1, -1
        );
        
        FragColor = use_kernel(kernel);
    }
    else if (postProcessingEffect == 5) {
        float kernel[9] = float[](
            1.0 / 16, 2.0 / 16, 1.0 / 16,
            2.0 / 16, 4.0 / 16, 2.0 / 16,
            1.0 / 16, 2.0 / 16, 1.0 / 16  
        );
        FragColor = use_kernel(kernel);
    }
    else if (postProcessingEffect == 6) {
        float kernel[9] = float[](
            1.0, 1.0, 1.0, 
            1.0, -8.0, 1.0,
            1.0, 1.0, 1.0
        );
        FragColor = use_kernel(kernel);
    }
    else if (postProcessingEffect == 7) {
        float kernel[9] = float[](
            1.0, 1.0, 1.0, 
            1.0, -8.0, 1.0,
            1.0, 1.0, 1.0
        );
        vec4 kernelColor = use_kernel(kernel);
        float color = kernelColor.r + kernelColor.g + kernelColor.b;
        if (color > 0.98) {
            FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
        else {
            FragColor = texture(screen, vUV);
        }
    }
    else {
        FragColor = texture(screen, vUV);
    }
}