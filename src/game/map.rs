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

// #####################################################################
// DO NOT CLEAN THIS MESS OR TRY TO MERGE IT WITH MODEL OR MESH (please)
// #####################################################################
#![macro_use]

use super::image;
use super::image::DynamicImage::*;
use super::image::GenericImageView;
use super::shader::Shader;
use super::texture::Texture;

use super::gl;
use nalgebra::{Matrix4, Vector3, Vector4};

use std::mem::size_of;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    };
}

#[repr(C)]
struct TileVertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
}

impl Default for TileVertex {
    fn default() -> TileVertex {
        TileVertex {
            pos: [0., 0.],
            uv: [0., 0.],
        }
    }
}

struct Tile {
    vertices: Vec<TileVertex>,
    indices: Vec<u32>,
    vao: u32,
    vbo: u32,
    ibo: u32,
}

impl Tile {
    pub fn new() -> Tile {
        let mut tile: Tile = Default::default();
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices = vec![
            TileVertex { pos: [10., 10.,], uv: [1., 0.,] },
            TileVertex { pos: [-10., 10.,], uv: [0., 0.,] },
            TileVertex { pos: [-10., -10.,], uv: [0., 1.,] },
            TileVertex { pos: [10., -10.,], uv: [1., 1.,] },
        ];
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let indices = vec![
            0u32, 1, 2,
            0, 2, 3,
        ];

        unsafe {
            tile.init(vertices, indices);
        }

        tile
    }

    unsafe fn init(&mut self, vertices: Vec<TileVertex>, indices: Vec<u32>) {
        self.vertices = vertices;
        self.indices = indices;

        // VAO
        gl::GenVertexArrays(1, &mut self.vao);
        gl::BindVertexArray(self.vao);

        // VBO
        gl::GenBuffers(1, &mut self.vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        let size = (self.vertices.len() * size_of::<TileVertex>()) as isize;
        let data = &self.vertices[0] as *const TileVertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // IBO
        gl::GenBuffers(1, &mut self.ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let size = size_of::<TileVertex>() as i32;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(TileVertex, pos) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(TileVertex, uv) as *const c_void,
        );

        gl::BindVertexArray(0);
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            vertices: Vec::new(),
            indices: Vec::new(),
            vao: 0,
            vbo: 0,
            ibo: 0,
        }
    }
}

impl Drop for Tile {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.ibo as *const u32);
            gl::DeleteBuffers(1, self.vbo as *const u32);
            gl::DeleteVertexArrays(1, self.vao as *const u32);
        }
    }
}

pub struct Map {
    tile: Tile,
    pub shader: Shader,
    tile_matrices: Vec<Matrix4<f32>>,
    tile_buffer: u32,
    tiles_num: usize,
    textures: Vec<Texture>,
}

impl Map {
    pub fn new(file: &str) -> Map {
        // Load the map file.
        let img =
            image::open(&Path::new(file)).expect("ERROR: Map failed to load");
        match img {
            ImageRgba8(_) => {}
            _ => panic!("ERROR: Map must be in RGBA8 format!"),
        };
        let tiles_num_x = img.width() as usize;
        let tiles_num_y = img.height() as usize;
        let tiles_num = tiles_num_x * tiles_num_y;
        let data = img.raw_pixels();

        // Create a unique tile for each sprite.
        let tile = Tile::new();
        let textures = vec![Texture::new("res/maps/tiles/testing.png")];

        // Load the tile shader.
        let shader = Shader::new("tile");

        // Create a stride in x,y-direction to place the tiles.
        let stride_x = Vector3::new(20f32, 0., 0.);
        let stride_y = Vector3::new(0f32, 20., 0.);
        let tile_start = Vector3::new(
            -(img.width() as f32 / 2.) * stride_x[0],
            -(img.width() as f32 / 2.) * stride_y[1],
            -0.01,
        );

        // Generate translation matrices for each tile.
        // Note: This could be done with Vector3, because right now,
        // were just translating, but who knows what else will come.
        let mut tile_matrices: Vec<Matrix4<f32>> =
            Vec::with_capacity(tiles_num);
        for y in 0..tiles_num_y {
            for x in 0..tiles_num_x {
                let matrix = Matrix4::new_translation(
                    &(tile_start
                        + stride_x * (x as f32)
                        + stride_y * (y as f32)),
                );
                tile_matrices.push(matrix);
            }
        }
        println!("INFO: First matrix: {}", tile_matrices.first().unwrap());
        println!("INFO: Last matrix: {}", tile_matrices.last().unwrap());

        // Yeah.. and the following magic doesn't work right now and I might
        // be stupid or overseeing something.
        let size_mat4 = size_of::<Matrix4<f32>>() as i32;
        let size_vec4 = size_of::<Vector4<f32>>() as i32;
        println!("INFO: Size of Matrix4: {} bytes.", size_mat4);
        println!("INFO: Size of Vector4: {} bytes.", size_vec4);

        let mut tile_buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut tile_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, tile_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (tiles_num * size_mat4 as usize) as isize,
                &tile_matrices[0] as *const Matrix4<f32> as *const c_void,
                gl::STATIC_DRAW,
            );
        }

        let vao = tile.vao;
        unsafe {
            gl::BindVertexArray(vao);

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_mat4,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_mat4,
                size_vec4 as *const c_void,
            );
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(
                4,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_mat4,
                (2 * size_vec4) as *const c_void,
            );
            gl::EnableVertexAttribArray(5);
            gl::VertexAttribPointer(
                5,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_mat4,
                (3 * size_vec4) as *const c_void,
            );

            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::VertexAttribDivisor(5, 1);

            gl::BindVertexArray(0);
        }

        Map {
            tile,
            shader,
            tile_matrices,
            tile_buffer,
            tiles_num,
            textures,
        }
    }

    pub fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        unsafe {
            self.shader.bind();
            self.shader.set_uniform_mat(0, &(projection * view));
            self.shader.bind_texture(0, &self.textures[0]);

            gl::BindVertexArray(self.tile.vao);
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.tile.indices.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
                self.tiles_num as i32,
            );
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            for tex_id in 0..self.textures.len() {
                gl::DeleteTextures(1, self.textures[tex_id].id as *const u32);
            }
        }
    }
}
