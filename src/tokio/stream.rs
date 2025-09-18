#[cfg(test)]
mod tests {
    use tokio_stream::{self as stream, StreamExt};

    // streams are for async traversal of an iterable. You can use this on any iterable collection
    // and exhaust the stream in an async fashion. If there are no items available, the thread will
    // yield.
    #[tokio::test(start_paused = true)]
    async fn test_next_items() {
        let mut stream = stream::iter(vec![1, 2, 3]);

        let mut i = 0;

        while let Some(v) = stream.next().await {
            i += 1;
            assert_eq!(v, i);
        }
    }

    use tokio::sync::mpsc;
    use tokio_stream::wrappers::ReceiverStream;

    // you can stream from a channel as well
    #[tokio::test(start_paused = true)]
    async fn test_channel_stream() {
        let (tx, rx) = mpsc::channel(3);
        let mut stream = ReceiverStream::new(rx);

        tx.send(10).await.unwrap();
        tx.send(20).await.unwrap();

        assert_eq!(stream.next().await, Some(10));
        assert_eq!(stream.next().await, Some(20));

        // drop the sender to close the channel
        drop(tx);
        assert_eq!(stream.next().await, None);
    }

    // streams yield if there are no items available
    #[tokio::test(start_paused = true)]
    async fn test_stream_yields() {
        let (tx, rx) = mpsc::channel(1);
        let mut stream = ReceiverStream::new(rx);

        // spawn a task that sends later
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            tx.send(42).await.unwrap();
        });

        // at this point, the stream has no items yet, so `.next().await`
        // will yield, allowing other tasks (the sender above) to run.
        let value = stream.next().await;

        assert_eq!(value, Some(42));
    }
}
