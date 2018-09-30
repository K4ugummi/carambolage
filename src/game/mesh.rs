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
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.
#![macro_use]

use std::mem::size_of;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

use super::gl;
use super::shader::Shader;

use super::image;
use super::image::DynamicImage::*;
use super::image::GenericImageView;

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    };
}

#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture>,

    vao: u32,
    vbo: u32,
    ibo: u32,
}

pub struct Texture {
    pub id: u32,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        textures: Vec<Texture>,
    ) -> Mesh {
        let mut mesh: Mesh = Default::default();

        unsafe {
            mesh.init(vertices, indices, textures);
        }

        mesh
    }

    /// render the mesh
    pub unsafe fn draw(&self, shader: &Shader) {
        shader.bind_texture(0, &self.textures[0]);

        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
        gl::BindVertexArray(0);
    }

    unsafe fn init(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        textures: Vec<Texture>,
    ) {
        self.vertices = vertices;
        self.indices = indices;
        self.textures = textures;

        // VAO
        gl::GenVertexArrays(1, &mut self.vao);
        gl::BindVertexArray(self.vao);

        // VBO
        gl::GenBuffers(1, &mut self.vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // IBO
        gl::GenBuffers(1, &mut self.ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let size = size_of::<Vertex>() as i32;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, pos) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, uv) as *const c_void,
        );

        gl::BindVertexArray(0);
    }
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

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            pos: [0., 0., 0.],
            uv: [0., 0.],
        }
    }
}

impl Default for Mesh {
    fn default() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            textures: Vec::new(),
            vao: 0,
            vbo: 0,
            ibo: 0,
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            for tex_id in 0..self.textures.len() {
                gl::DeleteTextures(1, self.textures[tex_id].id as *const u32);
            }
            gl::DeleteBuffers(1, self.ibo as *const u32);
            gl::DeleteBuffers(1, self.vbo as *const u32);
            gl::DeleteVertexArrays(1, self.vao as *const u32);
        }
    }
}

pub unsafe fn load_texture(path: &str) -> u32 {
    let mut tex_id = 0;

    gl::GenTextures(1, &mut tex_id);
    let img = image::open(&Path::new(path)).expect("Texture failed to load");
    match img {
        ImageRgba8(_) => {}
        _ => panic!("Texture must be in RGBA format!"),
    };

    let data = img.raw_pixels();

    gl::BindTexture(gl::TEXTURE_2D, tex_id);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA8 as i32,
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
