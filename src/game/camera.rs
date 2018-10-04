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
use nalgebra::{Matrix4, Point3, Vector3};
use time::Duration;
use util::Lerp;

pub struct Camera {
    // Parameter to create view matrix.
    position: Vector3<f32>,
    focus: Vector3<f32>,
    up: Vector3<f32>,

    // Internal parameter.
    height: f32,
    speed: f32,

    // Parameter for camera movement.
    focus_goal: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        let height = 50.;
        Camera {
            position: Vector3::new(0., 0., height),
            focus: Vector3::new(0., 0., 0.),
            up: Vector3::new(0., 1., 0.),
            height,
            speed: 0.8,

            focus_goal: Vector3::new(0., 0., 0.),
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        let dt = (delta_time.num_milliseconds() as f32) / 1_000.;
        self.focus = Vector3::lerp(&self.focus, &self.focus_goal, speed * dt);
        self.position = self.focus + Vector3::new(0., 0., self.height);
    }

    pub fn move_to_focus(&mut self, position: Vector3<f32>) {
        self.focus_goal = position;
    }

    pub fn set_to_focus(&mut self, position: Vector3<f32>) {
        self.focus_goal = position;
        self.focus = position;
        self.position = position + Vector3::new(0., 0., self.height);
    }

    pub fn get_viewmatrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &Point3::from_coordinates(self.position),
            &Point3::from_coordinates(self.focus),
            &self.up,
        )
    }
}
