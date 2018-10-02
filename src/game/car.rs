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
use super::controller::Controller;
use super::model::Model;

use nalgebra::{zero, Matrix4, Vector3};
use time::Duration;

pub struct Car {
    pub position: Vector3<f32>, // position in world space
    pub rotation: Vector3<f32>, // rotation in radians per axis
    _velocity: Vector3<f32>,
    _force: Vector3<f32>,
    mass: f32,

    pub model: Model,
}

impl Car {
    pub fn new(position: Vector3<f32>, mass: f32) -> Car {
        let mut car: Car = Default::default();
        car.position = position;
        if mass > 1. {
            car.mass = mass;
        }

        car
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    pub(super) fn run(
        &mut self,
        delta_time: Duration,
        controller: Option<Controller>,
    ) {
        /*
        if controller.is_some() {
            let dt = delta_time.num_milliseconds() as f32 / 1_000.;
            let controls = controller.unwrap();
            let translation = Matrix4::new_translation(
                &(Vector3::new(0., 1., 0.) * controls.y_axis() * dt),
            );
            let rotation =
                Matrix4::from_euler_angles(0., 0., controls.x_axis() * dt);
            self.position = self.position + Vector3::from_homogeneous(
                translation * rotation * self.position.to_homogeneous(),
            );
        }
        */
    }

    pub(super) fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        let rotation = Matrix4::from_euler_angles(
            self.rotation[0],
            self.rotation[1],
            self.rotation[2],
        );
        let translation = Matrix4::new_translation(&self.position);
        let model = rotation * translation;
        let mvp = projection * view * model;
        self.model.draw(&mvp);
    }
}

impl Default for Car {
    fn default() -> Car {
        Car {
            position: zero(),
            rotation: zero(),
            _velocity: zero(),
            _force: zero(),
            mass: 1.,

            model: Model::new(),
        }
    }
}
