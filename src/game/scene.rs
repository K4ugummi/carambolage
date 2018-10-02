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
use super::level::Level;

pub(super) struct Scene {
    pub cars: Vec<Car>,
    pub level: Level,
}

impl Scene {
    /// Make a new scene with a given number of cars.
    pub(super) fn new(num_cars: usize) -> Scene {
        assert!(num_cars > 0);
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
    pub(super) fn run(&mut self, time_step: Duration) {
        let time_step = (time_step.num_milliseconds() * 1_000) as f32;
        for car in &mut self.cars {
            car.run(time_step);
        }
    }

    pub(super) fn draw(&self, projection: &Matrix4<f32>) {
        assert!(!self.cars.is_empty());

        let mut min = self.cars[0].position;
        let mut max = self.cars[0].position;
        let mut camera_pos = zero();
        for car in &self.cars {
            camera_pos += car.position;
            min = inf(&min, &car.position);
            max = sup(&max, &car.position);
        }
        camera_pos /= self.cars.len() as f32;
        let camera_distance = (max - min).norm();

        let view = Matrix4::look_at_rh(
            &Point3::from_coordinates(
                camera_pos + Vector3::new(
                    0.,
                    0.,
                    camera_distance + (50. / self.cars.len() as f32),
                ),
            ),
            &Point3::from_coordinates(camera_pos),
            &Vector3::y_axis(),
        );

        // Draw map.
        self.level.draw(&view, &projection);
        // Draw objects.
        for i in 0..self.cars.len() {
            self.cars[i].draw(&view, &projection);
        }
    }
}
