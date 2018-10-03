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
use std::thread::sleep;
use time::{Duration, PreciseTime};

pub struct FrameLimiter {
    time_per_frame: Duration,
    delta_time: Duration,
    time: PreciseTime,
}

impl FrameLimiter {
    pub fn new(frame_rate: u32) -> FrameLimiter {
        let time_per_frame = if frame_rate == 0 {
            Duration::nanoseconds(1)
        } else {
            Duration::nanoseconds((1e9 / f64::from(frame_rate)) as i64)
        };
        let time = PreciseTime::now();

        FrameLimiter {
            time_per_frame,
            delta_time: Duration::nanoseconds(0),
            time,
        }
    }

    pub fn start(&mut self) -> Duration {
        let now = PreciseTime::now();
        self.delta_time = self.time.to(now);
        self.time = now;

        self.delta_time
    }

    pub fn stop(&mut self) {
        if self.delta_time < self.time_per_frame {
            let sleep_time = self.time_per_frame.checked_sub(&self.delta_time).unwrap().to_std().unwrap();
            sleep(sleep_time);
        }
    }
}
