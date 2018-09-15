extern crate glium;
extern crate nalgebra;
extern crate time;

use glium::{glutin, Surface};
use glutin::{ControlFlow, Event, WindowEvent};

mod scene;
use scene::Scene;

use time::{Duration, PreciseTime};

struct Game {
    scene: Scene,
    time: PreciseTime,
}

impl Game {
    fn new() -> Game {
        Game {
            scene: Scene::new(3),
            time: PreciseTime::now(),
        }
    }

    fn update_time(&mut self) -> Duration {
        let time_now = PreciseTime::now();
        let time_step = self.time.to(time_now);
        self.time = time_now;

        time_step
    }

    fn run(&mut self) -> ControlFlow {
        let time_step = self.update_time();
        self.scene.run(time_step);
        ControlFlow::Continue
    }

    fn handle_events(&mut self, event: Event) -> ControlFlow {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => ControlFlow::Break,
                _ => self.run(),
            },
            _ => self.run(),
        }
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut game = Game::new();

    let mut should_close = false;
    while !should_close {
        let mut render_target = display.draw();

        render_target.clear_color(0.042, 0., 0.042, 1.0);

        // Update scene
        game.run();

        render_target.finish().unwrap();

        // Handle events
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => should_close = true,
                _ => (),
            },
            _ => (),
        });
    }
}
