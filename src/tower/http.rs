#[cfg(test)]
mod tests {
    use axum::{Router, routing::get};
    use std::time::Duration;
    use tower::ServiceBuilder;
    use tower::limit::ConcurrencyLimitLayer;
    use tower_http::{compression::CompressionLayer, trace::TraceLayer};

    async fn hello() -> &'static str {
        "hello world"
    }

    pub fn build_app() -> Router {
        Router::new().route("/", get(hello)).layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(ConcurrencyLimitLayer::new(5)),
        )
    }

    use axum::body::{self, Body, Bytes};
    use axum::http::{Request, StatusCode, header};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_hello_world() {
        let app = build_app();

        let response = app
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes: Bytes = body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body_bytes, "hello world");
    }

    #[tokio::test(start_paused = true)]
    async fn test_timeout_layer() {
        use tokio::time::{self, sleep};
        use tower::BoxError;

        async fn slow() -> &'static str {
            sleep(Duration::from_secs(10)).await;
            "too slow"
        }

        let router = axum::Router::new().route("/slow", axum::routing::get(slow));

        let svc = ServiceBuilder::new()
            .layer(axum::error_handling::HandleErrorLayer::new(
                |_err: BoxError| async {
                    axum::http::Response::builder()
                        .status(StatusCode::REQUEST_TIMEOUT)
                        .body(Body::empty())
                        .unwrap()
                },
            ))
            .layer(tower::timeout::TimeoutLayer::new(Duration::from_secs(5)))
            .service(router);

        let fut = svc.oneshot(Request::builder().uri("/slow").body(Body::empty()).unwrap());

        // Jump virtual time forward past the timeout
        time::advance(Duration::from_secs(10)).await;

        let response = fut.await.unwrap();
        assert_eq!(response.status(), StatusCode::REQUEST_TIMEOUT);
    }

    #[tokio::test(start_paused = true)]
    async fn test_concurrency_limit_layer() {
        use axum::Router;
        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use axum::routing::get;
        use tokio::time::{self, Duration, sleep, timeout};
        use tower::limit::ConcurrencyLimitLayer;
        use tower::{ServiceBuilder, ServiceExt};

        // Handler that actually holds the permit for 5s (virtual time).
        async fn hold() -> &'static str {
            sleep(Duration::from_secs(5)).await;
            "done"
        }

        let router = Router::new().route("/hold", get(hold));

        let svc = ServiceBuilder::new()
            .layer(ConcurrencyLimitLayer::new(1)) // only 1 in-flight request permitted
            .service(router);

        // First request: occupies the only permit.
        let mut svc_clone = svc.clone();
        let fut1 = tokio::spawn(async move {
            svc_clone
                .oneshot(Request::builder().uri("/hold").body(Body::empty()).unwrap())
                .await
        });

        // Ensure the first request has started (advance a tiny bit of virtual time).
        time::advance(Duration::from_millis(1)).await;

        // Second request should BLOCK until the first finishes.
        // We wrap it in a short external timeout and assert it times out.
        let second_call = svc
            .clone()
            .oneshot(Request::builder().uri("/hold").body(Body::empty()).unwrap());

        // This should time out because capacity is exhausted and `oneshot` waits for readiness.
        assert!(
            timeout(Duration::from_secs(1), second_call).await.is_err(),
            "second request did not block; concurrency limit not in effect"
        );

        // Let the first request finish (advance 5s of virtual time).
        time::advance(Duration::from_secs(5)).await;

        // First request should be OK.
        let resp1 = fut1.await.unwrap().unwrap();
        assert_eq!(resp1.status(), StatusCode::OK);

        // Now that capacity is free, a new second request should succeed immediately.
        let resp2 = svc
            .oneshot(Request::builder().uri("/hold").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(resp2.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_compression_layer() {
        use axum::response::{IntoResponse, Response};

        async fn compressible() -> Response {
            (
                [(header::CONTENT_TYPE, "text/plain")],
                "this is a long string that should trigger compression under tower-http",
            )
                .into_response()
        }

        let router = axum::Router::new().route("/", axum::routing::get(compressible));

        let svc = ServiceBuilder::new()
            .layer(CompressionLayer::new())
            .service(router);

        let response = svc
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Accept-Encoding", "gzip")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("content-encoding").unwrap(), "gzip");
    }

    #[tokio::test]
    async fn test_trace_layer_runs() {
        let router = axum::Router::new().route("/", axum::routing::get(|| async { "trace ok" }));

        let svc = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .service(router);

        let response = svc
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
