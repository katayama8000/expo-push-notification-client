use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, thiserror::Error)]
pub enum CustomError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Deserialize error: {0}")]
    DeserializeErr(String),
    #[error("Server error: {0}")]
    ServerErr(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_error() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<CustomError>();
    }

    #[test]
    fn test_impl_display() {
        assert_eq!(
            CustomError::InvalidArgument("x".to_string()).to_string(),
            "Invalid argument: x"
        );
        assert_eq!(
            CustomError::DeserializeErr("x".to_string()).to_string(),
            "Deserialize error: x"
        );
        assert_eq!(
            CustomError::ServerErr("x".to_string()).to_string(),
            "Server error: x"
        );
    }
}
