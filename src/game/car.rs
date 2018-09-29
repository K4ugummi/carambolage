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
use nalgebra::geometry::Translation;
use nalgebra::{zero, Matrix4, Vector3};

pub struct Car {
    pub pos: Vector3<f32>,
    _vel: Vector3<f32>,
    _force: Vector3<f32>,
    _mass: f32,

    pub model: Model,
}

impl Car {
    pub fn new(pos: Vector3<f32>, _mass: f32) -> Car {
        assert!(_mass > 0.);
        Car {
            pos,
            _vel: zero(),
            _force: zero(),
            _mass,

            model: Model::new(),
        }
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    pub(super) fn run(&mut self, _time_step: f32) {
        //assert!(time_step > 0.);
        //self.pos += self.vel * time_step
        //    + self.force / (2. * self.mass) * time_step.powi(2);
        //self.vel += self.force / self.mass * time_step;
    }

    pub(super) fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        let model = Translation::from_vector(self.pos).to_homogeneous();
        let mvp = projection * view * model;
        self.model.draw(&mvp);
    }
}
