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
use super::model::Model;
use nalgebra::{zero, Matrix4, Vector3};

pub struct Car {
    pub pos: Vector3<f32>, // position in world space
    pub ori: Vector3<f32>, // rotation in radians per axis
    vel: f32,
    acc: f32,
    pub model: Model,
}

impl Car {
    pub fn new(pos: Vector3<f32>) -> Car {
        Car {
            pos,
            ..Default::default()
        }
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    pub(super) fn run(&mut self, time_step: f32) {
        assert!(time_step > 0.);
        self.vel += self.acc * time_step;
        self.pos += self.ori * self.vel * time_step;
    }

    pub(super) fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        let rotation = Matrix4::from_euler_angles(
            self.ori[0],
            self.ori[1],
            self.ori[2],
        );
        let translation = Matrix4::new_translation(&self.pos);
        let model = rotation * translation;
        let mvp = projection * view * model;
        self.model.draw(&mvp);
    }

    pub(super) fn set_acc(&mut self, acc: f32) {
        self.acc = acc;
    }
}

impl Default for Car {
    fn default() -> Car {
        Car {
            pos: zero(),
            ori: Vector3::new(0., 0., 0.),
            vel: zero(),
            acc: zero(),
            model: Model::new(),
        }
    }
}
