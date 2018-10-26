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
use super::gl;
use super::mesh::{Mesh, Vertex};
use super::shader::Shader;
use super::texture::Texture;
use nalgebra::{Matrix4, Vector4};

use std::mem::size_of;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg_attr(feature = "cargo-clippy", allow(enum_variant_names))]
#[derive(Copy, Clone, Debug)]
pub(super) enum TileType {
    TerrainGrass           = 0,
    TrackCheckpoint        = 1,
    TrackCornerLarge       = 2,
    TrackCornerSmall       = 3,
    TrackLeftRightLarge    = 4,
    TrackPitEntryLaneSmall = 5,
    TrackPitEntryLarge     = 6,
    TrackPitExitLaneSmall  = 7,
    TrackPitExitLarge      = 8,
    TrackRightLeftLarge    = 9,
    TrackStartFinish       = 10,
    TrackStraightSmall     = 11,
    TrackTire              = 12,
}

pub(super) struct Tile {
    is_used: bool,
    meshes: Vec<Mesh>,
    pub matrices: Vec<Matrix4<f32>>,
    instance_buffer: u32,
    texture: Texture,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        debug!("New {:?}", tile_type);
        let mut tile: Tile = Default::default();

        let path_str = format!(
            "{}{}",
            "res/models/",
            match tile_type {
                TileType::TerrainGrass => "terrain-grass.obj",
                TileType::TrackCheckpoint => "track-checkpoint.obj",
                TileType::TrackCornerLarge => "track-corner-large.obj",
                TileType::TrackCornerSmall => "track-corner-small.obj",
                TileType::TrackLeftRightLarge => "track-left-right-large.obj",
                TileType::TrackPitEntryLaneSmall => "track-pit-entrylane-small.obj",
                TileType::TrackPitEntryLarge => "track-pit-entry-large.obj",
                TileType::TrackPitExitLaneSmall => "track-pit-exitlane-small.obj",
                TileType::TrackPitExitLarge => "track-pit-exit-large.obj",
                TileType::TrackRightLeftLarge => "track-right-left-large.obj",
                TileType::TrackStartFinish => "track-start-finish.obj",
                TileType::TrackStraightSmall => "track-straight-small.obj",
                TileType::TrackTire => "track-tire.obj",
            }
        );

        debug!("Loading model: {}", path_str);
        let path = Path::new(&path_str);
        let obj = tobj::load_obj(path);

        let (models, _materials) = obj.unwrap();

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

            tile.meshes.push(Mesh::new(vertices, indices));
        }

        tile.texture = Texture::new("racetrack.png");

        tile
    }

    pub fn init_instance_buffer(&mut self) {
        let size_mat4 = size_of::<Matrix4<f32>>() as i32;
        let size_vec4 = size_of::<Vector4<f32>>() as i32;
        if self.matrices.len() != 0 {
            self.is_used = true;

            unsafe {
                gl::GenBuffers(1, &mut self.instance_buffer);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_buffer);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (self.matrices.len() * size_mat4 as usize) as isize,
                    &self.matrices[0] as *const Matrix4<f32> as *const c_void,
                    gl::STATIC_DRAW,
                );

                for mesh in self.meshes.iter_mut() {
                    gl::BindVertexArray(mesh.vao);

                    gl::EnableVertexAttribArray(3);
                    gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, size_mat4, ptr::null());
                    gl::EnableVertexAttribArray(4);
                    gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, size_mat4, size_vec4 as *const c_void);
                    gl::EnableVertexAttribArray(5);
                    gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, size_mat4, (2 * size_vec4) as *const c_void);
                    gl::EnableVertexAttribArray(6);
                    gl::VertexAttribPointer(6, 4, gl::FLOAT, gl::FALSE, size_mat4, (3 * size_vec4) as *const c_void);

                    gl::VertexAttribDivisor(3, 1);
                    gl::VertexAttribDivisor(4, 1);
                    gl::VertexAttribDivisor(5, 1);
                    gl::VertexAttribDivisor(6, 1);

                    gl::BindVertexArray(0);
                }
            }
        }
    }

    pub fn draw(&self, shader: &Shader) {
        if self.is_used {
            unsafe {
                shader.bind_texture(0, &self.texture);

                for mesh in &self.meshes {
                    gl::BindVertexArray(mesh.vao);
                    gl::DrawElementsInstanced(
                        gl::TRIANGLES,
                        mesh.indices.len() as i32,
                        gl::UNSIGNED_INT,
                        ptr::null(),
                        self.matrices.len() as i32,
                    );
                    gl::BindVertexArray(0);
                }
            }
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            is_used: false,
            meshes: Vec::new(),
            matrices: Vec::new(),
            instance_buffer: 0,
            texture: Default::default(),
        }
    }
}

impl Drop for Tile {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.instance_buffer as *const u32);
        }
    }
}
