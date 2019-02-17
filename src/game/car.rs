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
use crate::grphx::Model;
use log::debug;
use ncollide3d::shape::Cuboid;

use nalgebra::{clamp, zero, Matrix4, Vector3};

/// A GameObject controlled by a player.
pub struct Car {
    pub position: Vector3<f32>, // position in world space
    pub rotation: Vector3<f32>, // rotation in radians per axis
    _velocity: Vector3<f32>,
    _force: Vector3<f32>,
    _mass: f32,
    pub boost: f32,

    pub model: Model,
    pub cuboid: Cuboid<f32>,
}

impl Car {
    /// Create a new `Car`.
    ///
    /// For `model` and `color_palette` see `model_from_id()` and `color_from_id()`.
    /// `mass` is the mass of the car in [kg]
    pub fn new(model: &str, color_palette: &str, mass: f32) -> Car {
        debug!("New({}, {}, {})", model, color_palette, mass);

        let model = Model::new(model, color_palette);
        let (min, max) = model.get_min_max();
        let cuboid = Cuboid::new((max - min) * 0.25);

        Car {
            position: zero(),
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

    /// Draw the car to the currently bound framebuffer.
    pub(super) fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        // x,y-axis rotation are fixed to 0. No rollovers!
        let rotation = Matrix4::from_euler_angles(0., 0., self.rotation[2]);
        let translation = Matrix4::new_translation(&self.position);
        let model = translation * rotation * Matrix4::new_scaling(0.5f32);
        self.model.draw(&model, view, projection);
    }

    /// Return a `Car` file name from an id.
    ///
    /// 1:kart 2:parsche 3:farara 4:lamba 5:gtc1 6:gtc2 7:formula
    pub fn model_from_id(id: u32) -> String {
        match id {
            1 => String::from("c01.obj"),
            2 => String::from("c02.obj"),
            3 => String::from("c03.obj"),
            4 => String::from("c04.obj"),
            5 => String::from("c05.obj"),
            6 => String::from("c06.obj"),
            7 => String::from("c07.obj"),
            _ => String::from("c01.obj"),
        }
    }

    /// Return a `Car` color palette file name from an id.
    ///
    /// 1:blue 2:green 3:lime 4:orange 5:purple 6:red 7:yellow
    pub fn color_from_id(id: u32) -> String {
        match id {
            1 => String::from("car-blue.png"),
            2 => String::from("car-green.png"),
            3 => String::from("car-lime.png"),
            4 => String::from("car-orange.png"),
            5 => String::from("car-purple.png"),
            6 => String::from("car-red.png"),
            7 => String::from("car-yellow.png"),
            _ => String::from("car-blue.png"),
        }
    }
}
