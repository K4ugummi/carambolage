use glium::Frame;
use nalgebra::Point2;
use time::Duration;

use super::car::Car;

pub(super) struct Scene {
    cars: Vec<Car>,
}

impl Scene {
    /// Make a new scene with a given number of cars.
    pub(super) fn new(cars: usize) -> Scene {
        Scene {
            cars: (0..cars)
                .map(|x| Car::new(Point2::new(x as f32, 0.), 1.0))
                .collect(),
        }
    }

    /// Update the scene based on the internal state and a given time step.
    pub(super) fn run(&mut self, time_step: Duration) {
        let time_step = (time_step.num_milliseconds() * 1_000) as f32;
        //assert!(time_step > 0.);
        for car in &mut self.cars {
            car.run(time_step);
        }
    }

    pub(super) fn draw(&self, target: &mut Frame) {
        // TODO: Draw environment.

        // Draw objects.
        for car in &self.cars {
            car.draw(target);
        }
    }
}