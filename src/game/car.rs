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
use grphx::Model;
use ncollide3d::shape::Cuboid;

use nalgebra::{clamp, zero, Matrix4, Vector3};

pub(crate) struct Car {
    pub(crate) position: Vector3<f32>, // position in world space
    pub(crate) rotation: Vector3<f32>, // rotation in radians per axis
    _velocity: Vector3<f32>,
    _force: Vector3<f32>,
    _mass: f32,
    boost: f32,

    pub(crate) model: Model,
    pub(crate) cuboid: Cuboid<f32>,
}

impl Car {
    pub fn new(model: &str, color_palette: &str, position: Vector3<f32>, mass: f32) -> Car {
        debug!("New({}, {}, {:?}, {})", model, color_palette, position, mass);

        let model = Model::new(model, color_palette);
        let (min, max) = model.get_min_max();
        let cuboid = Cuboid::new((max - min) * 0.25);

        Car {
            position,
            rotation: zero(),
            _velocity: zero(),
            _force: zero(),
            _mass: mass,
            boost: 100.0,
            model,
            cuboid,
        }
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    pub(super) fn update(&mut self, dt: f32, controller: Option<Controller>) {
        if controller.is_some() {
            let ct = controller.unwrap();

            // accel:  0.0 - None
            //         1.0 - Pedal to the metal
            //        -1.0 - Emergency brake
            let accel = ct.get_y_axis();
            // steer:  0.0 - Forward
            //         1.0 - Full right
            //        -1.0 - Full left
            // * accel to prevent steering a non moving car.
            let steer = ct.get_x_axis() * accel;

            // x,y-axis rotation are fixed to 0. No rollovers!
            self.rotation[2] -= steer * dt * 3.5;

            let rot_mat = Matrix4::new_rotation(self.rotation);
            let mut forward = Vector3::new(0f32, 1., 0.).to_homogeneous();
            forward = rot_mat * forward;
            // Set homogeneous coordinate to 0 or unwrap() will panic.
            forward[3] = 0.;

            let booster = if ct.get_boost() {
                self.boost = clamp(self.boost - dt * 30.0, 0.0, 100.0);
                if self.boost > 0.1 {
                    14.0
                } else {
                    10.0
                }
            } else {
                self.boost = clamp(self.boost + dt * 14.0, 0.0, 100.0);
                10.0
            };

            self.position += Vector3::from_homogeneous(forward).unwrap() * accel * booster * dt;
        }
    }

    pub(super) fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        // x,y-axis rotation are fixed to 0. No rollovers!
        let rotation = Matrix4::from_euler_angles(0., 0., self.rotation[2]);
        let translation = Matrix4::new_translation(&self.position);
        let model = translation * rotation * Matrix4::new_scaling(0.5f32);
        self.model.draw(&model, view, projection);
    }
}
