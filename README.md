# aws-secrets

[<img alt="github" src="https://img.shields.io/badge/github-rnag/aws--secrets-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/rnag/aws-secrets)
[<img alt="crates.io" src="https://img.shields.io/crates/v/aws-secrets.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/aws-secrets)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/aws-secrets/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/aws-secrets)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/rnag/aws-secrets/build/main?style=for-the-badge" height="22">](https://github.com/rnag/aws-secrets/actions?query=branch%3Amain)

Retrieve AWS secrets and interact with [Secrets Manager] and [SSM Parameter Store].

[Secrets Manager]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html
[SSM Parameter Store]: https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html

---

This crate works with Cargo with a `Cargo.toml` like:

<!-- Note: the `tokio` dependency can be omitted if this crate doesn't
require any `async` features. -->
```toml
[dependencies]
aws-secrets = { version = "0.1.0", features = ["all"] }
serde_json = "1"  # optional
tokio = { version = "1", features = ["full"] }
```

## Getting started

Add some usage to your application. Here's an example of using `aws-secrets` in code.

> Note: this sample requires the `all` feature to be enabled.

```rust
use aws_secrets::{config_from_env, SSMParamExt, SecretsExt};
use serde_json::{to_string, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_config = config_from_env().await;

    // Retrieve a secret from AWS Secrets Manager
    let secret_name = "my-secret";
    let value: Value = secret_name.get_secret(&shared_config).await?;
    let secret_string = to_string(&value)?;
    println!("[{secret_name}] Retrieved secret. value={secret_string}");

    // Retrieve a parameter from AWS SSM Parameter Store
    let param_name = "/my/secure/param";
    let value = param_name.get_secure_string(&shared_config).await?;
    println!("[{param_name}] Retrieved parameter. value={value:?}");

    Ok(())
}
```

## Examples

You can check out sample usage of this crate in the [examples/](https://github.com/rnag/aws-secrets/tree/main/examples)
folder in the project repo on GitHub.

## Dependencies and Features

This library uses only the minimum required dependencies, in order
to keep the overall size small. It leverages the [AWS SDK for Rust]
for making calls to AWS APIs.

> **Note:** Any desired features must be enabled individually, as no features are enabled by default.

#### Available features

* `all` - Enables support for AWS Secrets Manager and SSM Parameter Store.
* `params` - Enables support for AWS SSM Parameter Store.
* `sm` - Enables support for AWS Secrets Manager.

#### Enabling Features

Update the project's `Cargo.toml` to include any optional features to enable:
```toml
[dependencies]
aws-secrets = { version = "*", features = ["all"] }
```

[AWS SDK for Rust]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/using.html

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[Contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/rnag/aws-secrets/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`aws-secrets` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

* [Ritvik Nag](https://github.com/rnag)
