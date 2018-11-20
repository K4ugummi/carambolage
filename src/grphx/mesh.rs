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
use log::{debug, info};
use serde_derive::{Deserialize, Serialize};

use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    };
}

/// A single point in 3D.
///
/// Currently it is used to represent a Vertex of a triangulated `Mesh`.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

/// Creates a `Vertex`with all values set to `0.0`.
impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            position: [0., 0., 0.],
            normal: [0., 0., 0.],
            uv: [0., 0.],
        }
    }
}

/// Part of a 3D Model.
///
/// It contains a `Vec<Vertex>` each representing a Point of the Model.
/// `Vec<u32>` is used to traw indexed triangles so three indices link to
/// the corresponding `Vertex` in `vertices`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Mesh {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,

    pub(super) vao: u32,
    pub(super) vbo: u32,
    pub(super) ibo: u32,
}

impl Mesh {
    /// Create a new Mesh by passing  vertices and indices as parameter.
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
        info!(
            "Mesh::new( vertices.len() == {}, indices.len() == {}",
            vertices.len(),
            indices.len()
        );
        let mut mesh: Mesh = Default::default();
        mesh.vertices = vertices;
        mesh.indices = indices;

        unsafe {
            mesh.init();
        }

        mesh
    }

    /// render the mesh
    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);
    }

    /// Generate the vertex array object and all buffers.
    unsafe fn init(&mut self) {
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
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, position) as *const c_void);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, normal) as *const c_void);
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, uv) as *const c_void);

        gl::BindVertexArray(0);

        debug!("Mesh::init() : vao == {}, vbo == {}, ibo == {}", self.vao, self.vbo, self.ibo);
    }
}

impl Default for Mesh {
    fn default() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            vao: 0,
            vbo: 0,
            ibo: 0,
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.ibo as *const u32);
            gl::DeleteBuffers(1, self.vbo as *const u32);
            gl::DeleteVertexArrays(1, self.vao as *const u32);
        }
    }
}
