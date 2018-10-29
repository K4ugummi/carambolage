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
use nalgebra::{inf, sup, Isometry3, Matrix4, Vector3};
use ncollide3d::query;

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
    pub(super) fn new() -> Scene {
        let cars = vec![
            Car::new("c03.obj", "car-blue.png", Vector3::new(-1.3, 0., 0.2), 1.0),
            Car::new("c04.obj", "car-red.png", Vector3::new(1.3, 0., 0.2), 1.0),
        ];

        let level = Level::new("maps/race_track_1");
        let camera = Camera::new();

        Scene { cars, level, camera }
    }

    /// Update the scene based on the internal state and a given time step.
    pub(super) fn update(&mut self, dt: f32, controller: &[Controller]) {
        // User Input
        for (id, car) in &mut self.cars.iter_mut().enumerate() {
            if id < controller.len() {
                car.update(dt, Some(controller[id]));
            } else {
                car.update(dt, None);
            }
        }

        // Update physics/position
        let mut car_pos = Vec::with_capacity(self.cars.len());
        for car in &self.cars {
            car_pos.push(Isometry3::new(car.position, car.rotation));
        }

        // The whole collision detection is stupid right now. I have learned a lot during my work on this game and
        // the way it is will do the job. I just want to finish the game so it feels "round" and continue with another
        // project. Sorry ¯\_(ツ)_/¯
        let prediction = 0.0;
        // Cars with cars
        for i in 0..car_pos.len() {
            for j in i + 1..car_pos.len() {
                // Most stupid collision detection, but we'll have a world an maybe max 8 cars, wo who cares.
                let penetrate = query::contact(&car_pos[i], &self.cars[i].cuboid, &car_pos[j], &self.cars[j].cuboid, prediction);
                if penetrate.is_some() {
                    let pen = penetrate.unwrap();
                    let w1 = pen.world1;
                    let w2 = pen.world2;
                    let dir = w1 - w2;
                    self.cars[i].position -= dir * 0.5;
                    self.cars[j].position += dir * 0.5;
                }
            }
        }

        // Cars with level
        for (i, cp) in car_pos.iter().enumerate() {
            self.cars[i].position[2] -= 9.81 * dt;

            let penetrate_ground = query::contact(&cp, &self.cars[i].cuboid, &self.level.ground.0, &self.level.ground.1, prediction);
            if penetrate_ground.is_some() {
                let pen = penetrate_ground.unwrap();
                let w1 = pen.world1;
                let w2 = pen.world2;
                let dir = w1 - w2;
                self.cars[i].position -= dir;
            };

            let penetrate_border = query::contact(&cp, &self.cars[i].cuboid, &self.level.track.0, &self.level.track.1, prediction);
            if penetrate_border.is_some() {
                let pen = penetrate_border.unwrap();
                let w1 = pen.world1;
                let w2 = pen.world2;
                let dir = w1 - w2;
                self.cars[i].position -= dir;
            };
        }

        // Just camera stuff
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
            let camera_distance = (max - min).norm() * 1.20;
            self.camera.move_to_height(camera_distance + 10.0);
            lerp_pos
        };
        self.camera.move_to_focus(camera_focus);
        self.camera.update(dt);
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
