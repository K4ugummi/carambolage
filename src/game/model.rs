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
use super::mesh::{Mesh, Texture, Vertex};
use super::shader::Shader;
use nalgebra::Matrix4;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub shader: Shader,
}

impl Model {
    pub fn new() -> Model {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices = vec![
            Vertex { pos: [2., 2., 0.,], uv: [1., 0.,] },
            Vertex { pos: [-2., 2., 0.,], uv: [0., 0.,] },
            Vertex { pos: [-2., -2., 0.,], uv: [0., 1.,] },
            Vertex { pos: [2., -2., 0.,], uv: [1., 1.,] },
        ];
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let indices = vec![
            0u32, 1, 2,
            0, 2, 3,
        ];

        let textures = unsafe {
            static mut CHOOSER: usize = 0;
            let result = match CHOOSER {
                0 => vec![Texture::new("res/textures/Ambulance.png")],
                1 => vec![Texture::new("res/textures/Audi.png")],
                2 => vec![Texture::new("res/textures/Black_viper.png")],
                3 => vec![Texture::new("res/textures/Car.png")],
                4 => vec![Texture::new("res/textures/Mini_truck.png")],
                5 => vec![Texture::new("res/textures/Mini_van.png")],
                6 => vec![Texture::new("res/textures/Police.png")],
                _ => vec![Texture::new("res/textures/taxi.png")],
            };
            CHOOSER += 1;
            result
        };

        let meshes = vec![Mesh::new(vertices, indices, textures)];

        let shader = Shader::new();

        Model { meshes, shader }
    }

    pub fn draw(&self, mvp: &Matrix4<f32>) {
        unsafe {
            self.shader.bind();
            self.shader.set_uniform_mat(&"uMVP", &mvp);
            for mesh in &self.meshes {
                mesh.draw(&self.shader);
            }
        }
    }
}
