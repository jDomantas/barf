use std::time::Instant;

const FPS: f32 = 60.0;
const TIME_PER_FRAME: f32 = 1.0 / FPS;
const MAX_TIME_BACKLOG: f32 = 0.5;

pub(crate) struct Timer {
    remaining: f32,
    last_timestamp: Instant,
}

impl Timer {
    pub(crate) fn new() -> Timer {
        Timer {
            remaining: 0.0,
            last_timestamp: Instant::now(),
        }
    }

    pub(crate) fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_timestamp);
        self.last_timestamp = now;
        let elapsed = elapsed.as_secs_f32();
        self.remaining += elapsed;
        if self.remaining > MAX_TIME_BACKLOG {
            self.remaining = MAX_TIME_BACKLOG;
        }
    }

    pub(crate) fn should_update(&mut self) -> bool {
        if self.remaining >= TIME_PER_FRAME {
            self.remaining -= TIME_PER_FRAME;
            true
        } else {
            false
        }
    }
}
