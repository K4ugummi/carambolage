use super::model::Model;
use nalgebra::geometry::Translation;
use nalgebra::{zero, Matrix4, Vector3};

pub struct Car {
    pub pos: Vector3<f32>,
    vel: Vector3<f32>,
    force: Vector3<f32>,
    mass: f32,

    model: Model,
}

impl Car {
    pub fn new(pos: Vector3<f32>, mass: f32) -> Car {
        assert!(mass > 0.);
        Car {
            pos,
            vel: zero(),
            force: zero(),
            mass,

            model: Model::new(),
        }
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    pub(super) fn run(&mut self, time_step: f32) {
        //assert!(time_step > 0.);
        //self.pos += self.vel * time_step
        //    + self.force / (2. * self.mass) * time_step.powi(2);
        //self.vel += self.force / self.mass * time_step;
    }

    pub(super) fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        let model = Translation::from_vector(self.pos).to_homogeneous();
        let mvp = projection * view * model;
        self.model.draw(&mvp);
    }
}
