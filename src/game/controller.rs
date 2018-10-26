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
use super::glfw::{Action, Key, Window};

use nalgebra::{zero, Vector2};
use time::Duration;
use util::Lerp;

use self::ControllerInternal as CI;
use self::ControllerLayout as CL;

#[derive(Debug)]
pub enum ControllerLayout {
    WASD,
    Arrows,
}

#[derive(Copy, Clone, Debug)]
struct ControllerInternal {
    forward: Key,
    is_forward: bool,
    backward: Key,
    is_backward: bool,
    left: Key,
    is_left: bool,
    right: Key,
    is_right: bool,
}

impl ControllerInternal {
    pub fn new(controller_layout: &ControllerLayout) -> ControllerInternal {
        match controller_layout {
            CL::WASD => Default::default(),
            CL::Arrows => CI {
                forward: Key::Up,
                backward: Key::Down,
                left: Key::Left,
                right: Key::Right,
                ..Default::default()
            },
        }
    }
}

impl Default for ControllerInternal {
    fn default() -> ControllerInternal {
        ControllerInternal {
            forward: Key::W,
            is_forward: false,
            backward: Key::S,
            is_backward: false,
            left: Key::A,
            is_left: false,
            right: Key::D,
            is_right: false,
        }
    }
}

// Prototype controller emulation for keyboard users.
#[derive(Copy, Clone, Debug)]
pub struct Controller {
    // Internal settings and flags.
    is_smooth: bool,
    ci: ControllerInternal,
    axis_goal: Vector2<f32>,

    // Buttons and input axis that can be used in the game.
    axis: Vector2<f32>,
}

// DO NOT CHANGE WASD to other keys please. Setting your controls to e.g.
// arrow keys will come later. Thanks in advance, K4ugummi.
impl Controller {
    pub fn new(smooth: bool, controller_layout: &ControllerLayout) -> Controller {
        debug!("New smooth: {}, layout: {:?}", smooth, controller_layout);
        Controller {
            is_smooth: smooth,
            ci: ControllerInternal::new(&controller_layout),
            axis_goal: zero(),
            axis: zero(),
        }
    }

    pub fn process_input(&mut self, window: &Window, delta_time: &Duration) {
        if window.get_key(self.ci.forward) == Action::Press && !self.ci.is_forward {
            self.set_y_axis(1.);
            self.ci.is_forward = true;
        }
        if window.get_key(self.ci.forward) == Action::Release && self.ci.is_forward {
            self.set_y_axis(0.);
            self.ci.is_forward = false;
        }
        if window.get_key(self.ci.backward) == Action::Press && !self.ci.is_backward {
            self.set_y_axis(-1.);
            self.ci.is_backward = true;
        }
        if window.get_key(self.ci.backward) == Action::Release && self.ci.is_backward {
            self.set_y_axis(0.);
            self.ci.is_backward = false;
        }
        if window.get_key(self.ci.left) == Action::Press && !self.ci.is_left {
            self.set_x_axis(-1.);
            self.ci.is_left = true;
        }
        if window.get_key(self.ci.left) == Action::Release && self.ci.is_left {
            self.set_x_axis(0.);
            self.ci.is_left = false;
        }
        if window.get_key(self.ci.right) == Action::Press && !self.ci.is_right {
            self.set_x_axis(1.);
            self.ci.is_right = true;
        }
        if window.get_key(self.ci.right) == Action::Release && self.ci.is_right {
            self.set_x_axis(0.);
            self.ci.is_right = false;
        }

        self.update(delta_time);
    }

    fn update(&mut self, delta_time: &Duration) {
        if self.is_smooth {
            let dt = delta_time.num_milliseconds() as f32 / 1_000.;
            self.axis = Vector2::lerp(&self.axis, &self.axis_goal, 5. * dt);
            self.axis[0] = (self.axis[0] * 10_000.).trunc() / 10_000.;
            self.axis[1] = (self.axis[1] * 10_000.).trunc() / 10_000.;
        } else {
            self.axis = self.axis_goal;
        }
    }

    pub fn get_x_axis(&self) -> f32 {
        self.axis[0]
    }

    pub fn get_y_axis(&self) -> f32 {
        self.axis[1]
    }

    fn set_x_axis(&mut self, value: f32) {
        self.axis_goal[0] = value;
    }

    fn set_y_axis(&mut self, value: f32) {
        self.axis_goal[1] = value;
    }
}
