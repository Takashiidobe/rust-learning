#[cfg(test)]
mod tests {
    // The http crate has data types for requests/responses from http. These are used by
    // higher-level crates like reqwest and axum.
    use http::{
        HeaderMap, HeaderValue, Request, Response, StatusCode,
        header::{LOCATION, USER_AGENT},
    };

    #[test]
    fn build_request() {
        let request = Request::builder()
            .uri("http://takashiidobe.com")
            .header(USER_AGENT, "chrome/whatever")
            .body(())
            .unwrap();

        assert_eq!(request.uri(), "http://takashiidobe.com");
    }

    #[test]
    fn build_response() {
        let request = Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header(LOCATION, "https://takashiidobe.com")
            .body(())
            .unwrap();

        assert_eq!(request.status(), StatusCode::FORBIDDEN);
        let headers = request.headers().to_owned();
        assert_eq!(
            headers,
            HeaderMap::from_iter([(
                LOCATION,
                HeaderValue::from_static("https://takashiidobe.com")
            )])
        );
    }
}
