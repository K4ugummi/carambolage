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
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.
use time::{Duration, PreciseTime};

pub struct FrameLimiter {
    time_per_frame: Duration,
    delta_time: Duration,
    dt: f32,
    time: PreciseTime,
}

impl FrameLimiter {
    pub fn new(frame_rate: u32) -> FrameLimiter {
        let time_per_frame = if frame_rate == 0 {
            Duration::nanoseconds(1)
        } else {
            Duration::microseconds((1e6 / f64::from(frame_rate)) as i64)
        };
        let time = PreciseTime::now();

        FrameLimiter {
            time_per_frame,
            delta_time: Duration::microseconds(1),
            dt: 1e-6f32,
            time,
        }
    }

    pub fn start(&mut self) -> f32 {
        let now = PreciseTime::now();
        self.time = now;

        self.dt
    }

    pub fn stop(&mut self) -> bool {
        self.delta_time = self.time.to(PreciseTime::now());
        self.dt = self.delta_time.num_microseconds().unwrap() as f32 * 1e-6f32;
        self.delta_time < self.time_per_frame
    }
}
