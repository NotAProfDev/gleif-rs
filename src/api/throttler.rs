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
        log::debug!(
            "Creating Throttler with rate_limit: {}, interval: {:?}",
            rate_limit,
            interval
        );
        Throttler {
            rate_limit,
            interval,
            state: Mutex::new(ThrottlerState {
                request_count: 0,
                interval_start: Instant::now(),
            }),
        }
    }

    /// Throttles the requests to ensure the rate limit is not exceeded.
    pub async fn throttle(&self) {
        let mut state = self.state.lock().await;
        let now = Instant::now();

        // Reset the interval start if no requests have been made
        if state.request_count == 0 {
            log::debug!("Resetting interval start and increasing request count to 1");
            state.interval_start = now;
            state.request_count += 1;
        }
        // Check if the rate limit has been exceeded
        else if state.request_count < self.rate_limit {
            state.request_count += 1;
            log::debug!("Increased Request count to {}", state.request_count);
        // Check if the interval has passed
        } else if now.duration_since(state.interval_start) >= self.interval {
            log::debug!("Interval passed, resetting interval start and request count");
            state.interval_start = now;
            state.request_count = 0;
        // Sleep until the interval has passed if the rate limit has been exceeded
        } else {
            let delay = state.interval_start + self.interval - now;
            log::debug!("Rate limit exceeded, sleeping for {:?}", delay);
            if delay > Duration::ZERO {
                sleep(delay).await;
                state.interval_start = Instant::now();
                state.request_count = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_throttler_new() {
        let rate_limit = 10;
        let interval = 60; // 60 seconds

        let throttler = Throttler::new(rate_limit, interval);

        // Verify that the throttler is initialized with the correct rate limit and interval
        assert_eq!(throttler.rate_limit, rate_limit);
        assert_eq!(throttler.interval, Duration::from_secs(interval));

        // Verify that the initial state is correct
        let state = throttler.state.lock().await;
        assert_eq!(state.request_count, 0);
        assert!(state.interval_start.elapsed() >= Duration::from_secs(0));
    }

    #[tokio::test]
    async fn test_throttler_throttle() {
        let rate_limit = 2;
        let interval = 5; // 5 second

        let throttler = Throttler::new(rate_limit, interval);

        // First request should pass immediately
        let start = Instant::now();
        throttler.throttle().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed < Duration::from_millis(100),
            "First request elapsed: {:?}",
            elapsed
        );

        // Second request should pass immediately
        let start = Instant::now();
        throttler.throttle().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed < Duration::from_millis(100),
            "Second request elapsed: {:?}",
            elapsed
        );

        // Third request should be throttled
        let start = Instant::now();
        throttler.throttle().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed >= Duration::from_secs(interval - 1)
                && elapsed <= Duration::from_secs(interval + 1),
            "Third request elapsed: {:?}",
            elapsed
        );

        // Wait for the interval to pass
        sleep(Duration::from_secs(1)).await;

        // Fourth request should pass immediately after the interval
        let start = Instant::now();
        throttler.throttle().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed < Duration::from_millis(100),
            "Fourth request elapsed: {:?}",
            elapsed
        );
    }
}
