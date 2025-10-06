#[cfg(test)]
mod tests {
    use std::io::{self, ErrorKind};

    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DataStoreError {
        #[error("data store disconnected")]
        Disconnect(#[from] io::Error),
    }

    #[test]
    fn turn_io_error_into_disconnect() {
        assert_eq!(
            DataStoreError::from(io::Error::new(ErrorKind::NotFound, "oh no")).to_string(),
            "data store disconnected"
        );
    }
}
