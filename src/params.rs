use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_ssm::Client;

use crate::{Error, Result};

/// Trait for `str` types, enables interaction with [AWS SSM] Parameter Store.
///
/// [AWS SSM]: https://github.com/awslabs/aws-sdk-rust/tree/main/examples/ssm
#[async_trait]
pub trait SSMParamExt {
    /// Retrieves a parameter (with data type `String`) from SSM Parameter Store.
    async fn get_string(self, config: &SdkConfig) -> Result<String>;
    /// Retrieves a parameter (with data type `SecureString`) from SSM Parameter Store.
    async fn get_secure_string(self, config: &SdkConfig) -> Result<String>;
    /// Retrieves a parameter (with data type `StringList`) from SSM Parameter Store.
    async fn get_string_list(self, config: &SdkConfig) -> Result<Vec<String>>;
}

/// Retrieves a parameter from AWS SSM Parameter Store;
/// inspired by the [Create Parameter] example.
///
/// [Create Parameter]: https://github.com/awslabs/aws-sdk-rust/blob/main/examples/ssm/src/bin/create-parameter.rs
async fn get_string_with_decryption<'a>(
    config: &'a SdkConfig,
    param_name: &'a str,
    with_decryption: bool,
) -> Result<String> {
    let client = Client::new(config);

    let resp = client
        .get_parameter()
        .name(param_name)
        .with_decryption(with_decryption)
        .send()
        .await
        .map_err(|e| Error::ReadParam {
            source: e,
            param_name: param_name.to_owned(),
        })?;

    let secret_param = resp.parameter.unwrap();

    Ok(secret_param.value.unwrap())
}

#[async_trait]
impl SSMParamExt for &str {
    async fn get_string(self, config: &SdkConfig) -> Result<String> {
        get_string_with_decryption(config, self, false).await
    }

    async fn get_secure_string(self, config: &SdkConfig) -> Result<String> {
        get_string_with_decryption(config, self, true).await
    }

    async fn get_string_list(self, config: &SdkConfig) -> Result<Vec<String>> {
        let value = get_string_with_decryption(config, self, false).await?;
        Ok(value.split(',').map(str::to_string).collect())
    }
}
