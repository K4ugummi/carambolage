use super::gl;
use super::image;
use super::image::DynamicImage::*;
use super::image::GenericImageView;

use std::os::raw::c_void;
use std::path::Path;

pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn new(path: &str) -> Texture {
        unsafe {
            Texture {
                id: load_texture(path),
            }
        }
    }
}

impl Default for Texture {
    fn default() -> Texture {
        Texture { id: 0 }
    }
}

unsafe fn load_texture(path: &str) -> u32 {
    let mut tex_id = 0;

    gl::GenTextures(1, &mut tex_id);
    let img =
        image::open(&Path::new(path)).expect("ERROR: Failed to load texture!");
    let image_format = match img {
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
        _ => panic!("ERROR: Wrong image format!"),
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
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MAG_FILTER,
        gl::LINEAR as i32,
    );

    tex_id
}
