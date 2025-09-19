#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    #[async_trait]
    trait Greeter {
        async fn greet(&self) -> String;
    }

    struct EnglishGreeter;
    struct SpanishGreeter;

    #[async_trait]
    impl Greeter for EnglishGreeter {
        async fn greet(&self) -> String {
            "Hello".into()
        }
    }

    // async trait allows you to use async fn in traits.
    #[async_trait]
    impl Greeter for SpanishGreeter {
        async fn greet(&self) -> String {
            "Hola".into()
        }
    }

    #[tokio::test]
    async fn test_async_trait_multiple_impls() {
        let eng = EnglishGreeter;
        let esp = SpanishGreeter;

        assert_eq!(eng.greet().await, "Hello");
        assert_eq!(esp.greet().await, "Hola");
    }

    use std::future::Future;
    use std::pin::Pin;

    // you can implement async traits manually, but the return type is quite verbose
    trait Fetcher {
        fn fetch(&self, input: i32) -> Pin<Box<dyn Future<Output = i32> + Send>>;
    }

    struct DoubleFetcher;

    impl Fetcher for DoubleFetcher {
        fn fetch(&self, input: i32) -> Pin<Box<dyn Future<Output = i32> + Send>> {
            Box::pin(async move { input * 2 })
        }
    }

    #[tokio::test]
    async fn test_manual_async_trait() {
        let f = DoubleFetcher;
        let result = f.fetch(21).await;
        assert_eq!(result, 42);
    }

    // This requires nightly but you can use GATs
    // trait GatFetcher {
    //     type Fut<'a>: Future<Output = i32> + 'a;

    //     fn fetch<'a>(&'a self, input: i32) -> Self::Fut<'a>;
    // }

    // struct GatDoubleFetcher;

    // impl GatFetcher for GatDoubleFetcher {
    //     type Fut<'a> = impl Future<Output = i32> + 'a;

    //     fn fetch<'a>(&'a self, input: i32) -> Self::Fut<'a> {
    //         async move { input * 2 }
    //     }
    // }

    // #[tokio::test]
    // async fn test_gat_async_trait() {
    //     let f = DoubleFetcher;
    //     let result = f.fetch(21).await;
    //     assert_eq!(result, 42);
    // }
}
