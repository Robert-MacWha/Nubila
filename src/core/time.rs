use std::time::{Duration, Instant};

pub struct Time {
    start_time: Instant,
    last_frame_time: Instant,
    delta_time: Duration,
    total_frames: u64,
}

impl Time {
    pub fn new() -> Self {
        let start_time = Instant::now();
        Time {
            start_time,
            last_frame_time: start_time,
            delta_time: Duration::from_secs(0),
            total_frames: 0,
        }
    }

    pub fn update(&mut self) {
        let current_time = Instant::now();
        self.delta_time = current_time - self.last_frame_time;
        self.last_frame_time = current_time;
        self.total_frames += 1;
    }

    // returns the time since the start of the game
    pub fn elapsed_time(&self) -> Duration {
        return self.start_time.elapsed();
    }

    // returns the time since the last frame
    pub fn delta_time(&self) -> Duration {
        return self.delta_time;
    }

    // returns the current framerate
    pub fn fps(&self) -> f64 {
        return 1.0 / self.delta_time.as_secs_f64();
    }

    // returns the average framerate this game
    pub fn average_fps(&self) -> f64 {
        return self.total_frames as f64 / self.elapsed_time().as_secs_f64();
    }
}
