#[cfg(test)]
mod tests {
    use std::io::{self, ErrorKind};

    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DataStoreError {
        #[error("data store disconnected")]
        Disconnect(#[from] io::Error),
        #[error("the data for key `{0}` is not available")]
        Redaction(String),
        #[error("invalid header (expected {expected:?}, found {found:?})")]
        InvalidHeader { expected: String, found: String },
        #[error("unknown data store error")]
        Unknown,
    }

    #[test]
    fn turn_io_error_into_disconnect() {
        assert_eq!(
            DataStoreError::from(io::Error::new(ErrorKind::NotFound, "oh no")).to_string(),
            "data store disconnected"
        );
    }
}
