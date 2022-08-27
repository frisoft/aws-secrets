#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate tracing;

mod utils;
use utils::*;

use serde_json::Value;
use structopt::StructOpt;

use aws_secrets::{config_from_env, SSMParamExt, SecretsExt};

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

    trace!(profile = ?aws_profile(), "retrieving AWS config.");
    let shared_config = config_from_env().await;

    trace!(secret_name, "retrieving secret.");
    trace!(param_name, "retrieving parameter.");

    let get_param_fn = match no_decrypt {
        true => param_name.get_string(&shared_config),
        false => param_name.get_secure_string(&shared_config),
    };

    // retrieve all secret values concurrently
    let (param_value, secret_value) = tokio::try_join!(
        get_param_fn,
        secret_name.get_secret::<Value>(&shared_config),
    )?;

    trace!(?secret_value, "successfully retrieved the secret.");
    trace!(?param_value, "successfully retrieved the parameter.");

    Ok(())
}
