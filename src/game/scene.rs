use nalgebra::{zero, Matrix4, Point3, Vector2, Vector3};
use rand::{thread_rng, Rng};
use time::Duration;

use super::car::Car;

pub(super) struct Scene {
    pub cars: Vec<Car>,
}

impl Scene {
    /// Make a new scene with a given number of cars.
    pub(super) fn new(cars: usize) -> Scene {
        let mut rng = thread_rng();

        Scene {
            cars: (0..cars)
                .map(|x| {
                    Car::new(
                        {
                            let x = rng.gen_range(-20f32, 20f32);
                            let y = rng.gen_range(-20f32, 20f32);
                            Vector3::new(x, y, 0.)
                        },
                        1.0,
                    )
                }).collect(),
        }
    }

    /// Update the scene based on the internal state and a given time step.
    pub(super) fn run(&mut self, time_step: Duration) {
        let time_step = (time_step.num_milliseconds() * 1_000) as f32;
        for car in &mut self.cars {
            car.run(time_step);
        }
    }

    pub(super) fn draw(&self, projection: &Matrix4<f32>) {
        let mut camera_pos = zero();
        for car in &self.cars {
            camera_pos += car.pos;
        }
        camera_pos /= self.cars.len() as f32;

        let view = Matrix4::look_at_rh(
            &Point3::from_coordinates(camera_pos + Vector3::new(0., 0., 100.)),
            &Point3::from_coordinates(camera_pos),
            &Vector3::y_axis(),
        );

        unsafe {
            static mut FIRST: bool = true;

            // Draw objects.
            for i in 0..self.cars.len() {
                if FIRST {
                    println!("Car_{}: position {}", i, self.cars[i].pos);
                }
                self.cars[i].draw(&view, &projection);
            }
            FIRST = false;
        }
    }
}
