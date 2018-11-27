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
use self::ControllerInternal as CI;
use self::ControllerLayout as CL;
use glfw::{Action, Key, Window};
use log::debug;
use nalgebra::{zero, Vector2};
use util::Lerp;

/// Control with WASD or arrow keys.
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

    boost: Key,
    is_boost: bool,
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
                boost: Key::RightShift,
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
            boost: Key::LeftShift,
            is_boost: false,
        }
    }
}

/// Prototype controller emulation for keyboard users.
#[derive(Copy, Clone, Debug)]
pub struct Controller {
    /// Internal settings and flags.
    is_smooth: bool,
    ci: ControllerInternal,
    axis_goal: Vector2<f32>,

    /// Buttons and input axis that can be used in the game.
    axis: Vector2<f32>,
    boost: bool,
}

impl Controller {
    /// Create a new controller. You can activate smooth axis interpolation for up, left, down, right
    /// by setting `smooth`to true.
    pub fn new(smooth: bool, controller_layout: &ControllerLayout) -> Controller {
        debug!("New smooth: {}, layout: {:?}", smooth, controller_layout);
        Controller {
            is_smooth: smooth,
            ci: ControllerInternal::new(&controller_layout),
            axis_goal: zero(),
            axis: zero(),
            boost: false,
        }
    }

    /// Process input keys for this controller.
    ///
    /// The input is handled from glfw::Window due to event polling.
    pub fn process_input(&mut self, window: &Window, dt: f32) {
        if window.get_key(self.ci.forward) == Action::Press && !self.ci.is_forward {
            self.set_y_axis(1.);
            self.ci.is_forward = true;
        } else if window.get_key(self.ci.forward) == Action::Release && self.ci.is_forward {
            self.set_y_axis(0.);
            self.ci.is_forward = false;
        }
        if window.get_key(self.ci.backward) == Action::Press && !self.ci.is_backward {
            self.set_y_axis(-1.);
            self.ci.is_backward = true;
        } else if window.get_key(self.ci.backward) == Action::Release && self.ci.is_backward {
            self.set_y_axis(0.);
            self.ci.is_backward = false;
        }
        if window.get_key(self.ci.left) == Action::Press && !self.ci.is_left {
            self.set_x_axis(-1.);
            self.ci.is_left = true;
        } else if window.get_key(self.ci.left) == Action::Release && self.ci.is_left {
            self.set_x_axis(0.);
            self.ci.is_left = false;
        }
        if window.get_key(self.ci.right) == Action::Press && !self.ci.is_right {
            self.set_x_axis(1.);
            self.ci.is_right = true;
        } else if window.get_key(self.ci.right) == Action::Release && self.ci.is_right {
            self.set_x_axis(0.);
            self.ci.is_right = false;
        }
        if window.get_key(self.ci.boost) == Action::Press && !self.ci.is_boost {
            self.boost = true;
            self.ci.is_boost = true;
        } else if window.get_key(self.ci.boost) == Action::Release && self.ci.is_boost {
            self.boost = false;
            self.ci.is_boost = false;
        }

        if self.is_smooth {
            self.axis = Vector2::lerp(&self.axis, &self.axis_goal, 5. * dt);
            self.axis[0] = (self.axis[0] * 10_000.).trunc() / 10_000.;
            self.axis[1] = (self.axis[1] * 10_000.).trunc() / 10_000.;
        } else {
            self.axis = self.axis_goal;
        }
    }

    /// Return the x axis value, clamped between [-1.0f32; 1.0f32].
    pub fn get_x_axis(&self) -> f32 {
        self.axis[0]
    }

    /// Return the y axis value, clamped between [-1.0f32; 1.0f32].
    pub fn get_y_axis(&self) -> f32 {
        self.axis[1]
    }

    /// Return true if the boost button is pressed.
    pub fn get_boost(&self) -> bool {
        self.boost
    }

    /// Sets the x axis value to make `process_input` better readable.
    fn set_x_axis(&mut self, value: f32) {
        self.axis_goal[0] = value;
    }

    /// Sets the y axis value to make `process_input` better readable.
    fn set_y_axis(&mut self, value: f32) {
        self.axis_goal[1] = value;
    }
}
