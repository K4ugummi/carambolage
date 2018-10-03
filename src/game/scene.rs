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
use nalgebra::{inf, sup, zero, Matrix4, Point3, Vector3};
use rand::{thread_rng, Rng};
use time::Duration;

use super::car::Car;
use super::controller::Controller;
use super::level::Level;

pub(super) struct Scene {
    pub cars: Vec<Car>,
    pub level: Level,
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

        Scene { cars, level }
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
    }

    pub(super) fn draw(&self, projection: &Matrix4<f32>) {
        let view = if self.cars.is_empty() {
            Matrix4::look_at_rh(&Point3::new(0., 0., 50.), &Point3::new(0., 0., 0.), &Vector3::y())
        } else {
            let mut min = self.cars[0].center_of_mass;
            let mut max = self.cars[0].center_of_mass;
            let mut camera_pos = zero();
            for car in &self.cars {
                camera_pos += car.center_of_mass;
                min = inf(&min, &car.center_of_mass);
                max = sup(&max, &car.center_of_mass);
            }
            camera_pos /= self.cars.len() as f32;
            let camera_distance = (max - min).norm();

            Matrix4::look_at_rh(
                &Point3::from_coordinates(camera_pos + Vector3::new(0., 0., camera_distance + (50. / self.cars.len() as f32))),
                &Point3::from_coordinates(camera_pos),
                &Vector3::y(),
            )
        };

        // Draw map.
        self.level.draw(&view, &projection);
        // Draw objects.
        for i in 0..self.cars.len() {
            self.cars[i].draw(&view, &projection);
        }
    }
}
