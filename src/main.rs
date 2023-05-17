use std::env;
use std::io::{self, Write};

use aws_sdk_secretsmanager::Client;
use eyre::Result;
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> Result<()> {
    let config = aws_config::from_env().load().await;
    let client = Client::new(&config);

    let secret_envs = env::vars()
        .into_iter()
        .filter(|(key, _)| key.ends_with("_SECRET_ARN"))
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

            Some(format!("{}='{}'", key, secret_string))
        })
        .collect::<Vec<_>>()
        .join("\n");

    io::stdout().write_all(secret_strings.as_bytes())?;

    Ok(())
}
