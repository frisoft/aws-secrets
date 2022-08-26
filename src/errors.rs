#[cfg(feature = "sm")]
use aws_sdk_secretsmanager::error::{GetSecretValueError, TagResourceError};
#[cfg(feature = "sm")]
use aws_sdk_secretsmanager::types::SdkError as SMError;
#[cfg(feature = "params")]
use aws_sdk_ssm::error::GetParameterError;
#[cfg(feature = "params")]
use aws_sdk_ssm::types::SdkError as SSMError;
use thiserror::Error;

/// Library-specific errors
#[derive(Error, Debug)]
pub enum Error {
    /// Raised when an error occurs in the `secretsmanager:SetTags` operation
    #[cfg(feature = "sm")]
    #[error("couldn't set tags")]
    SetTags(#[from] SMError<TagResourceError>),
    /// Indicates a `serde` error when de-serializing a JSON string.
    #[cfg(feature = "sm")]
    #[error("couldn't deserialize secret string")]
    DeserializeError(#[from] serde_json::Error),
    /// Raised when an error occurs in the `ssm:GetParameter` operation
    #[cfg(feature = "params")]
    #[error("[{param_name:?}] couldn't read param")]
    ReadParam {
        /// Name of the Parameter to retrieve
        param_name: String,
        /// Original error
        source: SSMError<GetParameterError>,
    },
    /// Raised when an error occurs in the `secretsmanager:GetSecretValue` operation
    #[cfg(feature = "sm")]
    #[error("[{secret_name:?}] couldn't read secret")]
    ReadSecret {
        /// Name of the Secret to retrieve
        secret_name: String,
        /// Original error
        source: SMError<GetSecretValueError>,
    },
    /// Unknown library error (currently unused)
    #[error("unknown error")]
    Unknown,
}
