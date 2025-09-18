pub mod httpmock;
pub mod tokio;

#[cfg(test)]
mod tests {
    use predicates::prelude::*;

    #[test]
    fn basic() {
        let less_than_ten = predicate::lt(10);
        assert!(less_than_ten.eval(&9));
        assert!(!less_than_ten.eval(&11));
    }

    #[sqlx::test]
    async fn test_async_fn() {
        tokio::task::yield_now().await;
    }
}
