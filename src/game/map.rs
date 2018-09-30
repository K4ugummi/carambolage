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
use super::image;
use super::image::DynamicImage::*;
use super::image::GenericImageView;

use nalgebra::Vector4;

use std::path::Path;

pub struct Map {
    data: Vec<u8>,
}

impl Map {
    pub fn new(file: &str) -> Map {
        let img = image::open(&Path::new(file)).expect("Map failed to load");
        match img {
            ImageRgba8(_) => {}
            _ => panic!("Map must be in RGBA format!"),
        };
        let data = img.raw_pixels();

        println!("width: {}, height: {}", img.width(), img.height());
        for x in 0..img.width() as u32 {
            for y in 0..img.height() as u32 {
                print!("{:?}", Vector4::from(img.get_pixel(x, y).data));
            }
            println!("");
        }
        println!("");

        Map { data }
    }
}
