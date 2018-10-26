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
#![macro_use]

use super::image;
use super::image::DynamicImage::*;
use super::image::GenericImageView;
use super::shader::Shader;
use super::tile::{Tile, TileType};

use nalgebra::{Matrix4, Vector3};

use std::path::Path;

pub struct Level {
    tiles: Vec<Tile>,
    pub shader: Shader,
    _width: u32,
    _height: u32,
}

impl Level {
    pub fn new(file: &str) -> Level {
        //debug!("New from {}", file);
        // Load the map file.
        let img = image::open(&Path::new(file)).expect("ERROR: Map failed to load").flipv();
        debug!("{}x{} px", img.width(), img.height());
        match img {
            ImageRgba8(_) => debug!("Format ImageRgba8"),
            _ => panic!("ERROR: Map not in RGBA8 format!"),
        };
        let tiles_num_x = img.width() as usize;
        let tiles_num_y = img.height() as usize;

        // Create a unique tile for each sprite.
        // Uhhh thats ugly :o
        let mut tiles: Vec<Tile> = Vec::new();
        tiles.push(Tile::new(TileType::TerrainGrass));
        tiles.push(Tile::new(TileType::TrackCheckpoint));
        tiles.push(Tile::new(TileType::TrackCornerLarge));
        tiles.push(Tile::new(TileType::TrackCornerSmall));
        tiles.push(Tile::new(TileType::TrackLeftRightLarge));
        tiles.push(Tile::new(TileType::TrackPitEntryLaneSmall));
        tiles.push(Tile::new(TileType::TrackPitEntryLarge));
        tiles.push(Tile::new(TileType::TrackPitExitLaneSmall));
        tiles.push(Tile::new(TileType::TrackPitExitLarge));
        tiles.push(Tile::new(TileType::TrackRightLeftLarge));
        tiles.push(Tile::new(TileType::TrackStartFinish));
        tiles.push(Tile::new(TileType::TrackStraightSmall));
        tiles.push(Tile::new(TileType::TrackTire));

        // Load the tile shader.
        let shader = Shader::new("tile");

        // Create a stride in x,y-direction to place the tiles.
        let stride_x = Vector3::new(8f32, 0., 0.);
        let stride_y = Vector3::new(0f32, 8., 0.);
        let tile_start = Vector3::new(
            -(img.width() as f32 / 2.) * stride_x[0],
            -(img.width() as f32 / 2.) * stride_y[1],
            -1.05,
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
            self.shader.set_uniform_mat(0, view);
            self.shader.set_uniform_mat(1, projection);
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
        TileType::TerrainGrass
    } else {
        let up = image.get_pixel(x as u32, (y + 1) as u32).data[0];
        let down = image.get_pixel(x as u32, (y - 1) as u32).data[0];
        let left = image.get_pixel((x - 1) as u32, y as u32).data[0];
        let right = image.get_pixel((x + 1) as u32, y as u32).data[0];

        match (up, down, left, right) {
            (255, 255, 255, 255) => TileType::TerrainGrass,
            (255, 255, 0, 0) => TileType::TerrainGrass,
            (0, 0, 255, 255) => TileType::TerrainGrass,
            (255, 0, 255, 0) => TileType::TerrainGrass,
            (255, 0, 0, 255) => TileType::TerrainGrass,
            (0, 255, 255, 0) => TileType::TerrainGrass,
            (0, 255, 0, 255) => TileType::TerrainGrass,
            (255, 0, 255, 255) => TileType::TerrainGrass,
            (255, 255, 255, 0) => TileType::TerrainGrass,
            (0, 255, 255, 255) => TileType::TerrainGrass,
            (255, 255, 0, 255) => TileType::TerrainGrass,

            (_, _, _, _) => TileType::TerrainGrass,
        }
    }
    /*
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
    */
}
