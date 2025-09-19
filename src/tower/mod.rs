#[cfg(test)]
mod tests {
    // tower provides a trait for writing networked services:
    // trait Service<Request> {
    //  type Response;
    //  type Error;
    //  type Future: Future<Output = Result<Self::Response, Self::Error>>;
    //
    //  // poll_ready returns if the service can accept a request
    //  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    //  // call can perform the request which returns a future
    //  fn call(&mut self, req: Request) -> Self::Future;
    // }

    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };
    use tower::{Service, ServiceExt};

    struct DoubleService;

    // You can implement Tower services and use them async:
    impl Service<i32> for DoubleService {
        type Response = i32;
        type Error = ();
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), ()>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: Self::Response) -> Self::Future {
            Box::pin(async move { Ok(req * 2) })
        }
    }

    // this service just doubles its input.
    #[tokio::test]
    async fn test_basic_service() {
        let mut svc = DoubleService;
        let resp = svc.ready().await.unwrap().call(21).await.unwrap();
        assert_eq!(resp, 42);
    }

    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use tokio::time::{Duration, Instant};

    /// Token bucket state for a single client
    struct TokenBucketState {
        tokens: u32,
        last_refill: Instant,
    }

    /// Multi-client token bucket middleware
    pub struct MultiRateLimiter<S> {
        inner: S,
        capacity: u32,
        refill_interval: Duration,
        state: Arc<Mutex<HashMap<String, TokenBucketState>>>,
    }

    impl<S> MultiRateLimiter<S> {
        pub fn new(inner: S, capacity: u32, refill_interval: Duration) -> Self {
            Self {
                inner,
                capacity,
                refill_interval,
                state: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn try_acquire(&self, client_id: &str) -> bool {
            let mut state_map = self.state.lock().unwrap();
            let now = Instant::now();

            // Get or insert state for this client
            let entry = state_map
                .entry(client_id.to_string())
                .or_insert(TokenBucketState {
                    tokens: self.capacity,
                    last_refill: now,
                });

            // refill tokens
            let elapsed = now.duration_since(entry.last_refill);
            let new_tokens = (elapsed.as_millis() / self.refill_interval.as_millis()) as u32;
            if new_tokens > 0 {
                entry.tokens = (entry.tokens + new_tokens).min(self.capacity);
                entry.last_refill = now;
            }

            if entry.tokens > 0 {
                entry.tokens -= 1;
                true
            } else {
                false
            }
        }
    }

    impl<S, Request> Service<(String, Request)> for MultiRateLimiter<S>
    where
        S: Service<Request> + Send + 'static,
        S::Future: Send + 'static,
        Request: Send + 'static,
    {
        type Response = S::Response;
        type Error = &'static str; // Reject with static str on rate-limit
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx).map_err(|_| "inner not ready")
        }

        fn call(&mut self, (client_id, req): (String, Request)) -> Self::Future {
            if self.try_acquire(&client_id) {
                let fut = self.inner.call(req);
                Box::pin(async move { fut.await.map_err(|_| "inner error") })
            } else {
                Box::pin(async { Err("rate limited") })
            }
        }
    }

    // we use tokio::test with start_paused for testing
    #[tokio::test(start_paused = true)]
    async fn test_multi_rate_limiter() {
        struct Echo;

        impl Service<i32> for Echo {
            type Response = i32;
            type Error = ();
            type Future = Pin<Box<dyn Future<Output = Result<i32, ()>> + Send>>;

            fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }

            fn call(&mut self, req: i32) -> Self::Future {
                Box::pin(async move { Ok(req) })
            }
        }

        let mut svc = MultiRateLimiter::new(Echo, 2, Duration::from_millis(100));

        assert_eq!(
            svc.ready()
                .await
                .unwrap()
                .call(("A".into(), 1))
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            svc.ready()
                .await
                .unwrap()
                .call(("A".into(), 2))
                .await
                .unwrap(),
            2
        );

        assert_eq!(
            svc.ready().await.unwrap().call(("A".into(), 3)).await,
            Err("rate limited")
        );

        assert_eq!(
            svc.ready()
                .await
                .unwrap()
                .call(("B".into(), 10))
                .await
                .unwrap(),
            10
        );

        tokio::time::advance(Duration::from_millis(150)).await;

        assert_eq!(
            svc.ready()
                .await
                .unwrap()
                .call(("A".into(), 4))
                .await
                .unwrap(),
            4
        );
    }
}
