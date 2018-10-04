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
use nalgebra::{inf, sup, Matrix4, Vector3};
use rand::{thread_rng, Rng};
use time::Duration;

use super::camera::Camera;
use super::car::Car;
use super::controller::Controller;
use super::level::Level;

pub(super) struct Scene {
    pub cars: Vec<Car>,
    pub level: Level,
    pub camera: Camera,
}

impl Scene {
    /// Make a new scene with a given number of cars.
    pub(super) fn new(num_cars: usize) -> Scene {
        let mut rng = thread_rng();
        let cars: Vec<Car> = (0..num_cars)
            .map(|_| {
                Car::new(
                    {
                        let x = rng.gen_range(-20f32, 20f32);
                        let y = rng.gen_range(-20f32, 20f32);
                        Vector3::new(x, y, 0.)
                    },
                    1.0,
                )
            }).collect();

        let level = Level::new("res/maps/example.png");
        let camera = Camera::new();

        Scene { cars, level, camera }
    }

    /// Update the scene based on the internal state and a given time step.
    pub(super) fn run(&mut self, delta_time: Duration, controller: &[Controller]) {
        for (id, car) in &mut self.cars.iter_mut().enumerate() {
            if id < controller.len() {
                car.run(delta_time, Some(controller[id]));
            } else {
                car.run(delta_time, None);
            }
        }
        let camera_focus = if self.cars.is_empty() {
            Vector3::new(0., 0., 0.)
        } else {
            let mut min = self.cars[0].position;
            let mut max = self.cars[0].position;
            let mut lerp_pos = Vector3::new(0., 0., 0.);
            for car in &self.cars {
                lerp_pos += car.position;
                min = inf(&min, &car.position);
                max = sup(&max, &car.position);
            }
            lerp_pos /= self.cars.len() as f32;
            let camera_distance = (max - min).norm();
            self.camera.move_to_height(camera_distance);
            lerp_pos
        };
        self.camera.move_to_focus(camera_focus);
        self.camera.update(delta_time);
    }

    pub(super) fn draw(&mut self, projection: &Matrix4<f32>) {
        let view = self.camera.get_viewmatrix();
        // Draw map.
        self.level.draw(&view, &projection);
        // Draw objects.
        for i in 0..self.cars.len() {
            self.cars[i].draw(&view, &projection);
        }
    }
}
