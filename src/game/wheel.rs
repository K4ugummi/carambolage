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

#![allow(dead_code)]

use super::controller::Controller;
use crate::grphx::Model;
use nalgebra::{Vector3, Matrix4, zero};
use ncollide3d::shape::Cylinder;

pub struct Wheel {
    pub position: Vector3<f32>, // Wheel position in local space
    pub rotation: Vector3<f32>, // Wheel rotation

    radius: f32,
    friction: f32,
    is_slipping: bool,
    
    pub model: Model,           // Model of the wheel
    pub cylinder: Cylinder<f32>,     // Wheel collider
}

impl Wheel {
    pub fn new(model: &str, color_palette: &str, position: Vector3<f32>) -> Self {
        
        let model = Model::new(model, color_palette);
        let (min, max) = model.get_min_max();
        let radius = (max.x - min.x) * 0.5;
        let half_height = (max.y - min.y) * 0.5;
        let cylinder = Cylinder::new(half_height, radius);
        
        Wheel {
            position,
            rotation: zero(),
            radius: 1.0,
            friction: 1.0,
            is_slipping: false,

            model,
            cylinder,
        }
    }

    pub fn update(&mut self, _dt: f32, _controller: Option<Controller>) {
        unimplemented!();
    }

    pub fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        let rotation = Matrix4::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        let translation = Matrix4::new_translation(&self.position);
        let model = translation * rotation * Matrix4::new_scaling(0.5f32);
        self.model.draw(&model, view, projection);
    }
}