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
use nalgebra::{Perspective3, Vector3};

use std::cell::Cell;
use std::sync::mpsc::Receiver;

type Event = Receiver<(f64, glfw::WindowEvent)>;

pub(crate) struct Game {
    glfw: Glfw,
    window: Window,
    events: Event,
    width: i32,
    height: i32,

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
        glfw.set_error_callback(Some(glfw::Callback {
            f: error_callback,
            data: Cell::new(0),
        }));

        let width = 640i32;
        let height = 480i32;
        let (mut window, events) = glfw
            .create_window(
                width as u32,
                height as u32,
                "Carambolage",
                glfw::WindowMode::Windowed,
            ).expect("Failed to create GLFW window");

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
            width,
            height,

            scene,
            time,
        }
    }

    pub(crate) fn run(&mut self) {
        let mut delta_time = self.time.to(PreciseTime::now());

        while !self.window.should_close() {
            self.window.make_current();
            self.process_events();
            self.process_input(
                delta_time.num_microseconds().unwrap() as f32 * 1e-6,
            );

            self.scene.run(delta_time);

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            let projection = Perspective3::new(
                self.width as f32 / self.height as f32,
                70.,
                0.1,
                1000.,
            ).unwrap();
            self.scene.draw(&projection);

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

    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                    self.width = width;
                    self.height = height;
                },
                _ => {}
            }
        }
    }

    pub fn process_input(&mut self, delta_time: f32) {
        if self.window.get_key(Key::Escape) == Action::Press {
            self.window.set_should_close(true)
        }

        if self.window.get_key(Key::W) == Action::Press {
            self.scene.cars[0].pos +=
                Vector3::new(0f32, 1., 0.) * delta_time * 10.;
        }
        if self.window.get_key(Key::S) == Action::Press {
            self.scene.cars[0].pos +=
                Vector3::new(0f32, -1., 0.) * delta_time * 10.;
        }
        if self.window.get_key(Key::A) == Action::Press {
            self.scene.cars[0].pos +=
                Vector3::new(-1f32, 0., 0.) * delta_time * 10.;
        }
        if self.window.get_key(Key::D) == Action::Press {
            self.scene.cars[0].pos +=
                Vector3::new(1f32, 0., 0.) * delta_time * 10.;
        }
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
