#[cfg(test)]
mod tests {

    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn test_async_read_from_slice() {
        let data = b"hello world";
        let mut reader: &[u8] = &data[..];

        let mut buf = [0u8; 5];
        let n = reader.read(&mut buf).await.unwrap();

        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");
    }

    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_async_write_to_vec() {
        let mut buf = Vec::new();
        let n = buf.write(b"tokio").await.unwrap();

        assert_eq!(n, 5);
        assert_eq!(&buf, b"tokio");
    }

    use tokio::io::{self};

    #[tokio::test]
    async fn test_async_duplex_round_trip() {
        let (mut client, mut server) = io::duplex(64);

        // Spawn a task to write to "server"
        let writer = tokio::spawn(async move {
            server.write_all(b"ping").await.unwrap();
            server
        });

        // Read from "client"
        let mut buf = [0u8; 4];
        client.read_exact(&mut buf).await.unwrap();

        assert_eq!(&buf, b"ping");

        // Ensure writer task finishes cleanly
        let _ = writer.await.unwrap();
    }
}
