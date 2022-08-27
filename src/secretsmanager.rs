use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_secretsmanager::model::Tag;
use aws_sdk_secretsmanager::output::TagResourceOutput;
use aws_sdk_secretsmanager::Client;
use serde::de::DeserializeOwned;

use crate::{Error, Result};

/// Trait for `str` types, enables interaction with [AWS Secrets Manager].
///
/// [AWS Secrets Manager]: https://github.com/awslabs/aws-sdk-rust/tree/main/examples/secretsmanager
#[async_trait]
pub trait SecretsExt {
    /// Retrieves and de-serializes a secret from AWS Secrets Manager;
    /// taken from the [Get Secret Value] example.
    ///
    /// [Get Secret Value]: https://github.com/awslabs/aws-sdk-rust/blob/main/examples/secretsmanager/src/bin/get-secret-value.rs
    async fn get_secret<T: DeserializeOwned>(self, config: &SdkConfig) -> Result<T>;
    /// Set or update the value of a **tag** on a secret that lives in
    /// AWS Secrets Manager.
    async fn set_tag(self, config: &SdkConfig, key: &str, value: &str)
        -> Result<TagResourceOutput>;
}

#[async_trait]
impl SecretsExt for &str {
    async fn get_secret<T: DeserializeOwned>(self, config: &SdkConfig) -> Result<T> {
        let client = Client::new(config);

        let resp = client
            .get_secret_value()
            .secret_id(self)
            .send()
            .await
            .map_err(|e| Error::ReadSecret {
                secret_name: self.to_owned(),
                source: e,
            })?;

        let secret_str = resp.secret_string().unwrap();

        serde_json::from_str(secret_str).map_err(Error::DeserializeError)
    }

    async fn set_tag(
        self,
        config: &SdkConfig,
        key: &str,
        value: &str,
    ) -> Result<TagResourceOutput> {
        let client = Client::new(config);

        let tag = Tag::builder().key(key).value(value).build();

        client
            .tag_resource()
            .secret_id(self)
            .tags(tag)
            .send()
            .await
            .map_err(Error::SetTag)
    }
}
