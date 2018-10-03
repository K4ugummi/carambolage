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

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg_attr(feature = "cargo-clippy", allow(enum_variant_names))]
#[derive(Copy, Clone, Debug)]
enum TileType {
    RoadNONE    = 0,
    // Straight
    RoadNS      = 1,
    RoadEW      = 2,
    // Curve
    RoadNE      = 3,
    RoadNW      = 4,
    RoadSE      = 5,
    RoadSW      = 6,
    // X-Cross
    RoadNSEW    = 7,
    // T-Cross
    RoadNEW     = 8,
    RoadNWS     = 9,
    RoadEWS     = 10,
    RoadNES     = 11,
}

struct Tile {
    vertices: Vec<TileVertex>,
    indices: Vec<u32>,
    pub matrices: Vec<Matrix4<f32>>,
    vao: u32,
    vbo: u32,
    ibo: u32,
    instance_buffer: u32,
    texture: Texture,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        let mut tile: Tile = Default::default();
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices = vec![
            TileVertex { pos: [5., 5.,], uv: [1., 0.,] },
            TileVertex { pos: [-5., 5.,], uv: [0., 0.,] },
            TileVertex { pos: [-5., -5.,], uv: [0., 1.,] },
            TileVertex { pos: [5., -5.,], uv: [1., 1.,] },
        ];
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let indices = vec![
            0u32, 1, 2,
            0, 2, 3,
        ];

        unsafe {
            tile.init(vertices, indices);
        }

        tile.texture = match tile_type {
            TileType::RoadNONE => Texture::new("res/maps/tiles/roadNONE.png"),

            TileType::RoadNS => Texture::new("res/maps/tiles/roadNS.png"),
            TileType::RoadEW => Texture::new("res/maps/tiles/roadEW.png"),

            TileType::RoadNE => Texture::new("res/maps/tiles/roadNE.png"),
            TileType::RoadNW => Texture::new("res/maps/tiles/roadNW.png"),
            TileType::RoadSE => Texture::new("res/maps/tiles/roadSE.png"),
            TileType::RoadSW => Texture::new("res/maps/tiles/roadSW.png"),

            TileType::RoadNSEW => Texture::new("res/maps/tiles/roadNSEW.png"),

            TileType::RoadNEW => Texture::new("res/maps/tiles/roadNEW.png"),
            TileType::RoadNWS => Texture::new("res/maps/tiles/roadNWS.png"),
            TileType::RoadEWS => Texture::new("res/maps/tiles/roadEWS.png"),
            TileType::RoadNES => Texture::new("res/maps/tiles/roadNES.png"),
        };

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
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, size, offset_of!(TileVertex, pos) as *const c_void);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, size, offset_of!(TileVertex, uv) as *const c_void);

        gl::BindVertexArray(0);
    }

    pub fn init_instance_buffer(&mut self) {
        let size_mat4 = size_of::<Matrix4<f32>>() as i32;
        let size_vec4 = size_of::<Vector4<f32>>() as i32;
        unsafe {
            gl::GenBuffers(1, &mut self.instance_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.matrices.len() * size_mat4 as usize) as isize,
                &self.matrices[0] as *const Matrix4<f32> as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(self.vao);

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, size_mat4, ptr::null());
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, size_mat4, size_vec4 as *const c_void);
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, size_mat4, (2 * size_vec4) as *const c_void);
            gl::EnableVertexAttribArray(5);
            gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, size_mat4, (3 * size_vec4) as *const c_void);

            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::VertexAttribDivisor(5, 1);

            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self, shader: &Shader) {
        unsafe {
            shader.bind_texture(0, &self.texture);

            gl::BindVertexArray(self.vao);
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
                self.matrices.len() as i32,
            );
            gl::BindVertexArray(0);
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            vertices: Vec::new(),
            indices: Vec::new(),
            matrices: Vec::new(),
            vao: 0,
            vbo: 0,
            ibo: 0,
            instance_buffer: 0,
            texture: Default::default(),
        }
    }
}

impl Drop for Tile {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, self.texture.id as *const u32);
            gl::DeleteBuffers(1, self.instance_buffer as *const u32);
            gl::DeleteBuffers(1, self.ibo as *const u32);
            gl::DeleteBuffers(1, self.vbo as *const u32);
            gl::DeleteVertexArrays(1, self.vao as *const u32);
        }
    }
}

