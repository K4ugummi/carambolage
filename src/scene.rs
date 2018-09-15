use nalgebra::{Point2, Vector2};

use time::Duration;

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
        assert!(time_step > 0.);
        for car in &mut self.cars {
            car.run(time_step);
        }
    }
}

pub(super) struct Car {
    pos: Point2<f32>,
    vel: Vector2<f32>,
    force: Vector2<f32>,
    mass: f32,
}

impl Car {
    pub(super) fn new(pos: Point2<f32>, mass: f32) -> Car {
        assert!(mass > 0.);
        Car {
            pos,
            vel: Vector2::new(0., 0.),
            force: Vector2::new(0., 0.),
            mass,
        }
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    fn run(&mut self, time_step: f32) {
        assert!(time_step > 0.);
        self.pos += self.vel * time_step
            + self.force / (2. * self.mass) * time_step.powi(2);
        self.vel += self.force / self.mass * time_step;
    }
}

#[cfg(test)]
mod car_test {
    use super::Car;
    use nalgebra::{Point2, Vector2};

    #[test]
    fn go_car_1() {
        let mut car = Car {
            pos: Point2::new(0., 0.),
            vel: Vector2::new(1., 0.),
            force: Vector2::new(0., 0.),
            mass: 1.,
        };
        car.run(1.);
        assert_eq!(car.pos, Point2::new(1., 0.));
    }

    #[test]
    fn go_car_2() {
        let mut car = Car {
            pos: Point2::new(0., 0.),
            vel: Vector2::new(0., 0.),
            force: Vector2::new(1., 0.),
            mass: 1.,
        };
        car.run(2.);
        assert_eq!(car.pos, Point2::new(2., 0.));
        assert_eq!(car.vel, Vector2::new(2., 0.));
    }
}
