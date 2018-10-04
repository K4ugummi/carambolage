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

#![allow(dead_code)] // TODO: Remove this flag when the transform is implemented!
use nalgebra::{Matrix4, Vector3};

/// Position, rotation and scale of an object.
///
/// Every gameobject in a scene has (should have) a Transform. Its purpose is to store and manipulate the position,
/// rotation and scale of the gameobject and determine its model matrix for rendering.
pub struct Transform {
    needs_update: bool,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    matrix: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            needs_update: false,
            position: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.),
            scale: Vector3::new(1., 1., 1.),
            matrix: Matrix4::identity(),
        }
    }

    /// Sets the transforms position in world coordinates.
    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
        self.needs_update = true;
    }

    /// Sets the rotation of eulerAngles in the following order:
    /// (I hope the order is right and it is in degrees not radians)
    /// - z degrees around the z axis
    /// - x degrees around the x axis
    /// - y degrees around the y axis
    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
        self.needs_update = true;
    }

    /// Sets the scale of the object for each axis.
    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
        self.needs_update = true;
    }

    /// Sets the objects origin relative to world coordinates.
    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    /// Returns the rotation of eulerAngles.
    /// x: yaw
    /// y: pitch
    /// z: roll
    pub fn rotation(&self) -> Vector3<f32> {
        self.rotation
    }

    /// Returns the scale of the object for each axis.
    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }

    /// Returns the model matrix according to scale, rotation and position of the object in world coordinates.
    /// Note: The object has to be mutable because it stores a model matrix, which will be updated in this function
    /// if some parameter has changed since the last matrix request.
    pub fn matrix(&mut self) -> Matrix4<f32> {
        if self.needs_update {
            let translation = Matrix4::new_translation(&self.position);
            let rotation = Matrix4::from_euler_angles(self.rotation[0], self.rotation[1], self.rotation[2]);
            let scale = Matrix4::from_scaled_axis(self.scale);
            self.matrix = translation * rotation * scale;
        }
        self.matrix
    }
}
