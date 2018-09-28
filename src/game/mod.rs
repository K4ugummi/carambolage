extern crate gl;
extern crate glfw;

mod car;
mod mesh;
mod model;
mod scene;
mod shader;

use self::car::Car;
use self::glfw::{Action, Context, Glfw, Key, Window};
use self::scene::Scene;
use super::time::{Duration, PreciseTime};
use nalgebra::Vector3;

use std::cell::Cell;
use std::sync::mpsc::Receiver;

type Event = Receiver<(f64, glfw::WindowEvent)>;

pub(crate) struct Game {
    glfw: Glfw,
    window: Window,
    events: Event,

    scene: Scene,
    time: PreciseTime,
}

impl Game {
    pub(crate) fn new() -> Game {
        let time = PreciseTime::now();

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.set_error_callback(Some(glfw::Callback {
            f: error_callback,
            data: Cell::new(0),
        }));

        let (mut window, events) = glfw
            .create_window(640, 480, "Carambolage", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Normal);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let scene = Scene::new(3);

        Game {
            glfw,
            window,
            events,
            scene,
            time,
        }
    }

    pub(crate) fn run(&mut self) {
        let mut delta_time = self.time.to(PreciseTime::now());

        while !self.window.should_close() {
            process_events(&self.events);

            process_input(
                &mut self.window,
                delta_time.num_microseconds().unwrap() as f32 * 1e-6,
                &mut self.scene.cars[0],
            );

            self.scene.run(delta_time);

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.window.make_current();
            self.scene.draw();

            let time_now = PreciseTime::now();
            delta_time = self.time.to(time_now);
            self.time = time_now;
            self.do_delta_time_sleep(delta_time);

            self.window.swap_buffers();
            self.glfw.poll_events();
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

pub fn process_events(events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            _ => {}
        }
    }
}

pub fn process_input(
    window: &mut glfw::Window,
    delta_time: f32,
    car: &mut Car,
) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(Key::W) == Action::Press {
        car.pos += Vector3::new(0f32, 1., 0.) * delta_time * 10.;
    }
    if window.get_key(Key::S) == Action::Press {
        car.pos += Vector3::new(0f32, -1., 0.) * delta_time * 10.;
    }
    if window.get_key(Key::A) == Action::Press {
        car.pos += Vector3::new(-1f32, 0., 0.) * delta_time * 10.;
    }
    if window.get_key(Key::D) == Action::Press {
        car.pos += Vector3::new(1f32, 0., 0.) * delta_time * 10.;
    }
}

fn error_callback(
    _: glfw::Error,
    description: String,
    error_count: &Cell<usize>,
) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}
