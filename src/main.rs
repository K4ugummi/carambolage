#[macro_use]
extern crate glium;
extern crate nalgebra;
extern crate time;

use glium::debug::DebugCallbackBehavior;
use glium::{glutin, Surface};
use glutin::dpi::LogicalSize;

mod game;
use game::Game;

fn print_display_info(display: &glium::Display) {
    let vendor = display.get_opengl_vendor_string();
    let version = display.get_opengl_version_string();
    let renderer = display.get_opengl_renderer_string();
    let dimensions = display.get_framebuffer_dimensions();
    println!("# INFO - Vendor:       {}", vendor);
    println!("# INFO - Renderer:     {}", renderer);
    println!("# INFO - Version:      {}", version);
    println!("# INFO - Dimension:    {:?}", dimensions);
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(LogicalSize::from((640, 480)))
        .with_title("Carambolage");

    let gl_request = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));

    let context = glutin::ContextBuilder::new()
        .with_gl(gl_request)
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl_debug_flag(true);

    let gl_window =
        glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let display =
        glium::Display::with_debug(gl_window, DebugCallbackBehavior::PrintAll)
            .unwrap();

    print_display_info(&display);

    let mut game = Game::new(&display);

    let mut should_close = false;
    while !should_close {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => should_close = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    match input.virtual_keycode {
                        Some(glutin::VirtualKeyCode::Escape) => {
                            should_close = true
                        }
                        _ => (),
                    }
                }
                _ => (),
            },
            _ => (),
        });

        // Prepere next frame.
        let mut render_target = display.draw();
        render_target.clear_color(0.05, 0.05, 0.05, 1.0);

        // Update game (physics, user input, score, ...)
        game.run();
        // Draw the current frame.
        game.draw(&mut render_target);

        render_target.finish().unwrap();
    }
}
