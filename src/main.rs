extern crate glium;
extern crate nalgebra;
extern crate time;

use glium::debug::DebugCallbackBehavior;
use glium::{glutin, Surface};

mod game;
use game::Game;

fn main() {
    println!("### Starting Carambolage ###");
    println!("# Initializing OpenGL");
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Carambolage");
    let gl_request = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));
    let context = glutin::ContextBuilder::new()
        .with_gl(gl_request)
        .with_gl_profile(glutin::GlProfile::Core);
    let gl_window =
        glutin::GlWindow::new(window, context, &events_loop).unwrap();
    let debug = DebugCallbackBehavior::PrintAll;
    let display = glium::Display::with_debug(gl_window, debug).unwrap();

    // Print debug stuff
    let version = display.get_opengl_version();
    println!(
        "# OpenGL version (API, MAJOR, MINOR): ({:?}, {}, {})",
        version.0, version.1, version.2
    );

    println!("# Initializing game");
    let mut game = Game::new(&display);

    println!("### Starting game loop ###");
    let mut should_close = false;
    while !should_close {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => should_close = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => match input
                    .virtual_keycode
                {
                    Some(glutin::VirtualKeyCode::Escape) => should_close = true,
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        });
        // Prepere next frame.
        let mut render_target = display.draw();
        render_target.clear_color(0.042, 0., 0.042, 1.0);

        // Update game (physics, user input, score, ...)
        game.run();
        // Draw the current frame.
        game.draw(&mut render_target);

        render_target.finish().unwrap();
    }
}
