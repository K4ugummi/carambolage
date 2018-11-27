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
use log::{debug, error};

use std::ptr;

pub(crate) struct FrameBuffer {
    fbo: u32,
    rbo: u32,

    pub color_buffer: u32,

    width: i32,
    height: i32,
}

impl FrameBuffer {
    pub(crate) fn new(width: i32, height: i32) -> FrameBuffer {
        debug!("FrameBuffer::new({}, {})", width, height);
        let mut frame_buffer: FrameBuffer = Default::default();

        frame_buffer.width = width;
        frame_buffer.height = height;

        frame_buffer.init();

        frame_buffer
    }

    fn init(&mut self) {
        unsafe {
            gl::GenFramebuffers(1, &mut self.fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

            gl::GenTextures(1, &mut self.color_buffer);
            gl::BindTexture(gl::TEXTURE_2D, self.color_buffer);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::SRGB as i32,
                self.width,
                self.height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.color_buffer, 0);

            gl::GenRenderbuffers(1, &mut self.rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.rbo);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, self.width, self.height);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, self.rbo);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                error!("Framebuffer not complete!");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    /// Bind the this `FrameBuffer` as thew current active one.
    pub unsafe fn bind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
    }

    /// Unbind any bound `FrameBuffer` and bind the default one.
    pub unsafe fn unbind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    /// Resize the buffer with a `width` and `height` in pixels.
    pub unsafe fn resize(&mut self, width: i32, height: i32) {
        debug!("Framebuffer::resize({}, {})", width, height);
        self.width = width;
        self.height = height;

        gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

        gl::BindTexture(gl::TEXTURE_2D, self.color_buffer);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::SRGB as i32,
            self.width,
            self.height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );
        gl::BindRenderbuffer(gl::RENDERBUFFER, self.rbo);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, self.width, self.height);

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.fbo as *const u32);
            gl::DeleteBuffers(1, self.rbo as *const u32);
            gl::DeleteTextures(1, self.color_buffer as *const u32);
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> FrameBuffer {
        FrameBuffer {
            fbo: 0,
            rbo: 0,

            color_buffer: 0,

            width: 0,
            height: 0,
        }
    }
}
