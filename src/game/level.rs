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
use crate::grphx::Model;
use log::debug;
use nalgebra::{zero, Isometry3, Matrix4, Point3, Vector3};
use ncollide3d::shape::{Cuboid, TriMesh};
use std::path::Path;

/// Environment of a `Scene`.
///
/// Currently our Environment consist of a race track with some colliders.
pub struct Level {
    /// Racetrack model
    model: Model,
    /// Identity matrix for model rendering.
    matrix: Matrix4<f32>,
    /// A simple box collider for the ground.
    pub(super) ground: (Isometry3<f32>, Cuboid<f32>),
    /// Racetrack border collider. Keep this mesh as simple as possible.
    pub(super) border: (Isometry3<f32>, TriMesh<f32>),
}

impl Level {
    /// Load a model from raw model files.
    pub fn new(file: &str) -> Level {
        debug!("New from {}", file);
        let model = Model::new(file, "racetrack.png");

        // No scaliing, rotating, translating (Just for render)
        let matrix = Matrix4::identity();

        let (col_ground, col_border) = Self::load_collider(file);
        let ground = (Isometry3::new(Vector3::new(0., 0., -100.0), zero()), col_ground);
        let border = (Isometry3::new(zero(), zero()), col_border);

        Level {
            model,
            matrix,
            ground,
            border,
        }
    }

    /// Load the collider mesh from an obj file.
    fn load_collider(file: &str) -> (Cuboid<f32>, TriMesh<f32>) {
        let path_str = format!("res/models/{}.col", file);
        let path = Path::new(&path_str);
        let obj = tobj::load_obj(path);
        let (models, _) = obj.unwrap();

        let mesh = &models[0].mesh;
        let num_vertices = mesh.positions.len() / 3;
        let num_indices = mesh.indices.len() / 3;

        let mut vertices = Vec::with_capacity(num_vertices);
        let mut indices = Vec::with_capacity(mesh.indices.len());
        for i in 0..num_indices {
            indices.push(Point3::new(
                mesh.indices[i * 3] as usize,
                mesh.indices[i * 3 + 1] as usize,
                mesh.indices[i * 3 + 2] as usize,
            ));
        }

        let p = &mesh.positions;
        for i in 0..num_vertices {
            vertices.push(Point3::new(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]));
        }

        let col_ground = Cuboid::new(Vector3::new(1_000.0, 1_000.0, 100.0));
        let col_border = TriMesh::new(vertices, indices, None);

        (col_ground, col_border)
    }

    /// Render the environment to the bound framebuffer.
    pub fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        self.model.draw(&self.matrix, view, projection);
    }
}
