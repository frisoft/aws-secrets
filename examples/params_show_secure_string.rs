#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate tracing;

use aws_sdk_ssm::error::GetParameterErrorKind::*;
use aws_sdk_ssm::types::SdkError;
use structopt::StructOpt;

use aws_secrets::{Error, SSMParamExt};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Retrieve the value of a `SecureString` Parameter from AWS SSM Parameter Store."
)]
struct Opt {
    /// Name of the Parameter to retrieve the value of.
    #[structopt(short, long, default_value = "/my/dummy/secure/param")]
    param_name: String,
}

// noinspection DuplicatedCode
#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init!();

    let Opt { param_name } = Opt::from_args();
    // technically not needed
    let param_name = param_name.as_str();

    let config = aws_config::load_from_env().await;

    trace!(
        data_type = "SecureString",
        param_name,
        "retrieving parameter."
    );

    // normally, we'd just call it like: `param_name.get_secure_string(&config).await?`
    let value = param_name.get_secure_string(&config).await.map_err(|e| {
        if let Error::ReadParam { ref source, .. } = e {
            if let SdkError::ServiceError { err, .. } = source {
                let message = err.message().unwrap_or("<empty>");
                match err.kind {
                    InvalidKeyId(_) => {
                        error!(?param_name, ?message, "Invalid key ID.");
                        return e;
                    }
                    InternalServerError(_) => {
                        error!(?param_name, ?message, "Internal server error.");
                        return e;
                    }
                    ParameterNotFound(_) => {
                        error!(?param_name, ?message, "Parameter not found.");
                        return e;
                    }
                    ParameterVersionNotFound(_) => {
                        error!(?param_name, ?message, "Parameter version not found.");
                        return e;
                    }
                    Unhandled(_) => {
                        error!(?param_name, ?message, "Unhandled error.");
                        return e;
                    }
                    _ => {}
                };
            }
        };
        error!(?param_name, error = ?e, "Unexpected error.");
        return e;
    })?;

    trace!(?value, "successfully retrieved the parameter.");

    Ok(())
}
