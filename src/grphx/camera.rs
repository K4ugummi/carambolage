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
use crate::util::Lerp;
use log::info;
use nalgebra::{clamp, Matrix4, Point3, Vector3};
use serde_derive::{Deserialize, Serialize};

/// Camera to calculate the view matrix and follow ingame objects.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Camera {
    // Parameter to create view matrix.
    position: Vector3<f32>,
    focus: Vector3<f32>,
    up: Vector3<f32>,

    // Internal parameter.
    height: f32,
    height_min: f32,
    height_max: f32,
    speed: f32,

    // Parameter for camera movement.
    focus_goal: Vector3<f32>,
    height_goal: f32,
    pub is_smooth_zoom: bool,
    pub is_smooth_pan: bool,
}

impl Camera {
    /// Create a new `Camera` with fixed values.
    pub fn new() -> Camera {
        info!("Camera::new()");
        let height = 50.;
        Camera {
            position: Vector3::new(0., 0., height),
            focus: Vector3::new(0., 0., 0.),
            up: Vector3::new(0., 1., 0.),
            height,
            height_min: 30.,
            height_max: 140.,
            speed: 1.8,

            focus_goal: Vector3::new(0., 0., 0.),
            height_goal: height,

            is_smooth_zoom: true,
            is_smooth_pan: true,
        }
    }

    /// Update the cameras position relative to the delta time `dt`.
    pub fn update(&mut self, dt: f32) {
        if self.is_smooth_pan {
            self.focus = Vector3::lerp(&self.focus, &self.focus_goal, self.speed * dt);
        } else {
            self.focus = self.focus_goal;
        }
        if self.is_smooth_zoom {
            self.height = f32::lerp(&self.height, &self.height_goal, self.speed * dt);
        } else {
            self.height = self.height_goal;
        }
        self.position = self.focus + Vector3::new(0., 0., self.height);
    }

    /// Smooth transition to the focus goal postion.
    pub fn move_to_focus(&mut self, position: Vector3<f32>) {
        self.focus_goal = position;
    }

    /// Set the focus goal and the camera instantaneously.
    pub fn _set_focus(&mut self, position: Vector3<f32>) {
        self.focus_goal = position;
        self.focus = position;
        self.position = position + Vector3::new(0., 0., self.height);
    }

    /// Smooth transition to the camera height goal.
    pub fn move_to_height(&mut self, distance: f32) {
        self.height_goal = clamp(distance, self.height_min, self.height_max);
    }

    /// Set the camera height instantaneously.
    pub fn _set_height(&mut self, distance: f32) {
        self.height_goal = clamp(distance, self.height_min, self.height_max);
        self.height = self.height_goal;
    }

    /// Get the view matrix, calculated from camera values.
    pub fn get_viewmatrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &Point3::from(self.position + Vector3::new(0., -5., 0.)),
            &Point3::from(self.focus_goal),
            &self.up,
        )
    }
}
