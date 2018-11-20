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
use log::{debug, info};
use nalgebra::{inf, sup, zero, Matrix4, Vector3};
use tobj;

use std::path::Path;

/// This is the visual representation of a gameobject.
///
/// Currently and can consist of seperate `Mesh`es which all are drawn with
/// on `Texture` as a color lookup table and `Shader`program.
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub shader: Shader,
    pub texture: Texture,
}

impl Model {
    /// Creates a new Model by passing a path to the obj file and a color palette name.
    ///
    /// The path to the obj file can be a relative or an absolut path as string.
    /// The palette file should be placed into "res/models" relative to the project or
    /// bin dir. The passed string for palette would look like this "car_green.png".
    pub fn new(file: &str, palette: &str) -> Model {
        info!("Model::new - file:{};palette:{}", file, palette);

        let file_str = format!("{}{}", "res/models/", file);
        let file = Path::new(&file_str);
        let obj = tobj::load_obj(file);

        let (models, _materials) = obj.unwrap();

        let mut meshes = Vec::with_capacity(models.len());
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

    /// This function draws the `Model`.
    ///
    /// Because the basic model has no translation, rotation or scale it needs the model-, view-,
    /// and projection matrix as parameter.
    pub fn draw(&self, model: &Matrix4<f32>, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        unsafe {
            self.shader.bind();
            Shader::bind_texture(0, &self.texture);
            Shader::set_uniform_mat4(0, model);
            Shader::set_uniform_mat4(1, view);
            Shader::set_uniform_mat4(2, projection);
            for mesh in &self.meshes {
                mesh.draw();
            }
        }
    }

    /// Get the minum and maximum x-, y-, and z-coordinates of all vertices in our model.
    ///
    /// This could be used to generate a bounding box. This is not an efficient function
    /// and does not manage later transformation of any mesh!
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
