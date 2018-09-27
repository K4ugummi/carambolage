mod car;
mod model;
mod scene;
mod shader;

use self::scene::Scene;

use super::glium::debug::DebugCallbackBehavior;
use super::glium::glutin::dpi::LogicalSize;
use super::glium::{glutin, Display, Frame, Surface};
use super::time::{Duration, PreciseTime};

pub(crate) struct Game {
    events_loop: glutin::EventsLoop,
    display: Display,

    scene: Scene,
    time: PreciseTime,
}

impl Game {
    pub(crate) fn new() -> Game {
        let time = PreciseTime::now();

        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions(LogicalSize::from((640, 480)))
            .with_resizable(false)
            .with_title("Carambolage");

        let gl_request =
            glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));

        let context = glutin::ContextBuilder::new()
            .with_gl(gl_request)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl_debug_flag(true);

        let gl_window =
            glutin::GlWindow::new(window, context, &events_loop).unwrap();

        let display =
            Display::with_debug(gl_window, DebugCallbackBehavior::PrintAll)
                .unwrap();

        let scene = Scene::new(3, &display);

        print_display_info(&display);

        Game {
            events_loop,
            display,
            scene,
            time,
        }
    }

    pub(crate) fn run(&mut self) {
        let mut keep_running = true;
        let mut delta_time = self.time.to(PreciseTime::now());

        while keep_running {
            self.events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => keep_running = false,
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode {
                            Some(glutin::VirtualKeyCode::Escape) => {
                                keep_running = false;
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                },
                _ => (),
            });

            // Prepere next frame.
            let mut render_target = self.display.draw();
            render_target.clear_color(0.05, 0.05, 0.05, 1.0);

            // Update game (physics, user input, score, ...)
            self.scene.run(delta_time);
            // Draw the current frame.
            self.scene.draw(&mut render_target);

            render_target.finish().unwrap();

            let time_now = PreciseTime::now();
            delta_time = self.time.to(time_now);
            self.time = time_now;
            self.do_delta_time_sleep(delta_time);
        }
    }

    fn do_delta_time_sleep(&mut self, delta_time: Duration) {
        use std::thread::sleep;
        let time_per_frame = Duration::nanoseconds(16666666);

        if delta_time < time_per_frame {
            let sleep_time = time_per_frame
                .checked_sub(&delta_time)
                .unwrap()
                .to_std()
                .unwrap();
            sleep(sleep_time);
        }
    }
}

fn print_display_info(display: &Display) {
    let vendor = display.get_opengl_vendor_string();
    let version = display.get_opengl_version_string();
    let renderer = display.get_opengl_renderer_string();
    let dimensions = display.get_framebuffer_dimensions();
    println!("# INFO - Vendor:       {}", vendor);
    println!("# INFO - Renderer:     {}", renderer);
    println!("# INFO - Version:      {}", version);
    println!("# INFO - Dimension:    {:?}", dimensions);
}
