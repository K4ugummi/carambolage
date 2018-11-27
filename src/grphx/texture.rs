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
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImageView;
use log::{debug, error};

use std::os::raw::c_void;
use std::path::Path;

/// A 2D Texture for OpenGL
pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn new(path: &str) -> Texture {
        unsafe { Texture { id: load_texture(path) } }
    }
}

impl Default for Texture {
    fn default() -> Texture {
        Texture { id: 0 }
    }
}

unsafe fn load_texture(path: &str) -> u32 {
    let path_str = format!("{}{}", "res/textures/", path);
    debug!("New from {}", path_str);

    let path = Path::new(&path_str);

    let mut tex_id = 0;

    gl::GenTextures(1, &mut tex_id);
    let img = image::open(&path).expect("ERROR: Failed to load texture!").flipv();
    let image_format = match img {
        ImageRgb8(_) => {
            debug!("Format: RGB8");
            gl::RGB
        }
        ImageRgba8(_) => {
            debug!("Format: RGBA8");
            gl::RGBA
        }
        _ => {
            error!("Format wrong");
            panic!("ERROR: Wrong image format!")
        }
    };

    let data = img.raw_pixels();

    gl::BindTexture(gl::TEXTURE_2D, tex_id);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        image_format as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        image_format,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

    debug!("id:{}, width:{}px, height:{}px", tex_id, img.width(), img.height());

    tex_id
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, self.id as *const u32);
        }
    }
}
