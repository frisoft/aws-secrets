//! Library-specific type definitions

/// A `Result` alias where the `Err` case is `aws_secrets::Error`.
pub type Result<T> = std::result::Result<T, crate::Error>;
