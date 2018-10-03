use std::thread::sleep;
use time::{Duration, PreciseTime};

pub struct FrameLimiter {
    time_per_frame: Duration,
    delta_time: Duration,
    time: PreciseTime,
}

impl FrameLimiter {
    pub fn new(frame_rate: u32) -> FrameLimiter {
        let time_per_frame = Duration::nanoseconds((1e9 / f64::from(frame_rate)) as i64);
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
