use std::time::Duration;

pub const POLL_RATE: Duration = Duration::from_millis(100);
pub const TICK_RATE: Duration = Duration::from_millis(100);

pub const WORLD_MIN: f64 = 0.0;
pub const WORLD_MAX: f64 = 5.0;

pub const SNAKE_BOX_SIZE: f64 = 0.2;
pub const SNAKE_STEP_SIZE: f64 = 0.1;
pub const INITIAL_SNAKE_LENGTH: usize = 3;
