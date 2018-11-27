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

/// Camera module to calculate view matrix.
mod camera;
/// Frame buffer for background rendering.
mod framebuffer;
/// Material unused.
mod material;
/// 3D Mesh for Model
mod mesh;
/// 3D Model for rendering.
mod model;
/// Handle FrameBuffer blending.
mod screen;
/// OpenGL shader program and usability functions.
mod shader;
/// 2D Texture for Models
mod texture;

pub(crate) use self::camera::*;
pub(crate) use self::framebuffer::*;
pub(crate) use self::mesh::*;
pub(crate) use self::model::*;
pub(crate) use self::screen::*;
pub(crate) use self::shader::*;
pub(crate) use self::texture::*;
