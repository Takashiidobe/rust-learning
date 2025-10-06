#[cfg(test)]
mod tests {

    use axum::{Router, routing::get};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`

    fn app() -> Router {
        Router::new().route("/", get(|| async { "Hello, world!" }))
    }

    #[tokio::test]
    async fn test_hello_world() {
        let app = app();

        let req = Request::builder().uri("/").body(Body::empty()).unwrap();

        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(res.into_body(), "Hello, world!".len())
            .await
            .unwrap();
        assert_eq!(body_bytes, "Hello, world!");
    }
}
