#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate tracing;

use aws_sdk_secretsmanager::error::TagResourceErrorKind::*;
use aws_sdk_secretsmanager::types::SdkError;
use structopt::StructOpt;

use aws_secrets::{Error, SecretsExt};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(about = "Set or update the tags of a Secret in AWS Secrets Manager.")]
struct Opt {
    /// Name of the Secret to update the tag of.
    #[structopt(short, long, default_value = "my-dummy-creds")]
    secret_name: String,

    /// Tag key to update the *value* of.
    #[structopt(short, long, default_value = "my-sample-tag")]
    key: String,

    /// Value to set on the tag *key*.
    #[structopt(short, long, default_value = "some dummy value.")]
    value: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let Opt {
        secret_name,
        key,
        value,
    } = Opt::from_args();
    // technically not needed
    let secret_name = secret_name.as_str();

    let config = aws_config::load_from_env().await;

    trace!(?secret_name, ?key, ?value, "updating tag on secret.");

    // normally, we'd just call it like: `secret_name.set_tag(&config, &key, &value).await?`
    let _ = secret_name
        .set_tag(&config, &key, &value)
        .await
        .map_err(|e| {
            if let Error::SetTags(ref source) = e {
                if let SdkError::ServiceError { err, .. } = source {
                    let message = err.message().unwrap_or("<empty>");
                    match err.kind {
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

    trace!(?secret_name, "successfully updated the tag.");

    Ok(())
}
