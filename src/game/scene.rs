use nalgebra::{inf, sup, zero, Matrix4, Point3, Vector3};
use rand::{thread_rng, Rng};
use time::Duration;

use super::car::Car;

pub(super) struct Scene {
    pub cars: Vec<Car>,
}

impl Scene {
    /// Make a new scene with a given number of cars.
    pub(super) fn new(num_cars: usize) -> Scene {
        let mut rng = thread_rng();
        let mut cars: Vec<Car> = (0..num_cars)
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

        assert!(num_cars > 0);
        cars[0].model.meshes[0].color = Vector3::new(1., 0., 0.);

        Scene { cars }
    }

    /// Update the scene based on the internal state and a given time step.
    pub(super) fn run(&mut self, time_step: Duration) {
        let time_step = (time_step.num_milliseconds() * 1_000) as f32;
        for car in &mut self.cars {
            car.run(time_step);
        }
    }

    pub(super) fn draw(&self, projection: &Matrix4<f32>) {
        assert!(self.cars.len() > 0);
        let mut min = self.cars[0].pos;
        let mut max = self.cars[0].pos;
        let mut camera_pos = zero();
        for car in &self.cars {
            camera_pos += car.pos;
            min = inf(&min, &car.pos);
            max = sup(&max, &car.pos);
        }
        camera_pos /= self.cars.len() as f32;
        let camera_distance = (max - min).norm();

        let view = Matrix4::look_at_rh(
            &Point3::from_coordinates(
                camera_pos + Vector3::new(0., 0., camera_distance + 5.),
            ),
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
