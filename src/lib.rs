pub mod anyhow;
pub mod async_trait;
pub mod axum;
pub mod delegate;
pub mod derive_more;
pub mod facet;
pub mod faux;
pub mod futures;
pub mod httpmock;
pub mod thiserror;
pub mod tokio;
pub mod tower;
pub mod ux;

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
