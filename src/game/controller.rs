use super::glfw::{Action, Context, Glfw, Key, Window};

use nalgebra::{zero, Vector2};
use time::Duration;

// Prototype controller emulation for keyboard users.
#[derive(Copy, Clone, Debug)]
pub struct Controller {
    // Settings and internal flags.
    is_smooth: bool,
    is_W: bool,
    is_A: bool,
    is_S: bool,
    is_D: bool,

    // Buttons and input axis.
    axis_goal: Vector2<f32>,
    axis: Vector2<f32>,
    button_0: bool,
    button_1: bool,
}

// DO NOT CHANGE WASD to other keys please. Setting your controls to e.g.
// arrow keys will come later. Thanks in advance, K4ugummi.
impl Controller {
    pub fn new(smooth: bool) -> Controller {
        Controller {
            is_smooth: smooth,
            is_W: false,
            is_A: false,
            is_S: false,
            is_D: false,
            axis_goal: zero(),
            axis: zero(),
            button_0: false,
            button_1: false,
        }
    }

    pub fn process_input(&mut self, window: &Window) {
        if window.get_key(Key::W) == Action::Press && !self.is_W {
            self.set_y_axis(1.);
            self.is_W = true;
        }
        if window.get_key(Key::W) == Action::Release && self.is_W {
            self.set_y_axis(0.);
            self.is_W = false;
        }
        if window.get_key(Key::S) == Action::Press && !self.is_S {
            self.set_y_axis(-1.);
            self.is_S = true;
        }
        if window.get_key(Key::S) == Action::Release && self.is_S {
            self.set_y_axis(0.);
            self.is_S = false;
        }
        if window.get_key(Key::A) == Action::Press && !self.is_A {
            self.set_x_axis(-1.);
            self.is_A = true;
        }
        if window.get_key(Key::A) == Action::Release && self.is_A {
            self.set_x_axis(0.);
            self.is_A = false;
        }
        if window.get_key(Key::D) == Action::Press && !self.is_D {
            self.set_x_axis(1.);
            self.is_D = true;
        }
        if window.get_key(Key::D) == Action::Release && self.is_D {
            self.set_x_axis(0.);
            self.is_D = false;
        }
    }

    pub fn run(&mut self, delta_time: Duration) {
        println!("Controller: {:?},{:?}", self.x_axis(), self.y_axis());
        if self.is_smooth {
            let dt = delta_time.num_milliseconds() as f32 / 1_000.;
            self.axis =
                Vector2::lerp(&self.axis, &self.axis_goal, 0.5 * dt * 10.);
            self.axis[0] = (self.axis[0] * 10_000.).trunc() / 10_000.;
            self.axis[1] = (self.axis[1] * 10_000.).trunc() / 10_000.;
        } else {
            self.axis = self.axis_goal;
        }
    }

    fn set_x_axis(&mut self, value: f32) {
        self.axis_goal[0] = value;
    }
    fn set_y_axis(&mut self, value: f32) {
        self.axis_goal[1] = value;
    }

    fn x_axis(&self) -> f32 {
        self.axis[0]
    }
    fn y_axis(&self) -> f32 {
        self.axis[1]
    }

    fn set_button_0(&mut self, is: bool) {
        self.button_0 = is;
    }

    fn set_button_1(&mut self, is: bool) {
        self.button_1 = is;
    }
}

trait Lerp {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self;
}

impl Lerp for Vector2<f32> {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self {
        a + (b - a) * factor
    }
}

impl Lerp for f32 {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self {
        a + (b - a) * factor
    }
}
