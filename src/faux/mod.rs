#[cfg(test)]
mod tests {
    // faux creates stand-in versions of structs so tests can control their behavior.

    #[cfg_attr(test, faux::create)]
    #[derive(Debug)]
    struct MathClient;

    #[cfg_attr(test, faux::methods)]
    impl MathClient {
        fn new() -> Self {
            Self
        }

        pub fn double(&self, value: i32) -> i32 {
            value * 2
        }
    }

    struct Calculator {
        client: MathClient,
    }

    impl Calculator {
        fn new(client: MathClient) -> Self {
            Self { client }
        }

        fn compute(&self, value: i32) -> i32 {
            self.client.double(value)
        }
    }

    #[test]
    fn doubles_with_real_client() {
        let calculator = Calculator::new(MathClient::new());
        assert_eq!(calculator.compute(4), 8);
    }

    #[test]
    fn overrides_behavior_with_faux() {
        let mut mock = MathClient::faux();
        faux::when!(mock.double(_)).then(|value| value + 10);

        let calculator = Calculator::new(mock);
        assert_eq!(calculator.compute(4), 14);
    }
}
