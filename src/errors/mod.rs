use std::fmt;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CustomErrorKind {
    NotFound,
    PermissionDenied,
    InvalidArgument,
    InvalidData,
    UnexpectedEof,
    ResourceBusy,
    TimedOut,
    OutOfMemory,
    Other,
}

#[derive(Debug, Error)]
pub struct CustomError {
    kind: CustomErrorKind,
    source: Option<anyhow::Error>,
    message: String,
}

impl CustomError {
    pub fn new(kind: CustomErrorKind, message: impl Into<String>) -> Self {
        CustomError {
            kind,
            source: None,
            message: message.into(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)?;
        if let Some(source) = &self.source {
            write!(f, " (source: {})", source)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use pretty_assertions::assert_eq;

    fn simulate_operation(
        should_fail: bool,
        error_type: CustomErrorKind,
    ) -> Result<(), CustomError> {
        if should_fail {
            match error_type {
                CustomErrorKind::NotFound => Err(CustomError::new(
                    CustomErrorKind::NotFound,
                    "The requested item was not found.",
                )),
                CustomErrorKind::PermissionDenied => Err(CustomError::new(
                    CustomErrorKind::PermissionDenied,
                    "Access denied to the resource.",
                )),
                _ => Err(CustomError::new(error_type, "Unknown Error")),
            }
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_errors() {
        if let Err(e) = simulate_operation(true, CustomErrorKind::NotFound) {
            assert_eq!(e.kind, CustomErrorKind::NotFound);
            assert_eq!(e.to_string(), "NotFound: The requested item was not found.");
        }

        if let Err(e) = simulate_operation(true, CustomErrorKind::InvalidArgument) {
            assert_eq!(e.kind, CustomErrorKind::InvalidArgument);
            assert_eq!(e.to_string(), "InvalidArgument: Unknown Error",);
        }

        assert!(simulate_operation(false, CustomErrorKind::Other).is_ok());
    }

    // we can take a Box of dyn Error and turn it back into the concrete type by downcasting
    #[test]
    fn downcasting() {
        let error = CustomError::new(CustomErrorKind::Other, "some unknown message");
        let error_kind = error.kind;
        let boxed_error: Box<dyn Error> = Box::new(error);
        assert_eq!(
            error_kind,
            boxed_error.downcast::<CustomError>().unwrap().kind
        );
    }
}
