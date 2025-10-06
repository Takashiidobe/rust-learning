#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tokio::time::{self, Duration, sleep, timeout};

    // use block on to resolve a future. In this case it should take n seconds to resolve before
    // returning 42
    async fn delayed_value(seconds: u64) -> i32 {
        sleep(Duration::from_secs(seconds)).await;
        42
    }

    #[tokio::test(start_paused = true)]
    async fn test_delayed_value() {
        let fut = delayed_value(10);

        assert_eq!(fut.await, 42);
    }

    #[tokio::test(start_paused = true)]
    async fn test_timeout() {
        let res = timeout(Duration::from_secs(12), delayed_value(10)).await;

        assert_eq!(res.unwrap(), 42);
    }

    #[tokio::test(start_paused = true)]
    async fn test_timeout_err() {
        // this fails because the time is too short for the delayed value test future
        let res = timeout(Duration::from_secs(2), delayed_value(10)).await;

        // so the branch should be an error and it should be elapsed
        assert!(res.is_err());
    }

    // intervals, as opposed to sleep, allow this test to run every 2 seconds (if tokio's time
    // wasn't mocked). If this was a sleep, it would be every 3 seconds.
    #[tokio::test(start_paused = true)]
    async fn test_interval_ticks() {
        let mut interval = time::interval(Duration::from_secs(2));

        let mut total = 0;

        // Act + Assert
        for _ in 0..3 {
            interval.tick().await;
            total += delayed_value(2).await;
        }

        assert_eq!(total, 42 * 3);
    }
}