pub struct Level {
    tiles: Vec<Tile>,
    pub shader: Shader,
    _width: u32,
    _height: u32,
}

impl Level {
    pub fn new(file: &str) -> Level {
        // Load the map file.
        let img = image::open(&Path::new(file)).expect("ERROR: Map failed to load").flipv();
        match img {
            ImageRgba8(_) => {}
            _ => panic!("ERROR: Map must be in RGBA8 format!"),
        };
        let tiles_num_x = img.width() as usize;
        let tiles_num_y = img.height() as usize;

        // Create a unique tile for each sprite.
        // Uhhh thats ugly :o
        let mut tiles: Vec<Tile> = Vec::new();
        tiles.push(Tile::new(TileType::RoadNONE));

        tiles.push(Tile::new(TileType::RoadNS));
        tiles.push(Tile::new(TileType::RoadEW));

        tiles.push(Tile::new(TileType::RoadNE));
        tiles.push(Tile::new(TileType::RoadNW));
        tiles.push(Tile::new(TileType::RoadSE));
        tiles.push(Tile::new(TileType::RoadSW));

        tiles.push(Tile::new(TileType::RoadNSEW));

        tiles.push(Tile::new(TileType::RoadNEW));
        tiles.push(Tile::new(TileType::RoadNWS));
        tiles.push(Tile::new(TileType::RoadEWS));
        tiles.push(Tile::new(TileType::RoadNES));

        // Load the tile shader.
        let shader = Shader::new("tile");

        // Create a stride in x,y-direction to place the tiles.
        let stride_x = Vector3::new(10f32, 0., 0.);
        let stride_y = Vector3::new(0f32, 10., 0.);
        let tile_start = Vector3::new(
            -(img.width() as f32 / 2.) * stride_x[0],
            -(img.width() as f32 / 2.) * stride_y[1],
            -0.01,
        );

        // Generate translation matrices for each tile.
        // Note: This could be done with Vector3, because right now,
        // were just translating, but who knows what else will come.
        for y in 0..tiles_num_y {
            for x in 0..tiles_num_x {
                let tile_type = get_tile_type(&img, x, y);
                let matrix = Matrix4::new_translation(&(tile_start + stride_x * (x as f32) + stride_y * (y as f32)));
                tiles[tile_type as usize].matrices.push(matrix);
            }
        }

        for tile in &mut tiles {
            tile.init_instance_buffer();
        }

        Level {
            tiles,
            shader,
            _width: tiles_num_x as u32,
            _height: tiles_num_y as u32,
        }
    }

    pub fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        unsafe {
            self.shader.bind();
            self.shader.set_uniform_mat(0, &(projection * view));
            for tile in &self.tiles {
                tile.draw(&self.shader);
            }
        }
    }
}

// Oh ohh... it works but needs some cleaning (later), when we have more tile sprites.
fn get_tile_type(image: &image::DynamicImage, x: usize, y: usize) -> TileType {
    let pixel = image.get_pixel(x as u32, y as u32);
    let pixel_data = (pixel.data[0], pixel.data[1], pixel.data[2], pixel.data[3]);

    if pixel_data == (0, 0, 0, 255) {
        TileType::RoadNONE
    } else {
        let up = image.get_pixel(x as u32, (y + 1) as u32).data[0];
        let down = image.get_pixel(x as u32, (y - 1) as u32).data[0];
        let left = image.get_pixel((x - 1) as u32, y as u32).data[0];
        let right = image.get_pixel((x + 1) as u32, y as u32).data[0];

        match (up, down, left, right) {
            (255, 255, 255, 255) => TileType::RoadNSEW,

            (255, 255, 0, 0) => TileType::RoadNS,
            (0, 0, 255, 255) => TileType::RoadEW,

            (255, 0, 255, 0) => TileType::RoadNW,
            (255, 0, 0, 255) => TileType::RoadNE,
            (0, 255, 255, 0) => TileType::RoadSW,
            (0, 255, 0, 255) => TileType::RoadSE,

            (255, 0, 255, 255) => TileType::RoadNEW,
            (255, 255, 255, 0) => TileType::RoadNWS,
            (0, 255, 255, 255) => TileType::RoadEWS,
            (255, 255, 0, 255) => TileType::RoadNES,

            (_, _, _, _) => TileType::RoadNONE,
        }
    }
}
