// This file is part of Carambolage.

// Carambolage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Carambolage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Carambolage.  If not, see <http://www.gnu.org/licenses/>.

/// GameObject, currently only a car.
mod car;
/// User input handling.
mod controller;
/// Environment of a `Scene`.
mod level;
/// Actual runtime data.
mod scene;
/// 3D translation, rotation and scale.
mod transform;

use self::controller::{Controller, ControllerLayout};
use self::scene::Scene;
use crate::grphx::Screen;
use crate::util::FrameLimiter;
use glfw::{Action, Context, Glfw, Key, Window};
use log::{debug, info};
use nalgebra::Perspective3;
use rodio::{Sink, Source};
use std::cell::Cell;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::thread::sleep;
use time::Duration;

type Event = Receiver<(f64, glfw::WindowEvent)>;

pub(crate) struct Game {
    // Glfw and GL
    glfw: Glfw,
    window: Window,
    events: Event,
    frame_limiter: FrameLimiter,

    screen: Screen,

    // Game
    settings: GameSettings,
    scene: Scene,
    controller: Vec<Controller>,
}

pub struct GameSettings {
    pub is_fullscreen: bool,
    pub width: u32,
    pub height: u32,
    pub map: u32,
    pub fps: u32,
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            is_fullscreen: false,
            width: 640,
            height: 480,
            map: 1,
            fps: 60,
        }
    }
}

impl Game {
    pub(crate) fn new(settings: GameSettings) -> Game {
        info!("Initializing game");
        let frame_limiter = FrameLimiter::new(settings.fps);

        debug!("Initializing glfw window");
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::SRgbCapable(true));
        glfw.set_error_callback(Some(glfw::Callback {
            f: error_callback,
            data: Cell::new(0),
        }));

        let (mut window, events) = glfw
            .with_primary_monitor(|glfw, m| {
                glfw.create_window(settings.width, settings.height, "Carambolage", {
                    if settings.is_fullscreen {
                        m.map_or(glfw::WindowMode::Windowed, |m| glfw::WindowMode::FullScreen(m))
                    } else {
                        glfw::WindowMode::Windowed
                    }
                })
            })
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Normal);

        debug!("Initializing openGL attributes");
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
        }

        let screen = Screen::new(settings.width, settings.height);

        let controller = vec![
            Controller::new(true, &ControllerLayout::WASD),
            Controller::new(true, &ControllerLayout::Arrows),
        ];
        let scene = Scene::new(settings.map);

        Game {
            glfw,
            window,
            events,
            frame_limiter,

            screen,

            settings,
            scene,
            controller,
        }
    }

    pub(crate) fn run(&mut self) {
        let device = rodio::default_output_device().unwrap();
        let file = File::open("res/sounds/music/The_Rush.mp3").unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();
        let mut sinc = Sink::new(&device);
        sinc.set_volume(0.5);
        sinc.append(source);

        let nano_sec = Duration::nanoseconds(1).to_std().unwrap();

        while !self.window.should_close() {
            let dt = self.frame_limiter.start();
            self.window.make_current();
            self.glfw.poll_events();
            self.process_events();
            self.process_input(dt);

            self.scene.update(dt, &self.controller);

            self.screen.first_step();
            let projection = Perspective3::new(self.settings.width as f32 / self.settings.height as f32, 70., 1.0, 200.).unwrap();
            self.scene.draw(&projection);

            self.screen.second_step();

            self.window.swap_buffers();
            while self.frame_limiter.stop() {
                self.glfw.poll_events();
                sleep(nano_sec);
            }
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(single_match))]
    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                    self.settings.width = width as u32;
                    self.settings.height = height as u32;
                    self.screen.resize(width as u32, height as u32);
                },
                _ => {}
            }
        }
    }

    pub fn process_input(&mut self, dt: f32) {
        if self.window.get_key(Key::Escape) == Action::Press {
            self.window.set_should_close(true)
        }

        for ctrl in &mut self.controller.iter_mut() {
            ctrl.process_input(&self.window, dt);
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}
