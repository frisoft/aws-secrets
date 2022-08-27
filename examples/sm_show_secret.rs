#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate tracing;

mod utils;
use utils::*;

use aws_sdk_secretsmanager::error::GetSecretValueErrorKind::*;
use aws_sdk_secretsmanager::types::SdkError;
use serde::Deserialize;
use structopt::StructOpt;

use aws_secrets::{config_from_env, Error, SecretsExt};

/// Dummy credentials.
/// # Note
/// The secret requires the following contents at a minimum:
///   {"Username":"my-user", "Password":"my-pass12345"}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DummyCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieve the value of a Secret from AWS Secrets Manager.")]
struct Opt {
    /// Name of the Secret to retrieve the value of.
    ///
    /// # Note
    /// The secret requires the following contents at a minimum:
    ///   {"Username":"my-user", "Password":"my-pass12345"}
    #[structopt(short, long, default_value = "my-dummy-creds")]
    secret_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_short!();

    let Opt { secret_name } = Opt::from_args();
    // technically not needed
    let secret_name = secret_name.as_str();

    trace!(profile = ?aws_profile(), "retrieving AWS config.");
    let config = config_from_env().await;

    trace!(secret_name, "retrieving secret.");

    // normally, we'd just call it like: `secret_name.get_secret(&config).await?`
    let value: DummyCredentials = secret_name.get_secret(&config).await.map_err(|e| {
        if let Error::ReadSecret { ref source, .. } = e {
            if let SdkError::ServiceError { err, .. } = source {
                let message = err.message().unwrap_or("<empty>");
                match err.kind {
                    DecryptionFailure(_) => {
                        error!(?secret_name, ?message, "Decryption failure.");
                        return e;
                    }
                    InternalServiceError(_) => {
                        error!(?secret_name, ?message, "Internal service error.");
                        return e;
                    }
                    InvalidParameterException(_) => {
                        error!(?secret_name, ?message, "Invalid parameter.");
                        return e;
                    }
                    InvalidRequestException(_) => {
                        error!(?secret_name, ?message, "Invalid request.");
                        return e;
                    }
                    ResourceNotFoundException(_) => {
                        error!(?secret_name, ?message, "Resource not found.");
                        return e;
                    }
                    Unhandled(_) => {
                        error!(?secret_name, ?message, "Unhandled error.");
                        return e;
                    }
                    _ => {}
                };
            }
        };
        error!(?secret_name, error = ?e, "Unexpected error.");
        return e;
    })?;

    trace!(?value, "successfully retrieved the secret.");

    Ok(())
}
