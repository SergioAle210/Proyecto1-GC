use std::time::{Duration, Instant};

pub struct Timer {
    last_frame: Instant,
    delta_time: Duration,
    fps: f32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            delta_time: Duration::new(0, 0),
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
        self.fps = 1.0 / self.delta_time.as_secs_f32();
    }

    pub fn get_fps(&self) -> f32 {
        let fps = self.fps;
        if fps > 60.0 {
            60.0
        } else {
            fps
        }
    }
}
