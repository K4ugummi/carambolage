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
use super::car::Car;
use super::controller::Controller;
use super::level::Level;
use crate::grphx::Camera;
use nalgebra::{inf, sup, zero, Isometry3, Matrix4, Vector3};
use ncollide3d::query;

/// Main application Scene.
///
/// This scene consists of `GameObject`s, an `Environment` and a main `Camera`.
pub struct Scene {
    pub cars: Vec<Car>,
    pub level: Level,
    pub camera: Camera,
}

impl Scene {
    /// Create a new scene. Choose a map via id.
    pub fn new(map_id: u32) -> Scene {
        let mut cars = Vec::new();
        cars.push(Car::new(&Car::model_from_id(3), &Car::color_from_id(1), 1.0));
        cars.push(Car::new(&Car::model_from_id(4), &Car::color_from_id(6), 1.0));

        // Choose the level according to an id.
        let level = match map_id {
            1 => Level::new("maps/race_track_1"),
            2 => Level::new("maps/race_track_2"),
            _ => Level::new("maps/race_track_1"),
        };
        let camera = Camera::new();

        let mut scene = Scene { cars, level, camera };
        scene.reset_cars();
        scene
    }

    pub fn reset_cars(&mut self) {
        for i in 0..self.cars.len() {
            self.cars[i].boost = 100.0;
            self.cars[i].rotation = zero();
            if i % 2 == 0 {
                self.cars[i].position = Vector3::new(-1.15, -1.7 * i as f32, 0.5);
            } else {
                self.cars[i].position = Vector3::new(1.15, -1.7 * i as f32, 0.5);
            }
        }
    }

    /// Update the scene.
    pub fn update(&mut self, dt: f32, controller: &[Controller], is_ingame: bool) {
        // User Input
        for (id, car) in &mut self.cars.iter_mut().enumerate() {
            if id < controller.len() {
                car.update(dt, Some(controller[id]));
            } else {
                car.update(dt, None);
            }
        }

        self.update_collisions(dt);
        self.update_scene_camera(dt, is_ingame);
    }

    /// Calculate and solve collisions.
    fn update_collisions(&mut self, dt: f32) {
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
            self.cars[i].position[2] -= 0.81 * dt;

            let penetrate_ground = query::contact(&cp, &self.cars[i].cuboid, &self.level.ground.0, &self.level.ground.1, prediction);
            if penetrate_ground.is_some() {
                let pen = penetrate_ground.unwrap();
                let w1 = pen.world1;
                let w2 = pen.world2;
                let dir = w1 - w2;
                self.cars[i].position -= dir;
            };

            let penetrate_border = query::contact(&cp, &self.cars[i].cuboid, &self.level.border.0, &self.level.border.1, prediction);
            if penetrate_border.is_some() {
                let pen = penetrate_border.unwrap();
                let w1 = pen.world1;
                let w2 = pen.world2;
                let dir = w1 - w2;
                self.cars[i].position -= dir;
            };
        }
    }

    /// Calculate the position the camera should move to.
    fn update_scene_camera(&mut self, dt: f32, is_ingame: bool) {
        if is_ingame {
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
                let camera_distance = (max - min).norm() * 1.20 + 10.0;
                self.camera.move_to_height(camera_distance);
                lerp_pos
            };
            self.camera.move_to_focus(camera_focus);
            self.camera.update(dt);
        } else {
            let height_min = self.camera.height_min;
            let is_smooth_zoom = self.camera.is_smooth_zoom;
            let is_smooth_pan = self.camera.is_smooth_pan;

            self.camera.height_min = 10.;
            self.camera.is_smooth_zoom = true;
            self.camera.is_smooth_pan = true;
            self.camera.move_to_focus(Vector3::new(0., 0., 0.));
            self.camera.move_to_height(10.);
            self.camera.update(dt);

            self.camera.height_min = height_min;
            self.camera.is_smooth_zoom = is_smooth_zoom;
            self.camera.is_smooth_pan = is_smooth_pan;
        }
    }

    /// Draw the entire `Scene` to the bound framebuffer.
    pub fn draw(&mut self, projection: &Matrix4<f32>) {
        let view = self.camera.get_viewmatrix();
        // Draw map.
        self.level.draw(&view, &projection);
        // Draw objects.
        for i in 0..self.cars.len() {
            self.cars[i].draw(&view, &projection);
        }
    }
}
