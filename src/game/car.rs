use super::glium;
use super::glium::{Frame, Surface};
use super::model::Model;
use super::nalgebra::{Matrix4, Point2, Vector2, Vector3};

pub(super) struct Car {
    pos: Point2<f32>,
    _rot: f32,
    vel: Vector2<f32>,
    force: Vector2<f32>,
    mass: f32,

    model: Model,
}

impl Car {
    pub fn new(
        pos: Point2<f32>,
        mass: f32,
        color: Vector3<f32>,
        display: &glium::Display,
    ) -> Car {
        assert!(mass > 0.);
        Car {
            pos,
            _rot: 0.,
            vel: Vector2::new(0., 0.),
            force: Vector2::new(0., 0.),
            mass,

            model: Model::new(color, display),
        }
    }

    /// Update the car position and velocity based on the internal car state for
    /// a given time step.
    pub(super) fn run(&mut self, time_step: f32) {
        //assert!(time_step > 0.);
        self.pos += self.vel * time_step
            + self.force / (2. * self.mass) * time_step.powi(2);
        self.vel += self.force / self.mass * time_step;
    }

    pub(super) fn draw(
        &self,
        target: &mut Frame,
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
    ) {
        // Convert nalgebra structs to arrays.
        let model_view_projection =
            (projection * view * self.model.matrix).as_ref().clone();
        let color_ref = self.model.color.as_ref().clone();

        // Write uniforms
        let uniforms = uniform! {
            uMVP: model_view_projection,
            uColor: color_ref,
        };

        target
            .draw(
                &self.model.vertex_buffer,
                &self.model.index_buffer,
                &self.model.program,
                &uniforms,
                &Default::default(),
            ).unwrap();
    }
}
