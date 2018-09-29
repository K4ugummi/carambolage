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
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.
use super::mesh::{Mesh, Vertex};
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
            Vertex { position: [-0.49211, 1., 0.], },
            Vertex { position: [0.49211, 1., 0.], },

            Vertex { position: [-0.66146, 0.33333, 0.], },
            Vertex { position: [-0.49053, 0.33333, 0.], },
            Vertex { position: [0.49053, 0.33333, 0.], },
            Vertex { position: [0.66146, 0.33333, 0.], },

            Vertex { position: [-0.66146, -0.33333, 0.], },
            Vertex { position: [-0.49053, -0.33333, 0.], },
            Vertex { position: [0.49053, -0.33333, 0.], },
            Vertex { position: [0.66146, -0.33333, 0.], },

            Vertex { position: [-0.66146, -1., 0.], },
            Vertex { position: [0.66146, -1., 0.], },
        ];
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let indices = vec![
            0u32, 2, 3,
            0, 3, 1,
            3, 4, 1,
            1, 4, 5,
            3, 7, 4,
            7, 8, 4,
            6, 10, 7,
            7, 10, 11,
            7, 11, 8,
            8, 11, 9,
        ];

        let meshes = vec![Mesh::new(vertices, indices)];

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
