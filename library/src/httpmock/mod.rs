#[cfg(test)]
mod tests {
    use std::io::Read;

    use httpmock::prelude::*;

    // httpmock is for running a mock http server where you can control its behavior.
    #[test]
    fn httpmocking() {
        let server = MockServer::start();

        // This crates a server that when a GET request to /translate?word=hello is made,
        // then it returns with a 200, with content-type text/html and responds with 'ohi'
        let hello_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/translate")
                .query_param("word", "hello");
            then.status(200)
                .header("content-type", "text/html")
                .body("ohi");
        });

        // isahc is an http client, so we can use it for this server.
        let mut response = isahc::get(server.url("/translate?word=hello")).unwrap();

        // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
        hello_mock.assert();
        // Ensure the mock server did respond as specified.
        assert_eq!(response.status(), 200);

        // make sure the body is "ohi"
        let mut buf = String::new();
        response.body_mut().read_to_string(&mut buf).unwrap();
        assert_eq!(buf, String::from("ohi"));
    }
}
