extern crate glium;
extern crate nalgebra;
extern crate rand;

mod car;
mod model;
mod scene;
mod shader;

use self::scene::*;

use glium::glutin::ControlFlow;
use glium::Frame;

use time::{Duration, PreciseTime};

pub(crate) struct Game {
    scene: Scene,
    time: PreciseTime,
}

impl Game {
    pub(crate) fn new(display: &glium::Display) -> Game {
        Game {
            scene: Scene::new(3, display),
            time: PreciseTime::now(),
        }
    }

    fn update_time(&mut self) -> Duration {
        let time_now = PreciseTime::now();
        let time_step = self.time.to(time_now);
        self.time = time_now;

        time_step
    }

    pub(crate) fn run(&mut self) -> ControlFlow {
        let time_step = self.update_time();
        self.scene.run(time_step);
        ControlFlow::Continue
    }

    pub fn draw(&self, target: &mut Frame) {
        // TODO: Draw UI

        // Draw game scene
        self.scene.draw(target);
    }
}
