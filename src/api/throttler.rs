use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// The `Throttler` struct manages rate limiting for API requests.
pub struct Throttler {
    rate_limit: u32,
    interval: Duration,
    state: Mutex<ThrottlerState>,
}

/// The `ThrottlerState` struct holds the state for the `Throttler`.
struct ThrottlerState {
    request_count: u32,
    interval_start: Instant,
}

impl Throttler {
    /// Creates a new `Throttler` with the specified rate limit and interval (in seconds).
    pub fn new(rate_limit: u32, interval: u64) -> Self {
        let interval = Duration::from_secs(interval);
        Throttler {
            rate_limit,
            interval,
            state: Mutex::new(ThrottlerState {
                request_count: 0,
                interval_start: Instant::now() - interval,
            }),
        }
    }

    /// Throttles the requests to ensure the rate limit is not exceeded.
    pub async fn throttle(&self) {
        let mut state = self.state.lock().await;
        let now = Instant::now();

        // Check if the rate limit has been exceeded
        if state.request_count < self.rate_limit {
            state.request_count += 1;
        // Check if the interval has passed
        } else if now.duration_since(state.interval_start) >= self.interval {
            state.interval_start = now;
            state.request_count = 1;
        // Sleep until the interval has passed if the rate limit has been exceeded
        } else {
            let delay = state.interval_start + self.interval - now;
            if delay > Duration::ZERO {
                sleep(delay).await;
                state.interval_start = Instant::now();
                state.request_count = 1;
            }
        }
    }
}
