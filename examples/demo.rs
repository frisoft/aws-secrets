#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate tracing;

use serde_json::Value;
use structopt::StructOpt;

use aws_secrets::{SSMParamExt, SecretsExt};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(about = "Retrieve values from AWS Secrets Manager and AWS SSM Parameter Store.")]
struct Opt {
    /// Name of the Secret to retrieve the value of.
    #[structopt(short, long, default_value = "my-dummy-creds")]
    secret_name: String,
    /// Name of the Parameter to retrieve the value of.
    #[structopt(short, long, default_value = "/my/dummy/secure/param")]
    param_name: String,
    /// True to retrieve SSM Parameter without decryption (i.e. if the data type is `String`)
    #[structopt(short, long)]
    no_decrypt: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    sensible_env_logger::init_timed_local!();

    let Opt {
        secret_name,
        param_name,
        no_decrypt,
    } = Opt::from_args();

    // technically not needed
    let secret_name = secret_name.as_str();
    let param_name = param_name.as_str();

    let config = aws_config::load_from_env().await;

    trace!(secret_name, "retrieving secret.");
    trace!(param_name, "retrieving parameter.");

    let get_param_fn = match no_decrypt {
        true => param_name.get_string(&config),
        false => param_name.get_secure_string(&config),
    };

    // retrieve all secret values concurrently
    let (secret_value, param_value) =
        tokio::join!(secret_name.get_secret::<Value>(&config), get_param_fn);

    let value = secret_value?;
    trace!(?value, "successfully retrieved the secret.");

    let value = param_value?;
    trace!(?value, "successfully retrieved the parameter.");

    Ok(())
}
