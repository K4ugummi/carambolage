extern crate glium;
extern crate nalgebra;
extern crate time;

use glium::{glutin, Surface};

mod game;
use game::Game;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Carambolage");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut game = Game::new();

    let mut should_close = false;
    while !should_close {
        // Prepere next frame.
        let mut render_target = display.draw();
        render_target.clear_color(0.042, 0., 0.042, 1.0);

        // Update game (physics, user input, score, ...)
        game.run();
        // Draw the current frame.
        game.draw(&mut render_target);

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
