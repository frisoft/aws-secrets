use thiserror::Error;

#[cfg(feature = "sm")]
mod sm_imports {
    pub(crate) use aws_sdk_secretsmanager::error::{GetSecretValueError, TagResourceError};
    pub(crate) use aws_sdk_secretsmanager::types::SdkError as SMError;
}
#[cfg(feature = "sm")]
use sm_imports::*;

#[cfg(feature = "params")]
mod params_imports {
    pub(crate) use aws_sdk_ssm::error::GetParameterError;
    pub(crate) use aws_sdk_ssm::types::SdkError as ParamsError;
}
#[cfg(feature = "params")]
use params_imports::*;

/// Library-specific errors
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Raised when an error occurs in the `secretsmanager:TagResource` operation
    #[cfg(feature = "sm")]
    #[error("couldn't set tag")]
    SetTag(#[from] SMError<TagResourceError>),
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
        source: ParamsError<GetParameterError>,
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
