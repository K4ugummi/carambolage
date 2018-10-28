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
use grphx::{Mesh, Shader, Texture, Vertex};

use nalgebra::{inf, sup, zero, Matrix4, Vector3};
use tobj;

use std::path::Path;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub shader: Shader,
    pub texture: Texture,
}

impl Model {
    pub fn new(path: &str, palette: &str) -> Model {
        let path_str = format!("{}{}", "res/models/", path);
        debug!("New from {} with palette {}", path_str, palette);

        let path = Path::new(&path_str);
        let obj = tobj::load_obj(path);

        let (models, materials) = obj.unwrap();
        debug!("{} meshes and {} materials", models.len(), materials.len());

        let mut meshes = Vec::new();
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // data to fill
            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position: [p[i * 3], p[i * 3 + 1], p[i * 3 + 2]],
                    normal: [n[i * 3], n[i * 3 + 1], n[i * 3 + 2]],
                    uv: [t[i * 2], t[i * 2 + 1]],
                })
            }

            meshes.push(Mesh::new(vertices, indices));
        }

        let shader = Shader::new("default");

        let texture = Texture::new(palette);

        Model { meshes, shader, texture }
    }

    pub fn draw(&self, model: &Matrix4<f32>, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        unsafe {
            self.shader.bind();
            self.shader.bind_texture(0, &self.texture);
            self.shader.set_uniform_mat(0, model);
            self.shader.set_uniform_mat(1, view);
            self.shader.set_uniform_mat(2, projection);
            for mesh in &self.meshes {
                mesh.draw();
            }
        }
    }

    pub fn get_min_max(&self) -> (Vector3<f32>, Vector3<f32>) {
        let mut min = zero();
        let mut max = zero();

        for mesh in &self.meshes {
            for vert in &mesh.vertices {
                min = inf(&min, &vert.position.into());
                max = sup(&max, &vert.position.into());
            }
        }
        debug!("(min, max) = ({}, {})", min, max);
        (min, max)
    }
}
