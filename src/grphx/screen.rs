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

pub(crate) struct Screen {
    vao: u32,
    vbo: u32,

    frame_buffer: FrameBuffer,
    post_proc_shader: Shader,

    post_proc_effect: i32,
}

impl Screen {
    pub(crate) fn new(width: u32, height: u32) -> Screen {
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
            post_proc_effect: 0,
        }
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            self.frame_buffer.resize(width as i32, height as i32);
        }
    }

    pub(crate) fn first_step(&self) {
        unsafe {
            self.frame_buffer.bind();
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub(crate) fn second_step(&self) {
        unsafe {
            self.frame_buffer.unbind();

            gl::Disable(gl::DEPTH_TEST);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.post_proc_shader.bind();
            self.post_proc_shader.set_uniform_int(0, self.post_proc_effect);
            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(5);
            gl::BindTexture(gl::TEXTURE_2D, self.frame_buffer.color_buffer);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    pub(crate) fn set_post_processing(&mut self, value: i32) {
        self.post_proc_effect = value;
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.vbo as *const u32);
            gl::DeleteVertexArrays(1, self.vao as *const u32);
        }
    }
}
