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
use super::{FrameBuffer, Shader};

use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

/// Contains the `Framebuffer` and uses a shader for simple postprocessing.
pub(crate) struct Screen {
    vao: u32,
    vbo: u32,

    frame_buffer: FrameBuffer,
    post_proc_shader: Shader,
}

impl Screen {
    /// Create a new `Screen` with `width`and `height`in pixels.
    pub(crate) fn new(width: u32, height: u32) -> Screen {
        // Vertex coordinates of two triangles from [-1.0, -1.0] to [1.0, 1.0].
        let vertices: [f32; 24] = [
            -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
        ];
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as isize,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = 4 * size_of::<f32>() as i32;
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * size_of::<f32>()) as *const c_void);
        }

        let frame_buffer = FrameBuffer::new(width as i32, height as i32);
        let post_proc_shader = Shader::new("post_proc");

        Screen {
            vao,
            vbo,
            frame_buffer,
            post_proc_shader,
        }
    }

    /// Takes the width and height in pixels for resizing the frame buffer.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            self.frame_buffer.resize(width as i32, height as i32);
        }
    }

    /// First step to render our scene.
    ///
    /// All buffers are cleared and depth testing is enabled again.
    pub(crate) fn first_step(&self) {
        unsafe {
            self.frame_buffer.bind();
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.5607, 0.7254, 0.298, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Secent step to render our scene.
    ///
    /// The scene is rendered to our framebuffer which is drawn
    /// to the default framebuffer.
    pub(crate) fn second_step(&self, gamma: f32) {
        unsafe {
            self.frame_buffer.unbind();

            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.post_proc_shader.bind();
            gl::BindVertexArray(self.vao);
            gl::Uniform1f(4, gamma);
            gl::ActiveTexture(5);
            gl::BindTexture(gl::TEXTURE_2D, self.frame_buffer.color_buffer);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

/// Delete the generated vertex array and all buffers.
impl Drop for Screen {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.vbo as *const u32);
            gl::DeleteVertexArrays(1, self.vao as *const u32);
        }
    }
}
