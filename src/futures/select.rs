#[cfg(test)]
mod tests {
    use futures::future::select;
    use futures::{FutureExt, future::Either};
    use tokio::time::{Duration, sleep};

    // futures provides select, same as tokio does. This chooses the first future to return.
    #[tokio::test]
    async fn test_futures_select() {
        let fast = async {
            sleep(Duration::from_millis(10)).await;
            "fast"
        }
        .fuse();
        let slow = async {
            sleep(Duration::from_millis(50)).await;
            "slow"
        }
        .fuse();

        // make sure to pin them before selecting
        tokio::pin!(fast);
        tokio::pin!(slow);

        let result = select(fast, slow).await;

        // The first completed future wins
        match result {
            Either::Left((val, _)) => assert_eq!(val, "fast"),
            Either::Right((val, _)) => assert_eq!(val, "slow"),
        }
    }

    use futures::{StreamExt, stream};

    // futures also has a stream api like tokio-stream
    #[tokio::test]
    async fn test_futures_stream() {
        let numbers = stream::iter(vec![1, 2, 3, 4]);
        let doubled: Vec<_> = numbers.map(|n| n * 2).collect().await;

        assert_eq!(doubled, vec![2, 4, 6, 8]);
    }

    use futures::try_join;

    async fn foo() -> Result<i32, &'static str> {
        Ok(2)
    }

    async fn bar() -> Result<i32, &'static str> {
        Err("oops")
    }

    // futures also has try join which  fails fast if any future returns an error
    #[tokio::test]
    async fn test_futures_try_join() {
        let result = try_join!(foo(), bar());
        assert_eq!(result, Err("oops"));
    }
}
