#![doc(html_root_url = "https://docs.rs/aws-secrets/0.1.0")]
#![warn(rust_2018_idioms, missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]

//! [![github]](https://github.com/rnag/aws-secrets)&ensp;[![crates-io]](https://crates.io/crates/aws-secrets)&ensp;[![docs-rs]](https://docs.rs/aws-secrets)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Retrieve AWS secrets and interact with [Secrets Manager] and [SSM Parameter Store].
//!
//! [Secrets Manager]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html
//! [SSM Parameter Store]: https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html
//!
//! <br>
//!
//! ## Usage
//!
//! > Note: this example requires the `all` feature to be enabled.
//!
//! ```no_run
//! use aws_secrets::{config_from_env, SSMParamExt, SecretsExt};
//! use serde_json::{to_string, Value};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let shared_config = config_from_env().await;
//!
//!     // Retrieve a secret from AWS Secrets Manager
//!     let secret_name = "my-secret";
//!     let value: Value = secret_name.get_secret(&shared_config).await?;
//!     let secret_string = to_string(&value)?;
//!     println!("[{secret_name}] Retrieved secret. value={secret_string}");
//!
//!     // Retrieve a parameter from AWS SSM Parameter Store
//!     let param_name = "/my/secure/param";
//!     let value = param_name.get_secure_string(&shared_config).await?;
//!     println!("[{param_name}] Retrieved parameter. value={value:?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! You can check out sample usage of this crate in the [examples/](https://github.com/rnag/aws-secrets/tree/main/examples)
//! folder in the project repo on GitHub.
//!
//! ## Readme Docs
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! ## Dependencies and Features
//!
//! This library uses only the minimum required dependencies, in order
//! to keep the overall size small. It leverages the [AWS SDK for Rust]
//! for making calls to AWS APIs.
//!
//! > **Note:** Any desired features must be enabled individually, as no features are enabled by default.
//!
//! #### Available features
//!
//! * `all` - Enables support for AWS Secrets Manager and SSM Parameter Store.
//! * `params` - Enables support for AWS SSM Parameter Store.
//! * `sm` - Enables support for AWS Secrets Manager.
//!
//! #### Enabling Features
//!
//! Update the project's `Cargo.toml` to include any optional features to enable:
//! ```toml
//! [dependencies]
//! aws-secrets = { version = "*", features = ["all"] }
//! ```
//!
//! [AWS SDK for Rust]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/using.html
//!
//! [crates.io]: https://crates.io/crates/aws-secrets
//! [`README.md`]: https://github.com/rnag/aws-secrets
//!

mod errors;
#[cfg(feature = "params")]
mod params;
#[cfg(feature = "sm")]
mod secretsmanager;
mod types;

pub use aws_config as config;
pub use aws_config::load_from_env as config_from_env;
pub use errors::Error;
#[cfg(feature = "params")]
pub use params::SSMParamExt;
#[cfg(feature = "sm")]
pub use secretsmanager::SecretsExt;
pub use types::Result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
