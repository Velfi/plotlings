use std::time::{Duration, Instant};

pub struct Interval {
    cycles: usize,
    start_of_current_cycle: Instant,
    cycle_duration: Duration,
}

impl Interval {
    pub fn new(cycle_duration: Duration) -> Self {
        Self {
            start_of_current_cycle: Instant::now(),
            cycles: 0,
            cycle_duration,
        }
    }

    pub fn new_from_seconds(seconds: u64) -> Self {
        Self::new(Duration::from_secs(seconds))
    }

    // Returns true if a cycle has occurred since the last time tick was called
    pub fn tick(&mut self) -> bool {
        if self.start_of_current_cycle.elapsed() > self.cycle_duration {
            self.cycles += 1;
            self.start_of_current_cycle = Instant::now();

            true
        } else {
            false
        }
    }
}
