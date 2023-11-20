use std::env;
use std::io::{stdout, Write};

use aws_sdk_secretsmanager::Client;
use eyre::Result;
use futures::future::try_join_all;

const SECRET_SUFFIX: &str = "_SECRET_ARN";

#[tokio::main]
async fn main() -> Result<()> {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;
    let client = Client::new(&config);

    let secret_envs = env::vars()
        .filter(|(key, _)| key.ends_with(SECRET_SUFFIX))
        .collect::<Vec<_>>();

    let secret_arns = secret_envs
        .iter()
        .map(|(_, value)| client.get_secret_value().secret_id(value).send())
        .collect::<Vec<_>>();

    let secret_values = try_join_all(secret_arns).await?;

    let secret_strings = secret_values
        .iter()
        .zip(secret_envs)
        .filter_map(|(secret_value, (key, _))| {
            let secret_string = secret_value.secret_string()?;
            let key = key.replace(SECRET_SUFFIX, "");

            Some(format!(
                "export {}=$'{}'",
                key,
                secret_string.replace('\'', r"\'")
            ))
        })
        .collect::<Vec<_>>()
        .join("\n");

    stdout().write_all(secret_strings.as_bytes())?;

    Ok(())
}
